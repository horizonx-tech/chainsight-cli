use candid::Principal;
use ic_agent::Agent;
use ic_utils::interfaces::WalletCanister;

pub async fn wallet_canister(
    agent_with_wallet_controller: &Agent,
    wallet_canister_id: Principal,
) -> anyhow::Result<WalletCanister> {
    let wallet_canister = WalletCanister::create(agent_with_wallet_controller, wallet_canister_id)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(wallet_canister)
}
