use candid::Principal;
use chainsight_cdk::initializer::CycleManagements;

use crate::types::Network;

pub fn network_param(network: &Network) -> &str {
    match network {
        Network::IC => "--network ic",
        Network::Local => "",
    }
}

pub fn generate_command_to_set_task(
    id: &str,
    network: &Network,
    interval: u32,
    delay: u32,
) -> String {
    format!(
        r#"dfx canister {} call {} set_task '({}, {})'"#,
        network_param(network),
        id,
        interval,
        delay
    )
}

pub fn init_in_env_task(network: &Network, id: &str, cycles: &Option<CycleManagements>) -> String {
    let cycles = cycles.unwrap_or_default();
    let total_cycles = cycles.vault_intial_supply
        + cycles.indexer.initial_value
        + cycles.db.initial_value
        + cycles.proxy.initial_value;
    format!(
        r#"dfx canister {} call {} init_in '(variant {{ "{}" }}, record {{
                refueling_interval = {}: nat64;
                vault_intial_supply = {}: nat;
                indexer = record {{ 
                    initial_value = {}: nat;
                    refueling_value = {}: nat;
                    refueling_threshold = {}: nat;
                }};
                db = record {{ 
                    initial_value = {}: nat;
                    refueling_value = {}: nat;
                    refueling_threshold = {}: nat;
                }};
                proxy = record {{ 
                    initial_value = {}: nat;
                    refueling_value = {}: nat;
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
        cycles.indexer.initial_value,
        cycles.indexer.refueling_value,
        cycles.indexer.refueling_threshold,
        cycles.db.initial_value,
        cycles.db.refueling_value,
        cycles.db.refueling_threshold,
        cycles.proxy.initial_value,
        cycles.proxy.refueling_value,
        cycles.proxy.refueling_threshold,
        total_cycles,
        network_param(network),
    )
}

pub fn principal_or_resolver_str(str: &str, network: &Network) -> String {
    match Principal::from_text(str) {
        Ok(p) => p.to_string(),
        Err(_) => format!("$(dfx canister {} id {})", network_param(network), str),
    }
}
