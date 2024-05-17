use anyhow::ensure;
use chainsight_cdk::config::components::SnapshotIndexerEVMConfig;
use quote::quote;

use crate::{
    lib::{
        codegen::components::snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
        utils::url::{is_supporting_ipv6_url, is_valid_rpc_url},
    },
    types::ComponentType,
};

pub fn generate_codes(manifest: &SnapshotIndexerEVMComponentManifest) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerEVM,
        "type is not SnapshotIndexerEVM"
    );
    let config: SnapshotIndexerEVMConfig = manifest.clone().into();
    let config_json = serde_json::to_string(&config)?;
    let code = quote! {
        use chainsight_cdk_macros::def_snapshot_indexer_evm_canister;
        def_snapshot_indexer_evm_canister!(#config_json);
    };
    Ok(code.to_string())
}

pub fn generate_app(_manifest: &SnapshotIndexerEVMComponentManifest) -> anyhow::Result<String> {
    Ok(quote! {}.to_string())
}

pub fn validate_manifest(manifest: &SnapshotIndexerEVMComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerEVM,
        "type is not SnapshotIndexerEVM"
    );

    let rpc_url = &manifest.datasource.location.args.rpc_url;
    is_supporting_ipv6_url(rpc_url)?;
    is_valid_rpc_url(rpc_url)?;

    Ok(())
}
