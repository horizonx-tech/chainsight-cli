use candid::Principal;
use chainsight_cdk::initializer::CycleManagements;

use crate::{lib::codegen::components::common::TimerSettings, types::Network};

pub fn network_param(network: &Network) -> &str {
    match network {
        Network::IC => "--network ic",
        Network::Local => "",
    }
}

pub fn generate_command_to_set_task(id: &str, network: &Network, timer: &TimerSettings) -> String {
    let delay = timer.delay_sec.unwrap_or(0);
    let is_round_start_timing = timer.is_round_start_timing.unwrap_or(false);
    format!(
        r#"dfx canister {} call {} set_task_with_rounded '({}, {}, {})'"#,
        network_param(network),
        id,
        timer.interval_sec,
        delay,
        is_round_start_timing
    )
}

pub fn init_in_env_task(network: &Network, id: &str, cycles: &CycleManagements) -> String {
    format!(
        r#"dfx canister {} call {} init_in '(variant {{ "{}" }}, record {{
                refueling_interval = {}: nat64;
                vault_intial_supply = {}: nat;
                indexer = record {{ 
                    initial_supply = {}: nat;
                    refueling_amount = {}: nat;
                    refueling_threshold = {}: nat;
                }};
                db = record {{ 
                    initial_supply = {}: nat;
                    refueling_amount = {}: nat;
                    refueling_threshold = {}: nat;
                }};
                proxy = record {{ 
                    initial_supply = {}: nat;
                    refueling_amount = {}: nat;
                    refueling_threshold = {}: nat;
                }};
        }})' --with-cycles {} --wallet $(dfx identity get-wallet {})"#,
        network_param(network),
        id,
        match network {
            Network::Local => "LocalDevelopment",
            Network::IC => "Production",
        },
        cycles.refueling_interval,
        cycles.vault_intial_supply,
        cycles.indexer.initial_supply,
        cycles.indexer.refueling_amount,
        cycles.indexer.refueling_threshold,
        cycles.db.initial_supply,
        cycles.db.refueling_amount,
        cycles.db.refueling_threshold,
        cycles.proxy.initial_supply,
        cycles.proxy.refueling_amount,
        cycles.proxy.refueling_threshold,
        cycles.initial_supply(),
        network_param(network),
    )
}

pub fn principal_or_resolver_str(str: &str, network: &Network) -> String {
    match Principal::from_text(str) {
        Ok(p) => p.to_string(),
        Err(_) => format!("$(dfx canister {} id {})", network_param(network), str),
    }
}
