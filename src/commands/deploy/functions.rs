use anyhow::Ok;
use candid::Principal;
use ic_agent::{Agent, Identity};
use ic_utils::{
    interfaces::{
        management_canister::builders::{CanisterInstall, CanisterSettings, InstallMode},
        ManagementCanister,
    },
    Argument,
};

use crate::{commands::utils::get_agent, lib::utils::identity::wallet_canister, types::Network};

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
    let agent = get_agent(network, port, Some(identity)).await?;

    let canister_id = if network == &Network::Local && wallet_principal.is_none() {
        create_canister_by_management_canister(&agent, cycles).await?
    } else {
        if wallet_principal.is_none() {
            return Err(anyhow::anyhow!(
                "wallet_principal is required for deployment to IC network."
            ));
        }
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
    let agent = get_agent(network, port, Some(identity)).await?;
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
    let agent = get_agent(network, port, Some(identity)).await?;

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
