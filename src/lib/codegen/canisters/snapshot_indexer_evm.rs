use anyhow::ensure;
use chainsight_cdk::config::components::SnapshotIndexerEVMConfig;
use quote::quote;

use crate::{
    lib::codegen::components::snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
    types::ComponentType,
};

pub fn generate_codes(
    manifest: &SnapshotIndexerEVMComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
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
    Ok(code)
}

pub fn generate_app(
    _manifest: &SnapshotIndexerEVMComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    Ok(quote! {})
}

pub fn validate_manifest(manifest: &SnapshotIndexerEVMComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerEVM,
        "type is not SnapshotIndexerEVM"
    );

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length

    Ok(())
}
