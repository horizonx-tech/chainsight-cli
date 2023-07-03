use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{common::CanisterIdType, relayer::RelayerComponentManifest},
        scripts::common::{generate_command_to_set_task, network_param},
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
    label: &str,
    datasrc_id: &str,
    datasrc_id_type: CanisterIdType,
    dst_address: &str,
    dst_network_id: u32,
    dst_rpc_url: &str,
    network: &Network,
) -> String {
    let target_canister = match datasrc_id_type {
        CanisterIdType::CanisterName => format!("$(dfx canister id {})", datasrc_id),
        CanisterIdType::PrincipalId => datasrc_id.to_string(),
    };

    let ecdsa_key_env = match network {
        Network::IC => "Test", // temp: use for test
        Network::Local => "LocalDevelopment",
    };

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\",
    \"{}\",
    record {{
        url = \"{}\";
        from = null;
        chain_id = {};
        key = variant {{ {} }};
    }})""#,
        network_param(network),
        label,
        target_canister,
        dst_address,
        dst_rpc_url,
        dst_network_id,
        ecdsa_key_env
    )
}

fn script_contents(manifest: &RelayerComponentManifest, network: Network) -> String {
    let script_to_setup = generate_command_to_setup(
        &manifest.metadata.label,
        &manifest.datasource.location.id,
        manifest.datasource.location.args.id_type.unwrap(), // todo: check validation
        &manifest.destination.oracle_address,
        manifest.destination.network_id,
        &manifest.destination.rpc_url,
        &network,
    );
    let script_to_set_task = generate_command_to_set_task(
        &manifest.metadata.label,
        &network,
        manifest.interval,
        10, // temp: fixed value, todo: make it configurable
    );

    format!(
        r#"#!/bin/bash

# setup
{}
# set_task
{}
"#,
        script_to_setup, script_to_set_task
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
