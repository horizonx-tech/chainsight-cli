use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::snapshot_indexer_https::SnapshotJsonRPCComponentManifest,
        scripts::common::{generate_command_to_set_task, init_in_env_task},
    },
    types::{ComponentType, Network},
};

fn script_contents(manifest: &SnapshotJsonRPCComponentManifest, network: Network) -> String {
    let script_to_set_task =
        generate_command_to_set_task(&manifest.metadata.label, &network, manifest.interval, 10);
    let init_in_env_task = init_in_env_task(&network, &manifest.metadata.label);

    format!(
        r#"#!/bin/bash
# init
{}
# set_task
{}
"#,
        init_in_env_task, script_to_set_task
    )
}

pub fn generate_scripts(
    manifest: &SnapshotJsonRPCComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerHTTPS,
        "type is not SnapshotJsonRPC"
    );

    Ok(script_contents(manifest, network))
}
