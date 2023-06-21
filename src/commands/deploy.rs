use std::{path::Path, process::Command};

use anyhow::bail;
use clap::Parser;
use slog::{info, debug, error};

use crate::lib::environment::EnvironmentImpl;

#[derive(Debug, Parser)]
#[command(name = "deploy")]
/// Deploy your ChainSight's project
pub struct DeployOpts {
    #[arg(long)]
    path: Option<String>,
    #[arg(long)]
    port: Option<u16>,
}

const GLOBAL_ERROR_MSG: &str = "Fail 'Deploy' command";

pub fn exec(env: &EnvironmentImpl, opts: DeployOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let builded_project_path_str = opts.path.unwrap_or(".".to_string());
    let builded_project_path = Path::new(&builded_project_path_str);
    let port = opts.port.unwrap_or(4943);
    let local_subnet = format!("http://127.0.0.1:{}", port);

    info!(
        log,
        r#"Deploy project..."#
    );

    // execute command
    info!(log, "Ping dfx local subnet");
    let output = Command::new("dfx")
        .current_dir(&builded_project_path)
        .args(["ping", &local_subnet])
        .output()
        .expect("failed to execute process: dfx ping");
    if output.status.success() {
        debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
        info!(log, "Ping dfx local subnet successfully");
    } else {
        error!(log, "Ping dfx local subnet failed");
        bail!(GLOBAL_ERROR_MSG.to_string())
    }
    info!(log, "Execute 'dfx canister create --all'");
    let output = Command::new("dfx")
        .current_dir(&builded_project_path)
        .args(["canister", "create", "--all"])
        .output()
        .expect("failed to execute process: dfx canister create --all");
    if output.status.success() {
        debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
        info!(log, "Executed 'dfx canister create --all'");
    } else {
        error!(log, "Failed to execute 'dfx canister create --all");
        bail!(GLOBAL_ERROR_MSG.to_string())
    }
    info!(log, "Execute 'dfx build'");
    let output = Command::new("dfx")
        .current_dir(&builded_project_path)
        .arg("build")
        .output()
        .expect("failed to execute process: dfx build");
    if output.status.success() {
        debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
        info!(log, "Executed 'dfx build'");
    } else {
        error!(log, "Failed to execute 'dfx build");
        bail!(GLOBAL_ERROR_MSG.to_string())
    }
    info!(log, "Execute 'dfx canister install --all'");
    let output = Command::new("dfx")
        .current_dir(&builded_project_path)
        .args(["canister", "install", "--all"])
        .output()
        .expect("failed to execute process: dfx canister install --all");
    if output.status.success() {
        debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
        info!(log, "Executed 'dfx canister install --all'");
    } else {
        error!(log, "Failed to execute 'dfx canister install --all");
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    info!(
        log,
        r#"Deploy to local successfully"#,
    );

    Ok(())
}