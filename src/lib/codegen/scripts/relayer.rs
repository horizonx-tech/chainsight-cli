use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{common::ComponentManifest, relayer::RelayerComponentManifest},
        scripts::common::{
            generate_command_to_set_task, init_in_env_task, network_param,
            principal_or_resolver_str,
        },
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
    id: &str,
    datasrc_id: &str,
    lens_targets: &[String],
    dst_address: &str,
    dst_network_id: u32,
    dst_rpc_url: &str,
    network: &Network,
) -> String {
    let target_canister = principal_or_resolver_str(datasrc_id, network);
    let lens_target_canisters = lens_targets
        .iter()
        .map(|t| principal_or_resolver_str(t, network))
        .collect::<Vec<String>>();

    let lens_targets_arg = if lens_target_canisters.is_empty() {
        "".to_string()
    } else {
        format!(
            r#"vec {{ \"{}\" }},"#,
            lens_target_canisters.join(r#"\"; \""#)
        )
    };

    let ecdsa_key_env = match network {
        Network::IC => "Production",
        Network::Local => "LocalDevelopment",
    };

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\",
    record {{
        url = \"{}\";
        from = null;
        chain_id = {};
        env = variant {{ {} }};
    }},
    \"{}\",
    {}
)""#,
        network_param(network),
        id,
        dst_address,
        dst_rpc_url,
        dst_network_id,
        ecdsa_key_env,
        target_canister,
        lens_targets_arg,
    )
}

fn script_contents(manifest: &RelayerComponentManifest, network: Network) -> String {
    let id = manifest.id().unwrap();
    let script_to_setup = generate_command_to_setup(
        &id,
        &manifest.datasource.location.id,
        &manifest.lens_targets.clone().map_or(vec![], |v| {
            v.identifiers
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        }),
        &manifest.destination.oracle_address,
        manifest.destination.network_id,
        &manifest.destination.rpc_url,
        &network,
    );
    let script_to_set_task = generate_command_to_set_task(&id, &network, &manifest.timer_settings);
    let init_in_env_task = init_in_env_task(&network, &id, &manifest.cycle_managements());

    format!(
        r#"#!/bin/bash
# init
{}
# setup
{}
# set_task
{}
"#,
        init_in_env_task, script_to_setup, script_to_set_task
    )
}

pub fn generate_scripts(
    manifest: &RelayerComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Relayer,
        "type is not Relayer"
    );

    Ok(script_contents(manifest, network))
}
