use crate::types::Network;

pub fn network_param(network: &Network) -> &str {
    match network {
        Network::IC => "--network ic",
        Network::Local => "",
    }
}

pub fn generate_command_to_set_task(
    label: &str,
    network: &Network,
    interval: u32,
    delay: u32,
) -> String {
    format!(
        r#"dfx canister {} call {} set_task '({}, {})'"#,
        network_param(network),
        label,
        interval,
        delay
    )
}

pub fn init_in_env_task(network: &Network, label: &str) -> String {
    format!(
        r#"dfx canister {} call {} init_in '(variant {{ "{}" }})'"#,
        network_param(network),
        label,
        match network {
            Network::Local => "LocalDevelopment",
            Network::IC => "Production",
        }
    )
}
