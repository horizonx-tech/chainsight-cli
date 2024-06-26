use anyhow::Context;
use candid::{Decode, Encode, Principal};
use clap::Parser;
use slog::info;

use crate::{
    commands::utils::{get_agent, working_dir},
    lib::{
        environment::EnvironmentImpl,
        utils::{
            component_ids_manager::ComponentIdsManager,
            dfx::{DfxWrapper, DfxWrapperNetwork},
        },
    },
    types::Network,
};

#[derive(Debug, Parser)]
#[command(name = "component-info")]
/// [EXPERIMENTAL] Display component information. IDs and other information, including sidecars, can be checked.
pub struct ComponentInfoOpts {
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

pub async fn exec(env: &EnvironmentImpl, opts: ComponentInfoOpts) -> anyhow::Result<()> {
    // Check if the `dfx` binary is available
    if DfxWrapper::new(DfxWrapperNetwork::IC, None).is_err() {
        anyhow::bail!(
            "The `dfx` binary is required to execute this operation. Please install dfx."
        );
    }

    let log = env.get_logger();
    let ComponentInfoOpts {
        path,
        network,
        port,
        component,
    } = opts;

    info!(log, r#"Start component-info component '{}'..."#, component);

    let working_dir_str = working_dir(path.clone())?;

    let component_id = if let Ok(principal) = Principal::from_text(&component) {
        principal
    } else {
        let comp_id_mgr = ComponentIdsManager::load(
            &match network {
                Network::Local => DfxWrapperNetwork::Local(port),
                Network::IC => DfxWrapperNetwork::IC,
            },
            &working_dir_str,
        )?;
        let id = comp_id_mgr
            .get(&component)
            .context(format!("Failed to get canister id for {}", component))?;
        Principal::from_text(id)?
    };

    let agent = get_agent(&network, port, None).await?;

    info!(log, "  component: {}", component_id.to_text());
    let res = exec_internal(&agent, &component_id).await?;
    info!(log, "  proxy: {}", res.proxy.to_text());
    info!(log, "  vault: {}", res.vault.to_text());
    info!(log, "  db: {}", res.db.to_text());

    Ok(())
}

pub struct ComponentInfo {
    pub proxy: Principal,
    pub vault: Principal,
    pub db: Principal,
}
pub async fn exec_internal(
    agent: &ic_agent::Agent,
    component_id: &Principal,
) -> anyhow::Result<ComponentInfo> {
    let proxy = get_proxy_from_component(agent, component_id).await?;
    let vault = vault_from_proxy(agent, &proxy).await?;
    let db = db_from_proxy(agent, &proxy).await?;
    Ok(ComponentInfo { proxy, vault, db })
}

async fn get_proxy_from_component(
    agent: &ic_agent::Agent,
    principal: &Principal,
) -> anyhow::Result<Principal> {
    let res = agent
        .update(principal, "get_proxy")
        .with_arg(Encode!().unwrap())
        .call_and_wait()
        .await?;
    Ok(Decode!(res.as_slice(), Principal).unwrap())
}

async fn vault_from_proxy(
    agent: &ic_agent::Agent,
    principal: &Principal,
) -> anyhow::Result<Principal> {
    let res = agent
        .query(principal, "vault")
        .with_arg(Encode!().unwrap())
        .call()
        .await?;
    Ok(Decode!(res.as_slice(), Principal).unwrap())
}

async fn db_from_proxy(
    agent: &ic_agent::Agent,
    principal: &Principal,
) -> anyhow::Result<Principal> {
    let res = agent
        .query(principal, "db")
        .with_arg(Encode!().unwrap())
        .call()
        .await?;
    Ok(Decode!(res.as_slice(), Principal).unwrap())
}
