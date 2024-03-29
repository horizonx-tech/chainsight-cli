use std::path::Path;

use anyhow::Context;
use candid::Principal;
use clap::Parser;
use slog::{debug, error, info};

use crate::{
    commands::{
        component_info,
        utils::{
            canister_id_from_canister_name, generate_agent, output_by_exec_cmd, working_dir,
            DfxArgsBuilder,
        },
    },
    lib::environment::EnvironmentImpl,
    types::Network,
};

#[derive(Debug, Parser)]
#[command(name = "delete")]
/// Delete your Chainsight component. This command deletes the component with sidecars and allows you to recover the remaining cycles.
pub struct DeleteOpts {
    /// Specify the path of the project to be deleted.
    /// If not specified, the current directory is targeted.
    #[arg(long, short = 'p')]
    pub path: Option<String>,

    /// Specify the component name or canister id to delete.
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

    let component_id = Principal::from_text(&opts.component).unwrap_or_else(|_| {
        canister_id_from_canister_name(working_dir, &opts.network, &opts.component)
            .expect("failed to get canister id")
    });
    let url = opts.network.to_url(opts.port);
    let agent = generate_agent(&url);
    if opts.network == Network::Local {
        agent.fetch_root_key().await.unwrap();
    }

    info!(log, "Confirm sidecars to delete");
    let res = component_info::exec_internal(&agent, &component_id).await?;
    let component_info::ComponentInfo { proxy, vault, db } = res;
    info!(log, "  proxy: {}", proxy.to_text());
    info!(log, "  vault: {}", vault.to_text());
    info!(log, "  db: {}", db.to_text());

    let wallet = get_wallet(working_dir, &opts.network)
        .expect("failed to get wallet")
        .to_string();
    info!(log, "Wallet to execute deletion: {}", wallet);

    let exec_delete = |label: &str, canister_id: String| -> bool {
        info!(log, "Deleting {} ({})", label, canister_id);
        let res = delete_canister(working_dir, canister_id, &wallet, &opts.network);
        match res {
            Ok(msg) => {
                info!(log, "Deleted {}", label);
                debug!(log, "{}", msg);
                true
            }
            Err(e) => {
                error!(log, "Failed to delete {}: {}", label, e);
                false
            }
        }
    };
    let before_balance = get_wallet_balance(working_dir, &opts.network);
    match before_balance {
        Ok(balance) => info!(log, "Balance before deletion: {}", balance),
        Err(e) => error!(log, "Failed to get balance: {}", e),
    }
    let res_db = exec_delete("db", db.to_text());
    let res_vault = exec_delete("vault", vault.to_text());
    let res_proxy = exec_delete("proxy", proxy.to_text());
    let res_component = exec_delete("component", component_id.to_text());
    let after_balance = get_wallet_balance(working_dir, &opts.network);
    match after_balance {
        Ok(balance) => info!(log, "Balance after deletion: {}", balance),
        Err(e) => error!(log, "Failed to get balance: {}", e),
    }

    let msg_from_result_flag = |res: bool| -> String {
        if res {
            "Removed".to_string()
        } else {
            "Fail to delete".to_string()
        }
    };
    info!(
        log,
        r#"Finish deleting component '{}'.
The results of the deleting are as follows.
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

// Get wallet from selected Identity
fn get_wallet(execution_dir: &Path, network: &Network) -> Result<Principal, String> {
    let args_builder = DfxArgsBuilder::new_only_network(network.clone());
    let args = args_builder.generate(vec!["identity", "get-wallet"]);

    let output = output_by_exec_cmd("dfx", execution_dir, args).expect("failed to execute process");
    if output.status.success() {
        let msg = std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout");
        Ok(Principal::from_text(msg.replace('\n', "")).unwrap())
    } else {
        let msg = std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr");
        Err(msg.to_string())
    }
}

// Get cycle balance of the selected Identity's cycles wallet
fn get_wallet_balance(execution_dir: &Path, network: &Network) -> Result<String, String> {
    let args_builder = DfxArgsBuilder::new_only_network(network.clone());
    let args = args_builder.generate(vec!["wallet", "balance"]);

    let output = output_by_exec_cmd("dfx", execution_dir, args).expect("failed to execute process");
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
