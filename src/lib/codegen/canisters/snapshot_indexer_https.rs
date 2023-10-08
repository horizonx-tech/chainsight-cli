use quote::{format_ident, quote};

use crate::lib::codegen::components::snapshot_indexer_https::SnapshotIndexerHTTPSComponentManifest;

use super::snapshot_indexer_icp::generate_queries_without_timestamp;

pub fn generate_codes(
    manifest: &SnapshotIndexerHTTPSComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let url = &manifest.datasource.url;
    let label = &manifest.metadata.label;
    let mut header_keys: Vec<&String> = manifest.datasource.headers.keys().collect();
    header_keys.sort();
    let mut header_values: Vec<&String> = manifest.datasource.headers.values().collect();
    header_values.sort();
    let mut query_keys: Vec<&String> = manifest.datasource.queries.keys().collect();
    query_keys.sort();
    let mut query_values: Vec<&String> = manifest.datasource.queries.values().collect();
    query_values.sort();
    let queries = generate_queries_without_timestamp(format_ident!("SnapshotValue"));

    let label_ident = format_ident!("{}", label);
    let out = quote! {
        use std::collections::HashMap;

        use chainsight_cdk::core::HttpsSnapshotIndexerSourceAttrs;
        use chainsight_cdk::web2::{HttpsSnapshotParam, Web2HttpsSnapshotIndexer};
        use chainsight_cdk_macros::{
            chainsight_common, did_export, init_in, prepare_stable_structure, stable_memory_for_vec,
            timer_task_func, snapshot_https_source, StableMemoryStorable,
        };
        use candid::{Decode, Encode};

        init_in!();
        chainsight_common!(60);
        use #label_ident::*;
        #[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable)]
        #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
        pub struct Snapshot {
            pub value: SnapshotValue,
            pub timestamp: u64,
        }
        prepare_stable_structure!();
        stable_memory_for_vec!("snapshot", Snapshot, 0, true);
        timer_task_func!("set_task", "index", true);

        const URL : &str = #url;
        fn get_attrs() -> HttpsSnapshotIndexerSourceAttrs {
            HttpsSnapshotIndexerSourceAttrs {
                queries: HashMap::from([
                    #(
                        (#query_keys.to_string(), #query_values.to_string()),
                    )*
                ]),
            }
        }
        async fn index() {
            let indexer = Web2HttpsSnapshotIndexer::new(
                URL.to_string(),
            );
            let res = indexer.get::<String, SnapshotValue>(
                HttpsSnapshotParam {
                    headers: vec![
                        #(
                            (#header_keys.to_string(), #header_values.to_string()),
                        )*
                    ].into_iter().collect(),
                    queries: vec![
                        #(
                            (#query_keys.to_string(), #query_values.to_string()),
                        )*
                    ].into_iter().collect(),
                }
            ).await.unwrap();
            let snapshot = Snapshot {
                value: res,
                timestamp: ic_cdk::api::time() / 1000000,
            };
            let _ = add_snapshot(snapshot.clone());
        }
        snapshot_https_source!();
        #queries
        did_export!(#label);
    };
    Ok(out)
}

pub fn generate_app(
    _manifest: &SnapshotIndexerHTTPSComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let v = quote! {
        use candid::{Decode, Encode};
        use chainsight_cdk_macros::StableMemoryStorable;
        // todo!("Implement a structure that matches the response type")
        #[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable)]
        pub struct SnapshotValue {
           pub dummy: u64
        }
    };
    Ok(v)
}
