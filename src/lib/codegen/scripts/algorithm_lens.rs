use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::algorithm_lens::AlgorithmLensComponentManifest,
        scripts::common::init_in_env_task,
    },
    types::{ComponentType, Network},
};

fn script_contents(manifest: &AlgorithmLensComponentManifest, network: Network) -> String {
    let init_in_env_task = init_in_env_task(&network, &manifest.metadata.label);

    format!(
        r#"#!/bin/bash
# init
{}
"#,
        init_in_env_task,
    )
}

pub fn generate_scripts(
    manifest: &AlgorithmLensComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmLens,
        "type is not AlgorithmLens"
    );

    Ok(script_contents(manifest, network))
}
