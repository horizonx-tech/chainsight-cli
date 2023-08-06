use anyhow::ensure;

use crate::{
    lib::codegen::{
        components::{
            algorithm_lens::{AlgorithmLensComponentManifest, AlgorithmLensDataSourceLocation},
            common::CanisterIdType,
        },
        scripts::common::{init_in_env_task, network_param},
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
    label: &str,
    locations: Vec<AlgorithmLensDataSourceLocation>,
    network: &Network,
) -> String {
    let target_canisters = locations
        .iter()
        .map(|l| {
            let target_canister = match l.id_type {
                CanisterIdType::CanisterName => format!("$(dfx canister id {})", l.id),
                CanisterIdType::PrincipalId => l.id.to_string(),
            };
            format!("\"{}\"", target_canister)
        })
        .collect::<Vec<_>>()
        .join(",\n\t");

    format!(
        r#"dfx canister {} call {} setup "(
    {}
)""#,
        network_param(network),
        label,
        target_canisters
    )
}
fn script_contents(manifest: &AlgorithmLensComponentManifest, network: Network) -> String {
    let init_in_env_task = init_in_env_task(&network, &manifest.metadata.label);

    let setup_contents = generate_command_to_setup(
        &manifest.metadata.label,
        manifest.datasource.locations.clone(),
        &network,
    );

    format!(
        r#"#!/bin/bash
# init
{}
# setup
{}
"#,
        init_in_env_task, setup_contents
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
