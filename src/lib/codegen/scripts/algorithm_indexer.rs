use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::algorithm_indexer::AlgorithmIndexerComponentManifest,
        scripts::common::{generate_command_to_set_task, init_in_env_task, network_param},
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
    label: &str,
    datasrc_id: &str,
    network: &Network,
    start_from: u64,
) -> String {
    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\",
    record{{
        start_from={}
    }}
    )""#,
        network_param(network),
        label,
        datasrc_id,
        start_from
    )
}

fn script_contents(manifest: &AlgorithmIndexerComponentManifest, network: Network) -> String {
    let script_to_setup = generate_command_to_setup(
        &manifest.metadata.label,
        &manifest.datasource.printipal,
        &network,
        manifest.datasource.from,
    );
    let script_to_set_task = generate_command_to_set_task(
        &manifest.metadata.label,
        &network,
        manifest.interval,
        10, // temp: fixed value, todo: make it configurable
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
        init_in_env_task, script_to_setup, script_to_set_task
    )
}

pub fn generate_scripts(
    manifest: &AlgorithmIndexerComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmIndexer,
        "type is not AlgorithmIndexer"
    );

    Ok(script_contents(manifest, network))
}
