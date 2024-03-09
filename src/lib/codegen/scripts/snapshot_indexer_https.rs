use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{
            common::ComponentManifest,
            snapshot_indexer_https::SnapshotIndexerHTTPSComponentManifest,
        },
        scripts::common::{generate_command_to_set_task, init_in_env_task},
    },
    types::{ComponentType, Network},
};

fn script_contents(manifest: &SnapshotIndexerHTTPSComponentManifest, network: Network) -> String {
    let id = manifest.id().unwrap();
    let script_to_set_task = generate_command_to_set_task(
        &id,
        &network,
        manifest.timer_settings.interval_sec,
        manifest.timer_settings.delay_sec.unwrap_or(0),
    );
    let init_in_env_task = init_in_env_task(&network, &id, &manifest.cycle_managements());

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
    manifest: &SnapshotIndexerHTTPSComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerHTTPS,
        "type is not SnapshotIndexerHTTPS"
    );

    Ok(script_contents(manifest, network))
}
