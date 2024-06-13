use anyhow::{ensure, Context};
use candid::{Encode, Principal};

use crate::{
    lib::{
        codegen::{
            components::{
                common::ComponentManifest,
                snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
            },
            scripts::common::{
                generate_command_to_set_task, init_in_env_task, network_param,
                principal_or_resolver_str,
            },
        },
        utils::component_ids_manager::ComponentIdsManager,
    },
    types::{ComponentType, Network},
};

fn generate_command_to_setup(
    id: &str,
    datasrc_id: &str,
    lens_targets: &[String],
    network: &Network,
) -> String {
    let target_canister = principal_or_resolver_str(datasrc_id, network);

    let lens_target_canisters = lens_targets
        .iter()
        .map(|t| principal_or_resolver_str(t, network))
        .collect::<Vec<String>>();

    let lens_targets_arg = if lens_target_canisters.is_empty() {
        "".to_string()
    } else {
        format!(
            r#"vec {{ \"{}\" }},"#,
            lens_target_canisters.join(r#"\"; \""#)
        )
    };

    format!(
        r#"dfx canister {} call {} setup "(
    \"{}\",
    {}
)""#,
        network_param(network),
        id,
        target_canister,
        lens_targets_arg
    )
}
fn script_contents(manifest: &SnapshotIndexerICPComponentManifest, network: Network) -> String {
    let id = manifest.id().unwrap();

    let setup_contents = generate_command_to_setup(
        &id,
        &manifest.datasource.location.id,
        &manifest.lens_targets.clone().map_or(vec![], |v| {
            v.identifiers
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        }),
        &network,
    );

    let start_timer_contents =
        generate_command_to_set_task(&id, &network, &manifest.timer_settings);
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

pub fn generate_component_setup_args(
    manifest: &SnapshotIndexerICPComponentManifest,
    comp_id_mgr: &ComponentIdsManager,
) -> anyhow::Result<Vec<u8>> {
    let target_name_or_id = manifest.datasource.location.id.clone();
    let resolver_name_or_id = |name_or_id: &str| -> String {
        if Principal::from_text(name_or_id).is_ok() {
            name_or_id.to_string()
        } else {
            comp_id_mgr
                .get(name_or_id)
                .context(format!("Failed to get canister id for {}", name_or_id))
                .unwrap()
        }
    };
    let target_canister = resolver_name_or_id(&target_name_or_id);
    let lens_target_name_or_id = manifest
        .lens_targets
        .clone()
        .map(|v| {
            v.identifiers
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    let args = if lens_target_name_or_id.is_empty() {
        Encode!(&target_canister)
    } else {
        let lens_targets = lens_target_name_or_id
            .iter()
            .map(|t| resolver_name_or_id(t))
            .collect::<Vec<String>>();
        Encode!(&target_canister, &lens_targets)
    }?;
    Ok(args)
}
