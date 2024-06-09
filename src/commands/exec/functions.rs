use candid::{Encode, Principal};
use ic_utils::{interfaces::WalletCanister, Argument};

use crate::lib::codegen::components::common::TimerSettings;

pub async fn call_init_in(wallet: &WalletCanister<'_>, target: Principal) -> anyhow::Result<()> {
    let (cycles_managements, total_initial_supply) = default_cycle_managements();
    let raw_args = Encode!(
        &chainsight_cdk::core::Env::LocalDevelopment,
        &cycles_managements
    )?;
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

fn default_cycle_managements() -> (chainsight_cdk::initializer::CycleManagements, u128) {
    let datum = chainsight_cdk::initializer::CycleManagements {
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
    };
    let total_initial_supply = datum.vault_intial_supply
        + datum.indexer.initial_supply
        + datum.db.initial_supply
        + datum.proxy.initial_supply;
    (datum, total_initial_supply)
}

pub async fn call_setup(
    wallet: &WalletCanister<'_>,
    target: Principal,
    raw_args: Vec<u8>, // note: because of the different argument formats depending on the component
) -> anyhow::Result<()> {
    wallet_call128(wallet, target, "setup".to_string(), raw_args, None).await?;
    Ok(())
}

pub async fn call_set_task(
    wallet: &WalletCanister<'_>,
    target: Principal,
    args: &TimerSettings,
) -> anyhow::Result<()> {
    let delay = args.delay_sec.unwrap_or(0);
    let is_round_start_timing = args.is_round_start_timing.unwrap_or(false);
    let raw_args = Encode!(&args.interval_sec, &delay, &is_round_start_timing)?;
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
