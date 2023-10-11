use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{
            common::ComponentManifest, snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
        },
        scripts::common::{
            generate_command_to_set_task, init_in_env_task, network_param,
            principal_or_resolver_str,
        },
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(id: &str, datasrc_id: &str, network: &Network) -> String {
    let target_canister = principal_or_resolver_str(datasrc_id, network);

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\"
)""#,
        network_param(network),
        id,
        target_canister
    )
}
fn script_contents(manifest: &SnapshotIndexerICPComponentManifest, network: Network) -> String {
    let id = manifest.id().unwrap();

    let setup_contents = generate_command_to_setup(&id, &manifest.datasource.location.id, &network);

    let start_timer_contents = generate_command_to_set_task(
        &id,
        &network,
        manifest.interval,
        5, // temp: fixed value, todo: make it configurable
    );
    let init_in_env_task = init_in_env_task(&network, &id);

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
