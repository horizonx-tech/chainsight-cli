use anyhow::Ok;
use candid::Principal;
use ic_agent::{Agent, Identity};
use ic_utils::{
    interfaces::{
        management_canister::builders::{CanisterInstall, CanisterSettings, InstallMode},
        ManagementCanister, WalletCanister,
    },
    Argument, Canister,
};

use crate::{
    lib::utils::dfx::{DfxWrapper, DfxWrapperNetwork},
    types::Network,
};

use super::types::UpdateSettingsArgs;

// from: dfinity/sdk/src/dfx/src/lib/operations/canister/create_canister.rs
const CANISTER_CREATE_FEE: u128 = 100_000_000_000_u128;
const CANISTER_INITIAL_CYCLE_BALANCE: u128 = 3_000_000_000_000_u128;

pub async fn canister_create(
    identity: Box<dyn Identity>,
    wallet_principal: &Option<Principal>,
    network: &Network,
    port: Option<u16>,
    cycles: Option<u128>,
) -> anyhow::Result<Principal> {
    let agent = get_agent(identity, network, port).await?;

    let canister_id = if network == &Network::Local && wallet_principal.is_none() {
        create_canister_by_management_canister(&agent, cycles).await?
    } else {
        let wallet_canister = wallet_canister(wallet_principal.unwrap(), &agent).await?;
        let cycles = cycles.unwrap_or(CANISTER_CREATE_FEE + CANISTER_INITIAL_CYCLE_BALANCE);
        let res = wallet_canister
            .wallet_create_canister(cycles, None, None, None, None)
            .await?;
        res.canister_id
    };

    Ok(canister_id)
}

// for local
async fn create_canister_by_management_canister(
    agent: &Agent,
    cycles: Option<u128>,
) -> anyhow::Result<Principal> {
    let mgr_canister = ManagementCanister::create(agent);
    let builder = mgr_canister
        .create_canister()
        .as_provisional_create_with_amount(cycles);
    let res = builder.call_and_wait().await?;
    Ok(res.0)
}

pub async fn canister_install(
    wasm_path: &str,
    deploy_dest_id: Principal,
    identity: Box<dyn Identity>,
    wallet_principal: &Option<Principal>,
    network: &Network,
    port: Option<u16>,
) -> anyhow::Result<()> {
    let agent = get_agent(identity, network, port).await?;
    let wasm_data = std::fs::read(wasm_path)?;

    if network == &Network::Local && wallet_principal.is_none() {
        install_canister_by_management_canister(&agent, &deploy_dest_id, &wasm_data).await?;
    } else {
        let wallet_canister = wallet_canister(wallet_principal.unwrap(), &agent).await?;
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

pub async fn canister_update_settings(
    deploy_dest_id: Principal,
    controllers_to_add: Vec<Principal>,
    identity: Box<dyn Identity>,
    wallet_principal: &Option<Principal>,
    network: &Network,
    port: Option<u16>,
) -> anyhow::Result<()> {
    let agent = get_agent(identity, network, port).await?;

    if network == &Network::Local && wallet_principal.is_none() {
        update_settings_by_management_canister(&agent, &deploy_dest_id, controllers_to_add).await?;
    } else {
        let wallet_canister = wallet_canister(wallet_principal.unwrap(), &agent).await?;
        wallet_canister
            .call(
                Principal::management_canister(),
                "update_settings",
                Argument::from_candid((UpdateSettingsArgs {
                    canister_id: deploy_dest_id,
                    settings: CanisterSettings {
                        controllers: Some(controllers_to_add),
                        compute_allocation: None,
                        memory_allocation: None,
                        freezing_threshold: None,
                        reserved_cycles_limit: None,
                    },
                },)),
                0,
            )
            .call_and_wait()
            .await?;
    }

    Ok(())
}

async fn update_settings_by_management_canister(
    agent: &Agent,
    canister_id: &Principal,
    controllers: Vec<Principal>,
) -> anyhow::Result<()> {
    let mgr_canister = ManagementCanister::create(&agent);
    let mut builder = mgr_canister.update_settings(canister_id);
    for controller in controllers {
        builder = builder.with_controller(controller);
    }
    builder.call_and_wait().await?;

    Ok(())
}

// utils
pub async fn get_agent(
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

pub async fn wallet_canister(id: Principal, agent: &Agent) -> anyhow::Result<WalletCanister> {
    let canister = Canister::builder()
        .with_agent(agent)
        .with_canister_id(id)
        .build()?;
    let wallet_canister = WalletCanister::from_canister(canister).await?;
    Ok(wallet_canister)
}

pub async fn get_wallet_principal_from_local_context(
    network: &Network,
    port: Option<u16>,
) -> anyhow::Result<Principal> {
    let dfx = DfxWrapper::new(
        match network {
            Network::Local => DfxWrapperNetwork::Local(port),
            _ => DfxWrapperNetwork::IC,
        },
        None,
    )
    .map_err(|e| anyhow::anyhow!(e))?
    .0;
    // todo: support direct loading of wallets.json
    let id = Principal::from_text(dfx.identity_get_wallet().map_err(|e| anyhow::anyhow!(e))?)?;
    Ok(id)
}
