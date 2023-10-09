use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{
            common::ComponentManifest, snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
        },
        scripts::common::{generate_command_to_set_task, init_in_env_task, network_param},
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
    id: &str,
    datasrc_id: &str,
    datasrc_network_id: u32,
    datasrc_rpc_url: &str,
    network: &Network,
) -> String {
    let ecdsa_key_id = match network {
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
    }}
)""#,
        network_param(network),
        id,
        datasrc_id,
        datasrc_rpc_url,
        datasrc_network_id,
        ecdsa_key_id
    )
}
fn script_contents(manifest: &SnapshotIndexerEVMComponentManifest, network: Network) -> String {
    let id = manifest.id().unwrap();
    let datasrc_location_args = manifest.datasource.location.args.clone();

    let setup_contents = generate_command_to_setup(
        &id,
        &manifest.datasource.location.id,
        datasrc_location_args.network_id.unwrap(), // todo: check validation
        &datasrc_location_args.rpc_url.unwrap(),   // todo: check validation
        &network,
    );

    let start_timer_contents = generate_command_to_set_task(
        &id,
        &network,
        manifest.interval,
        0, // temp: fixed value, todo: make it configurable
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
    manifest: &SnapshotIndexerEVMComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerEVM,
        "type is not SnapshotIndexerEVM"
    );

    Ok(script_contents(manifest, network))
}
