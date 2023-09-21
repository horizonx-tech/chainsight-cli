use std::{fs, os::unix::prelude::PermissionsExt, path::Path, process::Command};

use anyhow::bail;
use clap::{arg, Parser};
use slog::{debug, info, Logger};

use crate::{
    lib::{
        codegen::{
            components::{
                algorithm_indexer::AlgorithmIndexerComponentManifest,
                algorithm_lens::AlgorithmLensComponentManifest,
                common::{ComponentManifest, ComponentTypeInManifest},
                event_indexer::EventIndexerComponentManifest,
                relayer::RelayerComponentManifest,
                snapshot_indexer::SnapshotComponentManifest,
                snapshot_json_rpc::SnapshotJsonRPCComponentManifest,
            },
            project::ProjectManifestData,
        },
        environment::EnvironmentImpl,
        utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME},
    },
    types::{ComponentType, Network},
};

#[derive(Debug, Parser)]
#[command(name = "exec")]
/// Calls for component processing. Currently supports initialization and task start instructions.
pub struct ExecOpts {
    /// Specify the path of the project that manages the component to be called.
    /// Refer to the manifest of this project to build the commands that should be executed.
    #[arg(long)]
    path: Option<String>,

    /// Specify the name of the component you want to execute.
    /// If this option is not specified, the command will be given to all components managed by the project.
    #[arg(long)]
    component: Option<String>,

    /// Specify the network to execute on.
    #[arg(long)]
    #[clap(default_value = "local")]
    network: Network,

    /// Only generate commands.
    #[arg(long, conflicts_with = "only_execute_cmds")]
    only_generate_cmds: bool,

    /// Only execute commands.
    /// Perform this steps with commands already generated.
    #[arg(long, conflicts_with = "only_generate_cmds")]
    only_execute_cmds: bool,
}

const ENTRYPOINT_SHELL_FILENAME: &str = "entrypoint.sh";

pub fn exec(env: &EnvironmentImpl, opts: ExecOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        bail!(format!(r#"{}"#, msg));
    }

    info!(log, r#"Execute canister processing..."#);

    let project_path_str = project_path.unwrap_or(".".to_string());
    let artifacts_path_str = format!("{}/artifacts", &project_path_str);

    // load component definitions from manifests
    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;
    let mut component_data = vec![];
    for component in project_manifest.components.clone() {
        // TODO: need validations
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", &project_path_str, relative_component_path);
        let component_type = ComponentTypeInManifest::determine_type(&component_path)?;

        let data: Box<dyn ComponentManifest> = match component_type {
            ComponentType::EventIndexer => {
                Box::new(EventIndexerComponentManifest::load(&component_path)?)
            }
            ComponentType::AlgorithmIndexer => {
                Box::new(AlgorithmIndexerComponentManifest::load(&component_path)?)
            }
            ComponentType::SnapshotIndexer => {
                Box::new(SnapshotComponentManifest::load(&component_path)?)
            }
            ComponentType::Relayer => Box::new(RelayerComponentManifest::load(&component_path)?),
            ComponentType::AlgorithmLens => {
                Box::new(AlgorithmLensComponentManifest::load(&component_path)?)
            }
            ComponentType::SnapshotJsonRPC => {
                Box::new(SnapshotJsonRPCComponentManifest::load(&component_path)?)
            }
        };
        component_data.push(data);
    }

    if opts.only_execute_cmds {
        info!(log, r#"Skip to generate commands to call components"#);
    } else {
        // generate commands
        info!(log, r#"Start processing for commands generation..."#);
        execute_to_generate_commands(log, &artifacts_path_str, opts.network, &component_data)?;
    }

    if opts.only_generate_cmds {
        info!(log, r#"Skip to execute commands to components"#);
    } else {
        // execute commands
        info!(log, r#"Start processing for commands execution..."#);
        execute_commands(log, &artifacts_path_str)?;
    }

    info!(
        log,
        r#"Project "{}" canisters executed successfully"#, project_manifest.label
    );
    Ok(())
}

fn execute_to_generate_commands(
    log: &Logger,
    built_project_path_str: &str,
    network: Network,
    component_data: &Vec<Box<dyn ComponentManifest>>,
) -> anyhow::Result<()> {
    // generate /scripts
    let script_root_path_str = format!("{}/scripts", &built_project_path_str);
    let scripts_path_str = format!("{}/scripts/components", &built_project_path_str);
    let script_root_path = Path::new(&script_root_path_str);
    if script_root_path.exists() {
        fs::remove_dir_all(script_root_path)?;
    }
    fs::create_dir_all(Path::new(&scripts_path_str))?;

    for data in component_data {
        let filepath = format!("{}/{}.sh", &scripts_path_str, data.metadata().label);
        fs::write(&filepath, data.generate_scripts(network.clone())?)?;

        let mut perms = fs::metadata(&filepath)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&filepath, perms)?;

        info!(
            log,
            r#"Script for Component "{}" generated successfully"#,
            data.metadata().label
        );
    }

    // temp
    // - automatic relative path setting
    let entrypoint_filepath = format!("{}/{}", &script_root_path_str, ENTRYPOINT_SHELL_FILENAME);
    let component_names = component_data
        .iter()
        .map(|c| c.metadata().label.to_string())
        .collect::<Vec<String>>();
    let entrypoint_contents = format!(
        r#"#!/bin/bash
script_dir=$(dirname "$(readlink -f "$0")")
{}
"#,
        component_names
            .iter()
            .map(|label| format!(
                r#"
echo "Run script for '{}'"
. "$script_dir/components/{}.sh"
"#,
                &label, &label
            ))
            .collect::<Vec<String>>()
            .join("\n")
    );
    fs::write(&entrypoint_filepath, entrypoint_contents)?;
    let mut perms = fs::metadata(&entrypoint_filepath)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&entrypoint_filepath, perms)?;

    info!(log, r#"Entrypoint Script generated successfully"#);

    anyhow::Ok(())
}

fn execute_commands(log: &Logger, built_project_path_str: &str) -> anyhow::Result<()> {
    info!(
        log,
        "Run scripts to execute commands for deployed components"
    );
    let cmd_string = format!("./scripts/{}", ENTRYPOINT_SHELL_FILENAME);
    debug!(log, "Running command: `{}`", &cmd_string);
    let output = Command::new(&cmd_string)
        .current_dir(built_project_path_str)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute process: {}", &cmd_string));
    let complete_msg = format!("Executed '{}'", &cmd_string);
    if output.status.success() {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
        );
        info!(log, "{} successfully", complete_msg);
    } else {
        bail!(format!(
            "Failed: {} by: {}",
            complete_msg,
            std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr")
        ));
    }

    anyhow::Ok(())
}
