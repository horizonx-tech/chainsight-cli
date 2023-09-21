use std::{fs, path::Path, process::Command};

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
    /// Specify the path of the project to be deployed.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    path: Option<String>,

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

    // exec command - check
    info!(log, "Checking environments...");
    let local_subnet = format!("http://127.0.0.1:{}", opts.port.unwrap_or(4943));
    let args = match network {
        Network::Local => vec!["ping", &local_subnet],
        Network::IC => vec!["ping", "ic"],
    };
    exec_command(log, "dfx", artifacts_path, args)?;

    let args = match network {
        Network::Local => vec!["identity", "whoami"],
        Network::IC => vec!["identity", "whoami", "--network", "ic"],
    };
    exec_command(log, "dfx", artifacts_path, args)?;
    let args = match network {
        Network::Local => vec!["identity", "get-principal"],
        Network::IC => vec!["identity", "get-principal", "--network", "ic"],
    };
    exec_command(log, "dfx", artifacts_path, args)?;
    let args = match network {
        Network::Local => vec!["identity", "get-wallet"],
        Network::IC => vec!["identity", "get-wallet", "--network", "ic"],
    };
    exec_command(log, "dfx", artifacts_path, args)?;
    info!(log, "Checking environments finished successfully");

    // exec command - execution
    info!(
        log,
        r#"Start deploying project '{}'..."#, project_manifest.label
    );
    let args = match network {
        Network::Local => vec!["canister", "create", "--all"],
        Network::IC => vec!["canister", "create", "--all", "--network", "ic"],
    };
    exec_command(log, "dfx", artifacts_path, args)?;

    let args = match network {
        Network::Local => vec!["build"],
        Network::IC => vec!["build", "--network", "ic"],
    };
    exec_command(log, "dfx", artifacts_path, args)?;

    let args = match network {
        Network::Local => vec!["canister", "install", "--all"],
        Network::IC => vec!["canister", "install", "--all", "--network", "ic"],
    };
    exec_command(log, "dfx", artifacts_path, args)?;

    info!(log, "List deployed canister ids");
    let canister_ids_json_filename = "canister_ids.json";
    let canister_ids_json_path = match network {
        Network::Local => format!(
            "{}/.dfx/local/{}",
            artifacts_path_str, canister_ids_json_filename
        ),
        Network::IC => format!("{}/{}", artifacts_path_str, canister_ids_json_filename),
    };
    match fs::read_to_string(canister_ids_json_path) {
        Ok(contents) => info!(log, "{}", contents),
        Err(err) => error!(log, "Error reading canister_ids.json: {}", err),
    }

    info!(
        log,
        r#"Project '{}' deployed successfully"#, project_manifest.label
    );

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
