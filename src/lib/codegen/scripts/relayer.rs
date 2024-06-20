use anyhow::Context;
use candid::{Encode, Principal};
use chainsight_cdk::web3::Web3CtxParam;

use crate::{
    lib::{
        codegen::components::relayer::RelayerComponentManifest,
        utils::component_ids_manager::ComponentIdsManager,
    },
    types::Network,
};

pub fn generate_component_setup_args(
    manifest: &RelayerComponentManifest,
    network: &Network,
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
    let web3_ctx_param = Web3CtxParam {
        url: manifest.destination.rpc_url.clone(),
        from: None,
        chain_id: manifest.destination.network_id as u64,
        env: network.to_sdk_env(),
    };
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
        Encode!(
            &manifest.destination.oracle_address,
            &web3_ctx_param,
            &target_canister
        )
    } else {
        let lens_targets = lens_target_name_or_id
            .iter()
            .map(|t| resolver_name_or_id(t))
            .collect::<Vec<String>>();
        Encode!(
            &manifest.destination.oracle_address,
            &web3_ctx_param,
            &target_canister,
            &lens_targets
        )
    }?;

    Ok(args)
}
