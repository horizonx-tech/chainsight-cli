use std::path::Path;

// temp
use super::deploy::{
    functions::{get_agent, get_wallet_principal_from_local_context, wallet_canister},
    ComponentIdsManager,
};
use anyhow::{bail, Context};
use candid::Principal;
use clap::{arg, Parser};
use functions::{call_init_in, call_set_task, SetTaskArgs};
use ic_agent::Identity;
use slog::{info, Logger};
use types::ComponentsToInitialize;

use crate::{
    lib::{
        codegen::project::ProjectManifestData,
        environment::EnvironmentImpl,
        utils::{
            dfx::DfxWrapperNetwork, env::cache_envfile, identity::identity_from_keyring,
            is_chainsight_project, ARTIFACTS_DIR, DOTENV_FILENAME, PROJECT_MANIFEST_FILENAME,
        },
    },
    types::Network,
};

mod functions;
mod types;

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

    // load component definitions from manifests
    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;
    let artifacts_path_str = format!("{}/{}", &project_path_str, ARTIFACTS_DIR);
    let components = if let Some(component) = opts.component {
        ComponentsToInitialize::Single(component)
    } else {
        // todo: clean to collect component ids, better to use only manifest.yaml?
        let components = project_manifest
            .load_code_generator(&project_path_str)?
            .iter()
            .map(|cg| cg.manifest().id().unwrap())
            .collect::<Vec<_>>();
        ComponentsToInitialize::Multiple(components)
    };

    execute_initialize_components(
        log,
        &artifacts_path_str,
        components,
        opts.network,
        opts.port,
    )
    .await?;

    info!(
        log,
        r#"Project "{}" canisters executed successfully"#, project_manifest.label
    );

    Ok(())
}

async fn execute_initialize_components(
    log: &Logger,
    artifacts_path: &str,
    components: ComponentsToInitialize,
    network: Network,
    port: Option<u16>,
) -> anyhow::Result<()> {
    //// for loading component ids
    let dfx_bin_network = match network {
        Network::Local => DfxWrapperNetwork::Local(port),
        Network::IC => DfxWrapperNetwork::IC,
    };
    let comp_id_mgr = ComponentIdsManager::load(&dfx_bin_network, artifacts_path)?;
    let components = match components {
        ComponentsToInitialize::Single(name) => {
            let comp_id = comp_id_mgr
                .get(&name)
                .context(format!("Component not found: {}", name))?;
            vec![(name, comp_id)]
        }
        ComponentsToInitialize::Multiple(names) => names
            .iter()
            .map(|name| {
                let comp_id = comp_id_mgr
                    .get(name)
                    .context(format!("Component not found: {}", name))?;
                Ok((name.clone(), comp_id))
            })
            .collect::<anyhow::Result<Vec<_>>>()?,
    };

    // generate wallet canister
    // todo: enable to select identity context, wallet
    let caller_identity = identity_from_keyring(None)?;
    println!(
        "caller_identity: {:?}",
        caller_identity.sender().unwrap().to_text()
    );
    let agent = get_agent(Box::new(caller_identity), &network, port).await?;
    let wallet_canister_id = get_wallet_principal_from_local_context(&network, port).await?;
    println!("wallet_canister_id: {:?}", wallet_canister_id.to_text());
    let wallet = wallet_canister(wallet_canister_id, &agent).await?;

    // exec
    for (name, comp_id) in &components {
        info!(log, "Calling init_in: {} ({})", name, comp_id);
        call_init_in(&wallet, Principal::from_text(comp_id)?).await?;
        info!(log, "Called init_in: {} ({})", name, comp_id);
    }
    // todo: call `setup`
    for (name, comp_id) in &components {
        info!(log, "Calling set_task: {} ({})", name, comp_id);
        call_set_task(
            &wallet,
            Principal::from_text(comp_id)?,
            SetTaskArgs {
                task_interval_secs: 60,
                delay_secs: 5,
                is_rounded_start_time: true,
            }, // todo: set args
        )
        .await?;
        info!(log, "Called set_task: {} ({})", name, comp_id);
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
