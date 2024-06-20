use candid::Encode;
use chainsight_cdk::web3::Web3CtxParam;

use crate::{
    lib::codegen::components::snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
    types::Network,
};

pub fn generate_component_setup_args(
    manifest: &SnapshotIndexerEVMComponentManifest,
    network: &Network,
) -> anyhow::Result<Vec<u8>> {
    let datasrc_location_args = manifest.datasource.location.args.clone();
    let args = Encode!(
        &manifest.datasource.location.id,
        &Web3CtxParam {
            url: datasrc_location_args.rpc_url,
            from: None,
            chain_id: datasrc_location_args.network_id as u64,
            env: network.to_sdk_env(),
        }
    )?;
    Ok(args)
}
