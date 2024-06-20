use candid::Encode;
use chainsight_cdk::{indexer::IndexingConfig, web3::Web3CtxParam};

use crate::{
    lib::codegen::components::event_indexer::EventIndexerComponentManifest, types::Network,
};

pub fn generate_component_setup_args(
    manifest: &EventIndexerComponentManifest,
    network: &Network,
) -> anyhow::Result<Vec<u8>> {
    let args = Encode!(
        &manifest.datasource.id,
        &Web3CtxParam {
            url: manifest.datasource.network.rpc_url.clone(),
            from: None,
            chain_id: manifest.datasource.network.chain_id,
            env: network.to_sdk_env()
        },
        &IndexingConfig {
            start_from: manifest.datasource.from,
            chunk_size: manifest.datasource.batch_size,
        }
    )?;

    Ok(args)
}
