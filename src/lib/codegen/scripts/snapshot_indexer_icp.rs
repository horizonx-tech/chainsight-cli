use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{
            common::CanisterIdType, snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
        },
        scripts::common::{generate_command_to_set_task, init_in_env_task, network_param},
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
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
fn script_contents(manifest: &SnapshotIndexerICPComponentManifest, network: Network) -> String {
    let setup_contents = generate_command_to_setup(
        &manifest.metadata.label,
        &manifest.datasource.location.id,
        manifest.datasource.location.args.id_type.unwrap(), // todo: check validation // todo: fail commands::exec::tests::test_exec for here
        &network,
    );

    let start_timer_contents = generate_command_to_set_task(
        &manifest.metadata.label,
        &network,
        manifest.interval,
        5, // temp: fixed value, todo: make it configurable
    );
    let init_in_env_task = init_in_env_task(&network, &manifest.metadata.label);

    format!(
        r#"#!/bin/bash
# init
{}
# setup
{}
# set_task
{}
"#,
        init_in_env_task, setup_contents, start_timer_contents
    )
}

pub fn generate_scripts(
    manifest: &SnapshotIndexerICPComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerICP,
        "type is not SnapshotIndexerICP"
    );

    Ok(script_contents(manifest, network))
}
