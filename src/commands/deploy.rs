use std::{collections::BTreeMap, fs::File, path::Path, process::Command};

use anyhow::bail;
use clap::Parser;
use slog::{debug, error, info, Logger};

use crate::{
    lib::{
        codegen::project::ProjectManifestData,
        environment::EnvironmentImpl,
        utils::{ARTIFACTS_DIR, PROJECT_MANIFEST_FILENAME},
    },
    types::Network,
};

#[derive(Debug, Parser)]
#[command(name = "deploy")]
/// Deploy the components of your project.
/// If you want to operate on a local network, you need to build a local dfx network in advance.
pub struct DeployOpts {
    /// Specify the path of the project to deploy.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    path: Option<String>,

    /// Specify the component to deploy.
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

pub fn exec(env: &EnvironmentImpl, opts: DeployOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path_str = opts.path.unwrap_or(".".to_string());
    let artifacts_path_str = format!("{}/{}", &project_path_str, ARTIFACTS_DIR);
    let artifacts_path = Path::new(&artifacts_path_str);
    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;
    let network = opts.network;

    info!(log, "Checking environments...");
    check_before_deployment(log, artifacts_path, opts.port, network.clone())?;
    info!(log, "Checking environments finished successfully");

    // exec command - execution
    info!(
        log,
        r#"Start deploying project '{}'..."#, project_manifest.label
    );
    execute_deployment(log, &artifacts_path_str, opts.component, network)?;
    info!(
        log,
        r#"Project '{}' deployed successfully"#, project_manifest.label
    );

    Ok(())
}

fn check_before_deployment(
    log: &Logger,
    artifacts_path: &Path,
    port: Option<u16>,
    network: Network,
) -> anyhow::Result<()> {
    let local_subnet = format!("http://127.0.0.1:{}", port.unwrap_or(4943));

    exec_command(
        log,
        "dfx",
        artifacts_path,
        match network {
            Network::Local => vec!["ping", &local_subnet],
            Network::IC => vec!["ping", "ic"],
        },
    )?;

    let exec =
        |args: Vec<&str>| -> anyhow::Result<()> { exec_command(log, "dfx", artifacts_path, args) };
    let args_builder = DfxArgsBuilder::new_only_network(network);
    exec(args_builder.generate(vec!["identity", "whoami"]))?;
    exec(args_builder.generate(vec!["identity", "get-principal"]))?;
    exec(args_builder.generate(vec!["identity", "get-wallet"]))?;

    Ok(())
}

fn execute_deployment(
    log: &Logger,
    artifacts_path_str: &str,
    component: Option<String>,
    network: Network,
) -> anyhow::Result<()> {
    let artifacts_path = Path::new(&artifacts_path_str);

    let exec =
        |args: Vec<&str>| -> anyhow::Result<()> { exec_command(log, "dfx", artifacts_path, args) };
    let args_builder = DfxArgsBuilder::new(network.clone(), component);
    exec(args_builder.generate(vec!["canister", "create"]))?;
    exec(args_builder.generate(vec!["build"]))?;
    exec(args_builder.generate(vec!["canister", "install"]))?;

    // Check deployed ids
    info!(log, "List deployed canister ids");
    match read_canister_ids_json(artifacts_path_str, network) {
        Ok(contents) => info!(log, "{:#?}", contents),
        Err(err) => error!(log, "Error reading canister_ids.json: {}", err),
    }

    Ok(())
}

fn exec_command(
    log: &Logger,
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> anyhow::Result<()> {
    let cmd_string = format!("{} {}", cmd, args.join(" "));
    info!(log, "Running command: '{}'", cmd_string);

    let output = Command::new(cmd)
        .current_dir(execution_dir)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute process: {}", cmd_string));
    if output.status.success() {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
        );
        info!(log, "Suceeded: {}", cmd_string);
        anyhow::Ok(())
    } else {
        bail!(format!(
            "Failed: {} by: {} ",
            cmd_string,
            std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr")
        ));
    }
}

struct DfxArgsBuilder {
    network: Network,
    with_component_flag: bool,
    component: Option<String>,
}
impl DfxArgsBuilder {
    fn new(network: Network, component: Option<String>) -> Self {
        Self {
            network,
            with_component_flag: true,
            component,
        }
    }

    fn new_only_network(network: Network) -> Self {
        Self {
            network,
            with_component_flag: false,
            component: None,
        }
    }

    fn generate<'a>(&'a self, args: Vec<&'a str>) -> Vec<&'a str> {
        let mut args = args.clone();

        // network
        args = match self.network {
            Network::Local => args,
            Network::IC => Self::with_ic_network(args),
        };

        // component
        args = if self.with_component_flag {
            if let Some(c) = &self.component {
                Self::with_component(args, c)
            } else {
                Self::with_all(args)
            }
        } else {
            args
        };

        args
    }

    fn with_ic_network(mut args: Vec<&str>) -> Vec<&str> {
        args.push("--network");
        args.push("ic");
        args
    }

    fn with_all(mut args: Vec<&str>) -> Vec<&str> {
        args.push("--all");
        args
    }

    fn with_component<'a>(mut args: Vec<&'a str>, component: &'a str) -> Vec<&'a str> {
        args.push(component);
        args
    }
}

pub type CanisterName = String;
pub type NetworkName = String;
pub type CanisterIdString = String;
pub type CanisterIds = BTreeMap<CanisterName, BTreeMap<NetworkName, CanisterIdString>>;
fn read_canister_ids_json(
    artifacts_path_str: &str,
    network: Network,
) -> anyhow::Result<CanisterIds> {
    let json_filename = "canister_ids.json";
    let json_path = match network {
        Network::Local => format!("{}/.dfx/local/{}", artifacts_path_str, json_filename),
        Network::IC => format!("{}/{}", artifacts_path_str, json_filename),
    };
    let json = File::open(json_path)?;
    let canister_ids: CanisterIds = serde_json::from_reader(json)?;
    Ok(canister_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfx_args_builder_only_network() {
        struct Input<'a> {
            pub cmd: Vec<&'a str>,
            pub network: Network,
        }
        struct InOut<'a> {
            pub in_: Input<'a>,
            pub out: String,
        }

        let input_output: Vec<InOut> = vec![
            InOut {
                in_: Input {
                    cmd: vec!["identity", "whoami"],
                    network: Network::Local,
                },
                out: "identity whoami".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["identity", "whoami"],
                    network: Network::IC,
                },
                out: "identity whoami --network ic".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["identity", "get-principal"],
                    network: Network::Local,
                },
                out: "identity get-principal".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["identity", "get-principal"],
                    network: Network::IC,
                },
                out: "identity get-principal --network ic".to_string(),
            },
        ];

        for InOut { in_, out } in input_output {
            let args_builder = DfxArgsBuilder::new_only_network(in_.network);
            let actual = args_builder.generate(in_.cmd);
            assert_eq!(actual.join(" "), out);
        }
    }

    #[test]
    fn test_dfx_args_builder_with_components() {
        struct Input<'a> {
            pub cmd: Vec<&'a str>,
            pub network: Network,
            pub component: Option<String>,
        }
        struct InOut<'a> {
            pub in_: Input<'a>,
            pub out: String,
        }

        let input_output: Vec<InOut> = vec![
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::Local,
                    component: None,
                },
                out: "canister create --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::IC,
                    component: None,
                },
                out: "canister create --network ic --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::Local,
                    component: Some("icrc1_component".to_string()),
                },
                out: "canister create icrc1_component".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::IC,
                    component: Some("icrc1_component".to_string()),
                },
                out: "canister create --network ic icrc1_component".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::Local,
                    component: None,
                },
                out: "build --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::IC,
                    component: None,
                },
                out: "build --network ic --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::Local,
                    component: Some("icrc1_component".to_string()),
                },
                out: "build icrc1_component".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::IC,
                    component: Some("icrc1_component".to_string()),
                },
                out: "build --network ic icrc1_component".to_string(),
            },
        ];

        for InOut { in_, out } in input_output {
            let args_builder = DfxArgsBuilder::new(in_.network, in_.component);
            let actual = args_builder.generate(in_.cmd);
            assert_eq!(actual.join(" "), out);
        }
    }
}
