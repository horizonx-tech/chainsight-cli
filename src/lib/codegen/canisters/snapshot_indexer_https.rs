use std::collections::HashMap;

use anyhow::ensure;
use chainsight_cdk::{config::components::SnapshotIndexerHTTPSConfig, web2::build_url};
use quote::quote;

use crate::{
    lib::codegen::components::snapshot_indexer_https::{
        SnapshotIndexerHTTPSComponentManifest, SnapshotIndexerHTTPSDataSourceQueries,
    },
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
    let (queries, query_func) = match datasource.queries.clone() {
        SnapshotIndexerHTTPSDataSourceQueries::Static(queries) => (
            queries.into_iter().collect::<HashMap<String, String>>(),
            String::new(),
        ),
        _ => {
            let comment_for_query_func =
                "// You can implement the function to get the query parameters.\n".to_string();
            let query_func_quote = quote! {
                use std::collections::BTreeMap;
                pub fn get_query_parameters() -> BTreeMap<String, String> {
                    BTreeMap::from([
                        ("param1".to_string(), "value1".to_string()),
                        ("param2".to_string(), "value2".to_string())
                    ])
                }
            };
            (
                HashMap::new(),
                comment_for_query_func + &query_func_quote.to_string() + "\n",
            )
        }
    };
    let url = build_url(&datasource.url, queries);

    let mut options = json_typegen_shared::Options::default();
    options.derives = "Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable".into();
    options.import_style = json_typegen_shared::ImportStyle::QualifiedPaths;
    let codes = json_typegen_shared::codegen(struct_name, &url, options)
        .map_err(|e| anyhow::anyhow!("Failed to generate code by json_typegen_shared: {:?}", e))?;

    let comments_for_typegen = r#"// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
"#;
    let use_declares_for_typegen = "use candid::{Decode, Encode};\n";

    let top_comments = "// Auto-generated code from manifest.\n".to_string();

    Ok(top_comments + &query_func + comments_for_typegen + use_declares_for_typegen + &codes)
}
