use candid::{Encode, Principal};
use ic_utils::{interfaces::WalletCanister, Argument};

pub async fn call_init_in(wallet: &WalletCanister<'_>, target: Principal) -> anyhow::Result<()> {
    let raw_args = Encode!(
        &chainsight_cdk::core::Env::LocalDevelopment,
        &chainsight_cdk::initializer::CycleManagements {
            refueling_interval: 86400,
            vault_intial_supply: 500_000_000_000u128,
            indexer: chainsight_cdk::initializer::CycleManagement {
                initial_supply: 0u128,
                refueling_amount: 3_000_000_000_000u128,
                refueling_threshold: 1_500_000_000_000u128,
            },
            db: chainsight_cdk::initializer::CycleManagement {
                initial_supply: 1_000_000_000_000u128,
                refueling_amount: 1_000_000_000_000u128,
                refueling_threshold: 500_000_000_000u128,
            },
            proxy: chainsight_cdk::initializer::CycleManagement {
                initial_supply: 100_000_000_000u128,
                refueling_amount: 100_000_000_000u128,
                refueling_threshold: 50_000_000_000u128,
            },
        }
    )?;
    let total_initial_supply = 1_600_000_000_000u128; // todo
    wallet_call128(
        wallet,
        target,
        "init_in".to_string(),
        raw_args,
        Some(total_initial_supply),
    )
    .await?;

    Ok(())
}

pub struct SetTaskArgs {
    pub task_interval_secs: u32,
    pub delay_secs: u32,
    pub is_rounded_start_time: bool,
}
pub async fn call_set_task(
    wallet: &WalletCanister<'_>,
    target: Principal,
    args: SetTaskArgs,
) -> anyhow::Result<()> {
    let raw_args = Encode!(
        &args.task_interval_secs,
        &args.delay_secs,
        &args.is_rounded_start_time
    )?;
    wallet_call128(wallet, target, "set_task".to_string(), raw_args, None).await?;
    Ok(())
}

async fn wallet_call128(
    wallet: &WalletCanister<'_>,
    target: Principal,
    method_name: String,
    raw_args: Vec<u8>,
    with_cycles: Option<u128>,
) -> anyhow::Result<()> {
    let argument = Argument::from_raw(raw_args);
    wallet
        .call128(
            target,
            method_name,
            argument,
            with_cycles.unwrap_or_default(),
        )
        .call_and_wait()
        .await?;
    Ok(())
}
