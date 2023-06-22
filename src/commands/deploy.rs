use std::{path::Path, process::Command};

use anyhow::bail;
use clap::Parser;
use slog::{info, debug, error, Logger};

use crate::lib::environment::EnvironmentImpl;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Network {
    Local,
    IC // ref: https://internetcomputer.org/docs/current/developer-docs/setup/deploy-mainnet#step-2--check-the-current-status-of-the-ic-and-your-ability-to-connect-to-it-by-running-the-following-command-for-the-network-alias-ic
}

#[derive(Debug, Parser)]
#[command(name = "deploy")]
/// Deploy your ChainSight's project
pub struct DeployOpts {
    #[arg(long)]
    path: Option<String>,
    #[arg(long)]
    #[clap(default_value="local")]
    network: Network,
    #[arg(long)]
    port: Option<u16>,
}

const GLOBAL_ERROR_MSG: &str = "Fail 'Deploy' command";

pub fn exec(env: &EnvironmentImpl, opts: DeployOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let builded_project_path_str = opts.path.unwrap_or(".".to_string());
    let builded_project_path = Path::new(&builded_project_path_str);
    let network = opts.network;

    info!(
        log,
        r#"Deploy project..."#
    );

    // execute command
    info!(log, "Ping dfx subnet");
    let local_subnet = format!("http://127.0.0.1:{}", opts.port.unwrap_or(4943));
    let args = match network {
        Network::Local => vec!["ping", &local_subnet],
        Network::IC => vec!["ping", "ic"]
    };
    exec_command(
        log,
        "dfx",
        &builded_project_path,
        args,
        "Ping dfx subnet"
    )?;

    info!(log, "Execute 'dfx canister create --all'");
    let args = match network {
        Network::Local => vec!["canister", "create", "--all"],
        Network::IC => vec!["canister", "create", "--all", "--network", "ic"]
    };
    exec_command(
        log,
        "dfx",
        &builded_project_path,
        args,
        "Executed 'dfx canister create --all"
    )?;

    info!(log, "Execute 'dfx build'");
    let args = match network {
        Network::Local => vec!["build"],
        Network::IC => vec!["build", "--network", "ic"]
    };
    exec_command(
        log,
        "dfx",
        &builded_project_path,
        args,
        "Executed 'dfx build'"
    )?;

    info!(log, "Execute 'dfx canister install --all'");
    let args = match network {
        Network::Local => vec!["canister", "install", "--all"],
        Network::IC => vec!["canister", "install", "--all", "--network", "ic"]
    };
    exec_command(
        log,
        "dfx",
        &builded_project_path,
        args,
        "Executed 'dfx canister install --all'"
    )?;

    info!(
        log,
        r#"Deploy successfully"#,
    );

    Ok(())
}

fn exec_command(
    log: &Logger,
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
    complete_message: &str
) -> anyhow::Result<()> {
    let cmd_string = format!("{} {}", cmd, args.join(" "));
    debug!(log, "Running command: `{}`", cmd_string);

    let output = Command::new(cmd)
        .current_dir(&execution_dir)
        .args(args)
        .output()
        .expect(&format!("failed to execute process: {}", cmd_string));
    if output.status.success() {
        debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
        info!(log, "{} successfully", complete_message);
        anyhow::Ok(())
    } else {
        debug!(log, "{}", std::str::from_utf8(&output.stderr).unwrap_or(&"fail to parse stderr"));
        error!(log, "{} failed", complete_message);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }
}
