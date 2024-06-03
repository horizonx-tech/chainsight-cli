use candid::Principal;
use ic_agent::{Agent, Identity};
use ic_utils::{
    interfaces::{ManagementCanister, WalletCanister},
    Canister,
};

use crate::{
    lib::utils::dfx::{DfxWrapper, DfxWrapperNetwork},
    types::Network,
};

// from: dfinity/sdk/src/dfx/src/lib/operations/canister/create_canister.rs
const CANISTER_CREATE_FEE: u128 = 100_000_000_000_u128;
const CANISTER_INITIAL_CYCLE_BALANCE: u128 = 3_000_000_000_000_u128;

pub async fn canister_create(
    artifacts_path_str: &str,
    identity: Box<dyn Identity>,
    network: &Network,
    port: Option<u16>,
    component_id: Option<String>,
) -> anyhow::Result<Principal> {
    let agent = Agent::builder()
        .with_url(network.to_url(port))
        .with_identity(identity)
        .build()?;
    if network == &Network::Local {
        agent.fetch_root_key().await?;
    }

    // note: get wallet id
    let dfx = DfxWrapper::new(
        match network {
            Network::Local => DfxWrapperNetwork::Local(port),
            _ => DfxWrapperNetwork::IC,
        },
        None,
    );
    let wallet_principal =
        Principal::from_text(dfx.identity_get_wallet().map_err(|e| anyhow::anyhow!(e))?)?;

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

async fn wallet_canister(id: Principal, agent: &Agent) -> anyhow::Result<WalletCanister> {
    let canister = Canister::builder()
        .with_agent(agent)
        .with_canister_id(id)
        .build()?;
    let wallet_canister = WalletCanister::from_canister(canister).await?;
    Ok(wallet_canister)
}
