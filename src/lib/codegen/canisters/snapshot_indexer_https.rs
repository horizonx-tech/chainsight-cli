use anyhow::ensure;
use chainsight_cdk::{config::components::SnapshotIndexerHTTPSConfig, web2::build_url};
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

// TODO: Support for typegen with sample response and json schema
pub fn generate_app(manifest: &SnapshotIndexerHTTPSComponentManifest) -> anyhow::Result<String> {
    let SnapshotIndexerHTTPSComponentManifest { datasource, .. } = manifest;
    let struct_name = "SnapshotValue";
    let url = build_url(&datasource.url, datasource.queries.clone());

    let mut options = json_typegen_shared::Options::default();
    options.derives = "Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable".into();
    options.import_style = json_typegen_shared::ImportStyle::QualifiedPaths;
    let codes = json_typegen_shared::codegen(struct_name, &url, options)
        .map_err(|e| anyhow::anyhow!("Failed to generate code by json_typegen_shared: {:?}", e))?;

    let comments = r#"// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
"#
    .to_string();

    let use_declares = "use candid::{Decode, Encode};\n";

    Ok(comments + use_declares + &codes)
}
