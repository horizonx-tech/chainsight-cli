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
    /// Specify the path of the project to deploy.
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

    info!(log, "Checking environments...");
    check_before_deployment(log, artifacts_path, opts.port, network.clone())?;
    info!(log, "Checking environments finished successfully");

    // exec command - execution
    info!(
        log,
        r#"Start deploying project '{}'..."#, project_manifest.label
    );
    execute_deployment(log, &artifacts_path_str, network)?;
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

    let generate_args: fn(Vec<&'static str>) -> Vec<&'static str> = match network {
        Network::Local => |args| args,
        Network::IC => |args| args_with_ic_network(args),
    };
    let exec =
        |args: Vec<&str>| -> anyhow::Result<()> { exec_command(log, "dfx", artifacts_path, args) };

    exec(generate_args(vec!["identity", "whoami"]))?;
    exec(generate_args(vec!["identity", "get-principal"]))?;
    exec(generate_args(vec!["identity", "get-wallet"]))?;

    Ok(())
}

fn execute_deployment(
    log: &Logger,
    artifacts_path_str: &str,
    network: Network,
) -> anyhow::Result<()> {
    let artifacts_path = Path::new(&artifacts_path_str);

    let generate_args: fn(Vec<&'static str>) -> Vec<&'static str> = match network {
        Network::Local => |args| args,
        Network::IC => |args| args_with_ic_network(args),
    };
    let exec =
        |args: Vec<&str>| -> anyhow::Result<()> { exec_command(log, "dfx", artifacts_path, args) };

    exec(generate_args(vec!["canister", "create", "--all"]))?;
    exec(generate_args(vec!["build"]))?;
    exec(generate_args(vec!["canister", "install", "--all"]))?;

    // Check deployed ids
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

    Ok(())
}

fn args_with_ic_network(args: Vec<&str>) -> Vec<&str> {
    let mut args = args.clone();
    args.push("--network");
    args.push("ic");
    args
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
