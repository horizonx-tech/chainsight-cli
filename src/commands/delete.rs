use std::{path::Path, process::Command};

use anyhow::{bail, Context};
use candid::{Decode, Encode, Principal};
use clap::Parser;
use slog::{debug, error, info};

use crate::{
    commands::utils::{output_by_exec_cmd, DfxArgsBuilder},
    lib::{
        environment::EnvironmentImpl,
        utils::{is_chainsight_project, ARTIFACTS_DIR},
    },
    types::Network,
};

#[derive(Debug, Parser)]
#[command(name = "remove")]
/// Delete your Chainsight component. This command deletes the component with sidecars and allows you to recover the remaining cycles.
pub struct DeleteOpts {
    /// Specify the path of the project to be removed.
    /// If not specified, the current directory is targeted.
    #[arg(long, short = 'p')]
    pub path: Option<String>,

    /// Specify the component to remove.
    /// If this option is not specified, the command will be given to all components managed by the project.
    #[arg(long, short = 'c')]
    component: String,

    /// Specify the network to execute on.
    #[arg(long)]
    #[clap(default_value = "local")]
    network: Network,

    /// Specifies the port to call.
    /// This option is used only if the target is localhost.
    #[arg(long)]
    port: Option<u16>,
}

pub async fn exec(env: &EnvironmentImpl, opts: DeleteOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    info!(log, r#"Start deleting component '{}'..."#, opts.component);

    let working_dir_str = working_dir(opts.path.clone())?;
    let working_dir = Path::new(&working_dir_str);

    let url = match &opts.network {
        Network::Local => format!("http://localhost:{}", opts.port.unwrap_or(4943)),
        Network::IC => format!("https://ic0.app/"),
    };
    let component_id = Principal::from_text(&opts.component).unwrap_or_else(|_| {
        canister_id_from_canister_name(working_dir, &opts.network, &opts.component)
            .expect("failed to get canister id")
    });
    let agent = agent(&url);
    if opts.network == Network::Local {
        agent.fetch_root_key().await.unwrap();
    }

    info!(log, "Confirm sidecars to delete");
    let proxy = get_proxy_from_component(&agent, &component_id).await;
    info!(log, "  proxy: {}", proxy.to_text());
    let vault = vault_from_proxy(&agent, &proxy).await;
    info!(log, "  vault: {}", vault.to_text());
    let db = db_from_proxy(&agent, &proxy).await;
    info!(log, "  db: {}", db.to_text());

    let wallet = get_wallet(&opts.network)
        .expect("failed to get wallet")
        .to_string();
    info!(log, "Wallet to execute removal: {}", wallet);

    let exec_delete = |label: &str, canister_id: String| -> bool {
        info!(log, "Deleting {} ({})", label, canister_id);
        let res = delete_canister(working_dir, canister_id, &wallet, &opts.network);
        let is_succeeded = match res {
            Ok(msg) => {
                info!(log, "Deleted {}", label);
                debug!(log, "{}", msg);
                true
            }
            Err(e) => {
                error!(log, "Failed to delete {}: {}", label, e);
                false
            }
        };
        is_succeeded
    };
    let before_balance = get_wallet_balance(&opts.network);
    match before_balance {
        Ok(balance) => info!(log, "Balance before removal: {}", balance),
        Err(e) => error!(log, "Failed to get balance: {}", e),
    }
    let res_db = exec_delete("db", db.to_text());
    let res_vault = exec_delete("vault", vault.to_text());
    let res_proxy = exec_delete("proxy", proxy.to_text());
    let res_component = exec_delete("component", component_id.to_text());
    let after_balance = get_wallet_balance(&opts.network);
    match after_balance {
        Ok(balance) => info!(log, "Balance after removal: {}", balance),
        Err(e) => error!(log, "Failed to get balance: {}", e),
    }

    let msg_from_result_flag = |res: bool| -> String {
        if res {
            "Removed".to_string()
        } else {
            "Fail to remove".to_string()
        }
    };
    info!(
        log,
        r#"Finish deleting component '{}'.
The results of the removing are as follows.
  component {} {}
  proxy {} {}
  vault {} {}
  db {} {}"#,
        opts.component,
        component_id.to_text(),
        msg_from_result_flag(res_component),
        proxy.to_text(),
        msg_from_result_flag(res_proxy),
        vault.to_text(),
        msg_from_result_flag(res_vault),
        db.to_text(),
        msg_from_result_flag(res_db)
    );
    if res_db && res_vault && res_proxy && res_component {
        info!(
            log,
            "If there are canisters that could not be deleted, please delete them manually."
        );
    }

    Ok(())
}

// Return artifacts path as working directory
fn working_dir(project_path: Option<String>) -> anyhow::Result<String> {
    let path = if let Some(project_path) = &project_path {
        // Assuming the specified path is the project path, calculate the artifacts path.
        if let Err(msg) = is_chainsight_project(Some(project_path.clone())) {
            bail!(format!(r#"{}"#, msg));
        };
        format!("{}/{}", project_path, ARTIFACTS_DIR)
    } else {
        let current_path_str = ".";
        let current_path = Path::new(current_path_str);
        // If dfx.json is available, current path is used as artifacts path
        if current_path.join("dfx.json").exists() {
            current_path_str.to_string()
        } else {
            // Determine if it is a project path.
            if let Err(msg) = is_chainsight_project(None) {
                bail!(format!(r#"{}"#, msg));
            };
            format!("./{}", ARTIFACTS_DIR)
        }
    };

    Ok(path)
}

fn agent(url: &str) -> ic_agent::Agent {
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .with_verify_query_signatures(false)
        .build()
        .unwrap();
    agent
}

async fn get_proxy_from_component(agent: &ic_agent::Agent, principal: &Principal) -> Principal {
    let res = agent
        .update(&principal, "get_proxy")
        .with_arg(Encode!().unwrap())
        .call_and_wait()
        .await
        .unwrap(); // TODO: error handling
    Decode!(res.as_slice(), Principal).unwrap()
}

async fn vault_from_proxy(agent: &ic_agent::Agent, principal: &Principal) -> Principal {
    let res = agent
        .query(&principal, "vault")
        .with_arg(Encode!().unwrap())
        .call()
        .await
        .unwrap(); // TODO: error handling
    Decode!(res.as_slice(), Principal).unwrap()
}

async fn db_from_proxy(agent: &ic_agent::Agent, principal: &Principal) -> Principal {
    let res = agent
        .query(&principal, "db")
        .with_arg(Encode!().unwrap())
        .call()
        .await
        .unwrap(); // TODO: error handling
    Decode!(res.as_slice(), Principal).unwrap()
}

fn canister_id_from_canister_name(
    execution_dir: &Path,
    network: &Network,
    canister_name: &str,
) -> Result<Principal, String> {
    let args_builder = DfxArgsBuilder::new_only_network(network.clone());
    let args = args_builder.generate(vec!["canister", "id", canister_name]);

    let output = Command::new("dfx")
        .current_dir(execution_dir)
        .args(args)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let msg = std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout");
        Ok(Principal::from_text(msg.replace("\n", "")).unwrap())
    } else {
        let msg = std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr");
        Err(msg.to_string())
    }
}

// Get wallet from selected Identity
fn get_wallet(network: &Network) -> Result<Principal, String> {
    let args_builder = DfxArgsBuilder::new_only_network(network.clone());
    let args = args_builder.generate(vec!["identity", "get-wallet"]);

    let output = Command::new("dfx")
        .args(args)
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let msg = std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout");
        Ok(Principal::from_text(msg.replace("\n", "")).unwrap())
    } else {
        let msg = std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr");
        Err(msg.to_string())
    }
}

// Get cycle balance of the selected Identity's cycles wallet
fn get_wallet_balance(network: &Network) -> Result<String, String> {
    let args_builder = DfxArgsBuilder::new_only_network(network.clone());
    let args = args_builder.generate(vec!["wallet", "balance"]);

    let output = Command::new("dfx")
        .args(args)
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let msg = std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout");
        Ok(msg.to_string())
    } else {
        let msg = std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr");
        Err(msg.to_string())
    }
}

fn delete_canister(
    execution_dir: &Path,
    component: String,
    wallet: &str,
    network: &Network,
) -> anyhow::Result<String> {
    let args_builder = DfxArgsBuilder::new(network.clone(), Some(component));
    let args = args_builder.generate(vec!["canister", "delete", "--wallet", wallet, "--yes"]);

    let output = output_by_exec_cmd("dfx", execution_dir, args)?;

    let msg = std::str::from_utf8(&output.stderr).context("failed to parse stderr")?;
    Ok(msg.to_string())
}
