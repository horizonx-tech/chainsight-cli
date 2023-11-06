use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{common::ComponentManifest, event_indexer::EventIndexerComponentManifest},
        scripts::common::{generate_command_to_set_task, init_in_env_task, network_param},
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
    id: &str,
    datasrc_id: &str,
    network: &Network,
    rpc_url: &str,
    chain_id: u64,
    start_from: u64,
) -> String {
    let env = match network {
        Network::Local => "LocalDevelopment",
        Network::IC => "Production",
    };

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\",
    record{{
        env=variant{{\"{}\"}};
        url=\"{}\";
        chain_id={}        
    }},
    record{{
        start_from={}
    }}
    )""#,
        network_param(network),
        id,
        datasrc_id,
        env,
        rpc_url,
        chain_id,
        start_from
    )
}

fn script_contents(manifest: &EventIndexerComponentManifest, network: Network) -> String {
    let id = manifest.id().unwrap();
    let script_to_setup = generate_command_to_setup(
        &id,
        &manifest.datasource.id,
        &network,
        &manifest.datasource.network.rpc_url,
        manifest.datasource.network.chain_id,
        manifest.datasource.from,
    );
    let script_to_set_task = generate_command_to_set_task(
        &id,
        &network,
        manifest.interval,
        10, // temp: fixed value, todo: make it configurable
    );
    let init_in_env_task = init_in_env_task(&network, &id, &manifest.cycles);

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
    manifest: &EventIndexerComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::EventIndexer,
        "type is not EventIndexer"
    );

    Ok(script_contents(manifest, network))
}
