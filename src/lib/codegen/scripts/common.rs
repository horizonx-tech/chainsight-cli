use candid::Principal;

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

pub fn init_in_env_task(network: &Network, id: &str) -> String {
    format!(
        r#"dfx canister {} call {} init_in '(variant {{ "{}" }})'"#,
        network_param(network),
        id,
        match network {
            Network::Local => "LocalDevelopment",
            Network::IC => "Production",
        }
    )
}

pub fn principal_or_resolver_str(str: &str) -> String {
    match Principal::from_text(str) {
        Ok(p) => p.to_string(),
        Err(_) => format!("$(dfx canister id {})", str),
    }
}
