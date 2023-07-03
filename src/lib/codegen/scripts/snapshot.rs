use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{
            common::{CanisterIdType, DatasourceType},
            snapshot::SnapshotComponentManifest,
        },
        scripts::common::{generate_command_to_set_task, network_param},
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup_for_canister(
    label: &str,
    datasrc_id: &str,
    datasrc_id_type: CanisterIdType,
    network: &Network,
) -> String {
    let target_canister = match datasrc_id_type {
        CanisterIdType::CanisterName => format!("$(dfx canister id {})", datasrc_id),
        CanisterIdType::PrincipalId => datasrc_id.to_string(),
    };

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\"
)""#,
        network_param(network),
        label,
        target_canister
    )
}
fn script_contents_for_canister(manifest: &SnapshotComponentManifest, network: Network) -> String {
    let setup_contents = generate_command_to_setup_for_canister(
        &manifest.label,
        &manifest.datasource.location.id,
        manifest.datasource.location.args.id_type.unwrap(), // todo: check validation
        &network,
    );

    let start_timer_contents = generate_command_to_set_task(
        &manifest.label,
        &network,
        manifest.interval,
        5, // temp: fixed value, todo: make it configurable
    );

    format!(
        r#"#!/bin/bash

# setup
{}
# set_task
{}
"#,
        setup_contents, start_timer_contents
    )
}

fn generate_command_to_setup_for_contract(
    label: &str,
    datasrc_id: &str,
    datasrc_network_id: u32,
    datasrc_rpc_url: &str,
    network: &Network,
) -> String {
    let ecdsa_key_id = "LocalDevelopment"; // temp: because not to use (do not sign)

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\",
    record {{
        url = \"{}\";
        from = null;
        chain_id = {};
        key = variant {{ {} }};
    }}
)""#,
        network_param(network),
        label,
        datasrc_id,
        datasrc_rpc_url,
        datasrc_network_id,
        ecdsa_key_id
    )
}
fn script_contents_for_contract(manifest: &SnapshotComponentManifest, network: Network) -> String {
    let datasrc_location_args = manifest.datasource.location.args.clone();

    let setup_contents = generate_command_to_setup_for_contract(
        &manifest.label,
        &manifest.datasource.location.id,
        datasrc_location_args.network_id.unwrap(), // todo: check validation
        &datasrc_location_args.rpc_url.unwrap(),   // todo: check validation
        &network,
    );

    let start_timer_contents = generate_command_to_set_task(
        &manifest.label,
        &network,
        manifest.interval,
        0, // temp: fixed value, todo: make it configurable
    );

    format!(
        r#"#!/bin/bash

# setup
{}
# set_task
{}
"#,
        setup_contents, start_timer_contents
    )
}

pub fn generate_scripts(
    manifest: &SnapshotComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.type_ == ComponentType::Snapshot,
        "type is not Snapshot"
    );

    let contents = match manifest.datasource.type_ {
        DatasourceType::Canister => script_contents_for_canister(manifest, network),
        DatasourceType::Contract => script_contents_for_contract(manifest, network),
    };

    Ok(contents)
}
