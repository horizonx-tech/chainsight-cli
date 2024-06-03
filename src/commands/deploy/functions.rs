use anyhow::Ok;
use candid::Principal;
use ic_agent::{Agent, Identity};
use ic_utils::{
    interfaces::{
        management_canister::builders::{CanisterInstall, InstallMode},
        ManagementCanister, WalletCanister,
    },
    Argument, Canister,
};

use crate::{
    lib::utils::dfx::{DfxWrapper, DfxWrapperNetwork},
    types::Network,
};

// from: dfinity/sdk/src/dfx/src/lib/operations/canister/create_canister.rs
const CANISTER_CREATE_FEE: u128 = 100_000_000_000_u128;
const CANISTER_INITIAL_CYCLE_BALANCE: u128 = 3_000_000_000_000_u128;

pub async fn canister_create(
    identity: Box<dyn Identity>,
    network: &Network,
    port: Option<u16>,
) -> anyhow::Result<Principal> {
    let agent = get_agent(identity, network, port).await?;
    let wallet_principal = get_wallet_principal_from_local_context(network, port).await?;

    // todo: support from wallet in local
    let canister_id = if network == &Network::Local {
        create_canister_by_management_canister(&agent).await?
    } else {
        let wallet_canister = wallet_canister(wallet_principal, &agent).await?;
        let res = wallet_canister
            .wallet_create_canister(
                CANISTER_CREATE_FEE + CANISTER_INITIAL_CYCLE_BALANCE,
                None,
                None,
                None,
                None,
            )
            .await?;
        res.canister_id
    };

    Ok(canister_id)
}

// for local
async fn create_canister_by_management_canister(agent: &Agent) -> anyhow::Result<Principal> {
    let mgr_canister = ManagementCanister::create(agent);
    let builder = mgr_canister
        .create_canister()
        .as_provisional_create_with_amount(None);
    let res = builder.call_and_wait().await?;
    Ok(res.0)
}

pub async fn canister_install(
    wasm_path: &str,
    deploy_dest_id: Principal,
    identity: Box<dyn Identity>,
    network: &Network,
    port: Option<u16>,
) -> anyhow::Result<()> {
    let agent = get_agent(identity, network, port).await?;
    let wasm_data = std::fs::read(wasm_path)?;
    let wallet_principal = get_wallet_principal_from_local_context(network, port).await?;

    if network == &Network::Local {
        install_canister_by_management_canister(&agent, &deploy_dest_id, &wasm_data).await?;
    } else {
        let wallet_canister = wallet_canister(wallet_principal, &agent).await?;
        let install_args = CanisterInstall {
            mode: InstallMode::Install,
            canister_id: deploy_dest_id,
            wasm_module: wasm_data,
            arg: Vec::new(),
        };
        wallet_canister
            .call(
                Principal::management_canister(),
                "install_code",
                Argument::from_candid((install_args,)),
                0,
            )
            .call_and_wait()
            .await?;
    }

    Ok(())
}

async fn install_canister_by_management_canister(
    agent: &Agent,
    canister_id: &Principal,
    wasm_module: &[u8],
) -> anyhow::Result<()> {
    let mgr_canister = ManagementCanister::create(agent);
    let builder = mgr_canister.install(canister_id, wasm_module);
    builder.call_and_wait().await?;
    Ok(())
}

// utils
async fn get_agent(
    identity: Box<dyn Identity>,
    network: &Network,
    port: Option<u16>,
) -> anyhow::Result<Agent> {
    let agent = Agent::builder()
        .with_url(network.to_url(port))
        .with_identity(identity)
        .build()?;
    if network == &Network::Local {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}

async fn wallet_canister(id: Principal, agent: &Agent) -> anyhow::Result<WalletCanister> {
    let canister = Canister::builder()
        .with_agent(agent)
        .with_canister_id(id)
        .build()?;
    let wallet_canister = WalletCanister::from_canister(canister).await?;
    Ok(wallet_canister)
}

async fn get_wallet_principal_from_local_context(
    network: &Network,
    port: Option<u16>,
) -> anyhow::Result<Principal> {
    let dfx = DfxWrapper::new(
        match network {
            Network::Local => DfxWrapperNetwork::Local(port),
            _ => DfxWrapperNetwork::IC,
        },
        None,
    );
    let id = Principal::from_text(dfx.identity_get_wallet().map_err(|e| anyhow::anyhow!(e))?)?;
    Ok(id)
}
