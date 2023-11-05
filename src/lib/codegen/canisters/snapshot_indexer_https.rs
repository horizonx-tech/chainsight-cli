use std::collections::HashMap;

use anyhow::ensure;
use chainsight_cdk::config::components::SnapshotIndexerHTTPSConfig;
use quote::quote;

use crate::{
    lib::codegen::components::snapshot_indexer_https::SnapshotIndexerHTTPSComponentManifest,
    types::ComponentType,
};

pub fn generate_codes(manifest: &SnapshotIndexerHTTPSComponentManifest) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerHTTPS,
        "type is not SnapshotIndexerHTTPS"
    );
    let config: SnapshotIndexerHTTPSConfig = manifest.clone().into();
    let config_json = serde_json::to_string(&config)?;
    let code = quote! {
        use chainsight_cdk_macros::def_snapshot_indexer_https_canister;
        def_snapshot_indexer_https_canister!(#config_json);
    };
    Ok(code.to_string())
}

pub fn generate_app(_manifest: &SnapshotIndexerHTTPSComponentManifest) -> anyhow::Result<String> {
    let v = quote! {
        use candid::{Decode, Encode};
        use chainsight_cdk_macros::StableMemoryStorable;
        // todo!("Implement a structure that matches the response type")
        #[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable)]
        pub struct SnapshotValue {
           pub dummy: u64
        }
    };
    Ok(v.to_string())
}
