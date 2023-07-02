use std::{path::Path, fs, os::unix::prelude::PermissionsExt};

use anyhow::{bail, Ok};
use clap::{arg, Parser};
use slog::{error, info, Logger};

use crate::{lib::{environment::EnvironmentImpl, utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME}, codegen::{project::ProjectManifestData, components::{common::{ComponentTypeInManifest, ComponentManifest}, event_indexer::EventIndexerComponentManifest, snapshot::SnapshotComponentManifest, relayer::RelayerComponentManifest}}}, types::{ComponentType, Network}};

#[derive(Debug, Parser)]
#[command(name = "exec")]
/// Execute Component's processing
pub struct ExecOpts {
    #[arg(long)]
    path: Option<String>,
    #[arg(long)]
    component: Option<String>,
    #[clap(default_value="local")]
    network: Network,
    #[arg(long, conflicts_with = "only_execute_cmds")]
    only_generate_cmds: bool,
    #[arg(long, conflicts_with = "only_generate_cmds")]
    only_execute_cmds: bool,
}

const GLOBAL_ERROR_MSG: &str = "Fail 'Exec' command";

pub fn exec(env: &EnvironmentImpl, opts: ExecOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        error!(
            log,
            r#"{}"#,
            msg
        );
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    info!(
        log,
        r#"Execute canister processing..."#
    );

    let project_path_str = project_path.unwrap_or(".".to_string());
    let artifacts_path_str = format!("{}/artifacts", &project_path_str);

    // load component definitions from manifests
    let project_manifest = ProjectManifestData::load(&format!("{}/{}", &project_path_str, PROJECT_MANIFEST_FILENAME))?;
    let mut component_data = vec![];
    for component in project_manifest.components.clone() {
        // TODO: need validations
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", &project_path_str, relative_component_path);
        let component_type = ComponentTypeInManifest::determine_type(&component_path)?;

        let data: Box<dyn ComponentManifest> = match component_type {
            ComponentType::EventIndexer => Box::new(EventIndexerComponentManifest::load(&component_path)?),
            ComponentType::Snapshot => Box::new(SnapshotComponentManifest::load(&component_path)?),
            ComponentType::Relayer => Box::new(RelayerComponentManifest::load(&component_path)?),
        };
        component_data.push(data);
    };

    if opts.only_generate_cmds {
        info!(log, r#"Skip to generate commands to call components"#);
    } else {
        // generate commands
        execute_to_generate_commands(log, &artifacts_path_str, opts.network, &component_data)?;
    }

    if opts.only_execute_cmds {
        info!(log, r#"Skip to execute commands to components"#);
    } else {
        // execute commands
        execute_commands()?;
    }

    info!(
        log,
        r#"Project "{}" canisters executed successfully"#,
        project_manifest.label
    );
    Ok(())
}

fn execute_to_generate_commands(log: &Logger, builded_project_path_str: &str, network: Network, component_data: &Vec<Box<dyn ComponentManifest>>) -> anyhow::Result<()> {
    // generate /scripts
    let script_root_path_str = format!("{}/scripts", &builded_project_path_str);
    let scripts_path_str = format!("{}/scripts/components", &builded_project_path_str);
    let script_root_path = Path::new(&script_root_path_str);
    if script_root_path.exists() {
        fs::remove_dir_all(&script_root_path)?;
    }
    fs::create_dir_all(Path::new(&scripts_path_str))?;

    let mut shells_to_call: Vec<String> = vec![];
    for data in component_data {
        let filepath = format!("{}/{}.sh", &scripts_path_str, data.label());
        fs::write(
            &filepath,
            data.generate_scripts(network.clone())?
        )?;

        let mut perms = fs::metadata(&filepath)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&filepath, perms)?;

        shells_to_call.push(format!("./components/{}.sh", data.label()));
    }

    // temp
    // - automatic relative path setting
    let entrypoint_filepath = format!("{}/entrypoint.sh", &script_root_path_str);
    fs::write(
        &entrypoint_filepath,
        shells_to_call.join("\n")
    )?;
    let mut perms = fs::metadata(&entrypoint_filepath)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&entrypoint_filepath, perms)?;


    Ok(())
}

fn execute_commands() -> anyhow::Result<()> {
    Ok(())
}
