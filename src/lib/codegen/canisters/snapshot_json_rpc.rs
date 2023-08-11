use quote::{format_ident, quote};

use crate::lib::codegen::components::snapshot_json_rpc::SnapshotJsonRPCComponentManifest;

use super::snapshot::generate_queries_without_timestamp;

pub fn generate_codes(
    manifest: &SnapshotJsonRPCComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let url = &manifest.datasource.url;
    let label = &manifest.metadata.label;
    let header_keys: Vec<&String> = manifest.datasource.headers.keys().collect();
    let header_values: Vec<&String> = manifest.datasource.headers.values().collect();
    let query_keys: Vec<&String> = manifest.datasource.queries.keys().collect();
    let query_values: Vec<&String> = manifest.datasource.queries.values().collect();
    let queries = generate_queries_without_timestamp(format_ident!("SnapshotValue"));
    let out = quote! {
        use chainsight_cdk::web2::{JsonRpcSnapshotParam, Web2JsonRpcSnapshotIndexer};
        use chainsight_cdk_macros::{
            chainsight_common, did_export, init_in, prepare_stable_structure, stable_memory_for_vec,
            timer_task_func, StableMemoryStorable,
        };
        use candid::{Decode, Encode};

        init_in!();
        chainsight_common!(60);
        mod app;
        use app::*;
        #[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable)]
        #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
        pub struct Snapshot {
            pub value: SnapshotValue,
            pub timestamp: u64,
        }
        prepare_stable_structure!();
        stable_memory_for_vec!("snapshot", Snapshot, 0, true);
        timer_task_func!("set_task", "index", true);
        async fn index() {
            let indexer = Web2JsonRpcSnapshotIndexer::new(
                #url.to_string(),
            );
            let res = indexer.get::<String, SnapshotValue>(
                JsonRpcSnapshotParam {
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
        #queries
        did_export!(#label);
    };
    Ok(out)
}
