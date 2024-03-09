use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{
            algorithm_indexer::AlgorithmIndexerComponentManifest, common::ComponentManifest,
        },
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
    network: &Network,
    start_from: u64,
    chunk_size: Option<u64>,
) -> String {
    let target_canister = principal_or_resolver_str(datasrc_id, network);
    let chunk_size = if let Some(chunk_size) = chunk_size {
        format!("opt {}", chunk_size)
    } else {
        "null".to_string()
    };

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\",
    record{{
        start_from={};
        chunk_size={};
    }}
    )""#,
        network_param(network),
        id,
        target_canister,
        start_from,
        chunk_size
    )
}

fn script_contents(manifest: &AlgorithmIndexerComponentManifest, network: Network) -> String {
    let id = manifest.id().unwrap();
    let script_to_setup = generate_command_to_setup(
        &id,
        &manifest.datasource.principal,
        &network,
        manifest.datasource.from,
        manifest.datasource.batch_size,
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
    manifest: &AlgorithmIndexerComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmIndexer,
        "type is not AlgorithmIndexer"
    );

    Ok(script_contents(manifest, network))
}
