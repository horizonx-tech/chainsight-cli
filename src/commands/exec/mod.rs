use std::{collections::BTreeMap, path::Path};

use anyhow::{bail, Context};
use candid::Principal;
use clap::{arg, Parser};
use functions::{call_init_in, call_set_task, call_setup};
use slog::{info, Logger};

use crate::{
    commands::utils::get_agent,
    lib::{
        codegen::{
            components::{codegen, common::ComponentTypeInManifest},
            project::ProjectManifestData,
        },
        environment::EnvironmentImpl,
        utils::{
            component_ids_manager::ComponentIdsManager,
            dfx::DfxWrapperNetwork,
            env::cache_envfile,
            identity::{
                get_wallet_principal_from_local_context, identity_from_keyring, wallet_canister,
            },
            is_chainsight_project, ARTIFACTS_DIR, DOTENV_FILENAME, PROJECT_MANIFEST_FILENAME,
        },
    },
    types::{ComponentType, Network},
};

mod functions;

#[derive(Debug, Parser)]
#[command(name = "exec")]
/// Calls for component processing. Currently supports initialization and task start instructions.
pub struct ExecOpts {
    /// Specify the path of the project that manages the component to be called.
    /// Refer to the manifest of this project to build the commands that should be executed.
    #[arg(long, short = 'p')]
    path: Option<String>,

    /// Specify the name of the component you want to execute.
    /// If this option is not specified, the command will be given to all components managed by the project.
    #[arg(long, short = 'c')]
    component: Option<String>,

    /// Specify the context of identity to execute on.
    /// If this option is specfied & no string, the default context is used.
    #[arg(long)]
    context: Option<String>,

    /// Specify the wallet to use.
    /// If this option is not specified, the default wallet is used.
    #[arg(long, short = 'w')]
    wallet: Option<String>,

    /// Specify the network to execute on.
    #[arg(long)]
    #[clap(default_value = "local")]
    network: Network,

    /// Specifies the port to call.
    /// This option is used only if the target is localhost.
    #[arg(long)]
    port: Option<u16>,
}

pub async fn exec(env: &EnvironmentImpl, opts: ExecOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        bail!(format!(r#"{}"#, msg));
    }

    info!(log, r#"Execute canister processing..."#);

    let project_path_str = project_path.unwrap_or(".".to_string());

    // load env
    let env_file_path = format!("{}/{}", &project_path_str, DOTENV_FILENAME);
    if Path::new(&env_file_path).is_file() {
        info!(log, r#"Load env file: "{}""#, &env_file_path);
        cache_envfile(Some(&env_file_path))?;
    }

    execute_initialize_components(
        log,
        &project_path_str,
        opts.component,
        opts.context,
        opts.wallet,
        opts.network,
        opts.port,
    )
    .await?;

    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;
    info!(
        log,
        r#"Project "{}" canisters executed successfully"#, project_manifest.label
    );

    Ok(())
}

async fn execute_initialize_components(
    log: &Logger,
    project_path_str: &str,
    component_name: Option<String>,
    identity_context: Option<String>,
    wallet: Option<String>,
    network: Network,
    port: Option<u16>,
) -> anyhow::Result<()> {
    // loading component ids
    let dfx_bin_network = match &network {
        Network::Local => DfxWrapperNetwork::Local(port),
        Network::IC => DfxWrapperNetwork::IC,
    };
    let artifacts_path = format!("{}/{}", &project_path_str, ARTIFACTS_DIR);
    let comp_id_mgr = ComponentIdsManager::load(&dfx_bin_network, &artifacts_path)?;
    let components = if let Some(name) = component_name {
        let comp_id = comp_id_mgr
            .get(&name)
            .context(format!("Component not found: {}", name))?;
        vec![(name, comp_id)]
    } else {
        comp_id_mgr.get_all_entries()
    };

    // generate wallet canister
    let caller_identity = identity_from_keyring(identity_context.clone())?;
    let agent = get_agent(&network, port, Some(Box::new(caller_identity))).await?;
    let wallet_canister_id = match wallet {
        Some(canister_id) => Principal::from_text(canister_id).map_err(|e| anyhow::anyhow!(e))?,
        None => get_wallet_principal_from_local_context(&network, port, identity_context).await?,
    };
    let wallet = wallet_canister(wallet_canister_id, &agent).await?;

    // exec: init_in
    for (name, comp_id) in &components {
        info!(log, "Calling init_in: {} ({})", name, comp_id);
        call_init_in(&wallet, Principal::from_text(comp_id)?, &network).await?;
        info!(log, "Called init_in: {} ({})", name, comp_id);
    }

    // exec: setup
    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;
    let component_path_mapping: BTreeMap<String, (ComponentType, String)> = project_manifest
        .components
        .iter()
        .map(|c| {
            let c_path = format!("{}/{}", &project_path_str, c.component_path);
            let c_type = ComponentTypeInManifest::determine_type(&c_path)
                .unwrap_or_else(|_| panic!("Failed to determine component type: {}", &c_path));
            let id = Path::new(&c_path).file_stem().unwrap().to_str().unwrap();
            (id.to_owned(), (c_type, c_path))
        })
        .collect();
    for (name, comp_id) in &components {
        let (component_type, component_path) = component_path_mapping
            .get(name.as_str())
            .context(format!("Component not found: {}", &name))?;
        let generator = codegen::generator(*component_type, component_path, name)?;

        if let Some(raw_args) = generator.generate_component_setup_args(&network, &comp_id_mgr)? {
            info!(log, "Calling setup: {} ({})", name, comp_id);
            call_setup(&wallet, Principal::from_text(comp_id)?, raw_args).await?;
            info!(log, "Called setup: {} ({})", name, comp_id);
        } else {
            info!(log, "Skip calling setup: {} ({})", name, comp_id);
        };
    }

    // exec: set_task
    for (name, comp_id) in &components {
        let (component_type, component_path) = component_path_mapping
            .get(name.as_str())
            .context(format!("Component not found: {}", &name))?;
        let generator = codegen::generator(*component_type, component_path, name)?;
        if let Some(args) = generator.manifest().timer_settings() {
            info!(log, "Calling set_task: {} ({})", name, comp_id);
            call_set_task(&wallet, Principal::from_text(comp_id)?, &args).await?;
            info!(log, "Called set_task: {} ({})", name, comp_id);
        } else {
            info!(log, "Skip calling set_task: {} ({})", name, comp_id);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::commands::{
        new,
        test::tests::{run, test_env},
    };

    use super::*;

    fn set_up(project_name: &str) {
        let _ = new::exec(
            &test_env(),
            new::NewOpts {
                project_name: Some(project_name.to_string()),
                no_samples: false,
                example: None,
            },
        );
    }
    fn tear_down(project_name: &str) {
        fs::remove_dir_all(project_name).unwrap();
    }
    #[test]
    fn test_exec() {
        let project_name = "exec_test_exec";
        run(
            || {
                set_up(project_name);
            },
            || {
                let _ = exec(
                    &test_env(),
                    ExecOpts {
                        path: Some(project_name.to_string()),
                        component: None,
                        context: None,
                        wallet: None,
                        network: Network::Local,
                        port: None,
                    },
                );
            },
            || {
                tear_down(project_name);
            },
        );
    }
}
