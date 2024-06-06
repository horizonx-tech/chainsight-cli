use std::path::Path;

use super::deploy::ComponentIdsManager;
use anyhow::bail;
use clap::{arg, Parser};
use ic_agent::Identity;
use slog::{info, Logger};
use types::ComponentsToInitialize;

use crate::{
    lib::{
        codegen::{
            components::{codegen::generator, common::ComponentTypeInManifest},
            project::ProjectManifestData,
        },
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
    let artifacts_path_str = format!("{}/artifacts", &project_path_str);
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
    // todo: enable to selec identity context, wallet
    let caller_identity = identity_from_keyring()?;
    let caller_principal = caller_identity.sender().map_err(|e| anyhow::anyhow!(e))?;

    //// for loading component ids
    let dfx_bin_network = match network {
        Network::Local => DfxWrapperNetwork::Local(port),
        Network::IC => DfxWrapperNetwork::IC,
    };
    let comp_id_mgr = ComponentIdsManager::load(&dfx_bin_network, artifacts_path)?;
    let components = match components {
        ComponentsToInitialize::Single(name) => vec![name],
        ComponentsToInitialize::Multiple(names) => names,
    };

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
