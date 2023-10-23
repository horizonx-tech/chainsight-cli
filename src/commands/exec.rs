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
                snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
                snapshot_indexer_https::SnapshotIndexerHTTPSComponentManifest,
                snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
            },
            project::ProjectManifestData,
        },
        environment::EnvironmentImpl,
        utils::{
            env::cache_envfile, is_chainsight_project, DOTENV_FILENAME, PROJECT_MANIFEST_FILENAME,
        },
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
const UTILS_SHELL_FILENAME: &str = "utils.sh";

pub fn exec(env: &EnvironmentImpl, opts: ExecOpts) -> anyhow::Result<()> {
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
    let mut component_data = vec![];
    for component in project_manifest.components.clone() {
        // TODO: need validations
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", &project_path_str, relative_component_path);
        let component_type = ComponentTypeInManifest::determine_type(&component_path)?;
        let id = Path::new(&component_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        let data: Box<dyn ComponentManifest> =
            match component_type {
                ComponentType::EventIndexer => Box::new(
                    EventIndexerComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::AlgorithmIndexer => Box::new(
                    AlgorithmIndexerComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::SnapshotIndexerICP => Box::new(
                    SnapshotIndexerICPComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::SnapshotIndexerEVM => Box::new(
                    SnapshotIndexerEVMComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::Relayer => {
                    Box::new(RelayerComponentManifest::load_with_id(&component_path, id)?)
                }
                ComponentType::AlgorithmLens => Box::new(
                    AlgorithmLensComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::SnapshotIndexerHTTPS => Box::new(
                    SnapshotIndexerHTTPSComponentManifest::load_with_id(&component_path, id)?,
                ),
            };
        component_data.push(data);
    }

    let artifacts_path_str = format!("{}/artifacts", &project_path_str);

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
    // generate scripts per component (/scripts/components)
    let script_root_path_str = format!("{}/scripts", &built_project_path_str);
    let scripts_path_str = format!("{}/scripts/components", &built_project_path_str);
    let script_root_path = Path::new(&script_root_path_str);
    if script_root_path.exists() {
        fs::remove_dir_all(script_root_path)?;
    }
    fs::create_dir_all(Path::new(&scripts_path_str))?;

    for data in component_data {
        let id = data.id().unwrap();
        let filepath = format!("{}/{}.sh", &scripts_path_str, &id);
        fs::write(&filepath, data.generate_scripts(network.clone())?)?;

        chmod_executable(&filepath)?;

        info!(
            log,
            r#"Script for Component "{}" generated successfully"#, &id
        );
    }

    // generate common scripts (/scripts)
    let entrypoint_filepath = format!("{}/{}", &script_root_path_str, ENTRYPOINT_SHELL_FILENAME);
    let component_ids = component_data
        .iter()
        .map(|c| c.id().unwrap())
        .collect::<Vec<String>>();
    fs::write(&entrypoint_filepath, entrypoint_sh(component_ids))?;
    let utils_filepath = format!("{}/{}", &script_root_path_str, UTILS_SHELL_FILENAME);
    fs::write(&utils_filepath, utils_sh())?;
    for path in [&entrypoint_filepath, &utils_filepath] {
        chmod_executable(path)?;
    }

    info!(log, r#"Entrypoint Script generated successfully"#);

    anyhow::Ok(())
}

fn chmod_executable(path: &str) -> anyhow::Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)?;
    Ok(())
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
            "Failed: {} by: {}\n(stdout at run time)\n{}",
            complete_msg,
            std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr"),
            std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
        ));
    }

    anyhow::Ok(())
}

fn entrypoint_sh(component_ids: Vec<String>) -> String {
    let contents_for_component = component_ids
        .iter()
        .map(|id| {
            format!(
                r#"
echo "Run script for '{}'"
. "$script_dir/components/{}.sh"
"#,
                &id, &id
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"#!/bin/bash
script_dir=$(dirname "$(readlink -f "$0")")

. "$script_dir/utils.sh"

set -e -o pipefail
trap 'on_error $BASH_SOURCE $LINENO "$BASH_COMMAND" "$@"' ERR

{}
"#,
        contents_for_component
    )
}

fn utils_sh() -> String {
    r#"#!/bin/bash

function on_error()
{
    status=$?
    script=$1
    line=$2
    command=$3

    {
        # echo "Status: $status"
        echo "occured on $script [Line $line]"
        echo "command: $command"
    } 1>&2
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;

    use crate::{
        commands::{
            new,
            test::tests::{run, test_env},
        },
        lib::logger::create_root_logger,
    };

    use super::*;

    fn set_up(project_name: &str) {
        let _ = new::exec(
            &test_env(),
            new::NewOpts {
                project_name: project_name.to_string(),
                no_samples: false,
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
                        only_generate_cmds: true,
                        only_execute_cmds: false,
                    },
                );
            },
            || {
                tear_down(project_name);
            },
        );
    }
    #[test]
    fn test_execute_commands() {
        let project_name = "exec_test_execute_commands";
        let custom_setup = || {
            fs::create_dir_all(format!("{}/scripts", project_name)).unwrap();
            let entrypoint_filepath =
                format!("{}/scripts/{}", project_name, ENTRYPOINT_SHELL_FILENAME);
            fs::write(
                &entrypoint_filepath,
                "#!/bin/bash
            echo 'Dummy script\r'",
            )
            .unwrap();
            fs::set_permissions(entrypoint_filepath, PermissionsExt::from_mode(0o755)).unwrap();
        };
        run(
            custom_setup,
            || {
                let result = execute_commands(&create_root_logger(1), project_name);
                assert!(result.is_ok());
            },
            || {
                tear_down(project_name);
            },
        );
    }

    #[test]
    fn test_scripts_snapshot() {
        let project_name = "exec_test_scripts_snapshot";
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
                        only_generate_cmds: true,
                        only_execute_cmds: false,
                    },
                );
                let project = ProjectManifestData::load(&format!(
                    "{}/{}",
                    project_name, PROJECT_MANIFEST_FILENAME
                ))
                .unwrap();

                let scripts_root = format!("{}/artifacts/scripts", project_name);
                let entrypoint_sh_path = format!("{}/{}", scripts_root, ENTRYPOINT_SHELL_FILENAME);
                assert_display_snapshot!(
                    ENTRYPOINT_SHELL_FILENAME,
                    fs::read_to_string(entrypoint_sh_path).unwrap()
                );
                let utils_sh_path = format!("{}/{}", scripts_root, UTILS_SHELL_FILENAME);
                assert_display_snapshot!(
                    UTILS_SHELL_FILENAME,
                    fs::read_to_string(utils_sh_path).unwrap()
                );

                let component_ids = project
                    .components
                    .iter()
                    .map(|c| {
                        let path = std::path::Path::new(c.component_path.as_str());
                        path.file_stem().unwrap().to_str().unwrap().to_string()
                    })
                    .collect::<Vec<String>>();
                for id in component_ids {
                    let component_sh_path = format!("{}/components/{}.sh", scripts_root, id);
                    assert_display_snapshot!(
                        format!("{}.sh", id),
                        fs::read_to_string(component_sh_path).unwrap()
                    );
                }
            },
            || {
                tear_down(project_name);
            },
        );
    }
}
