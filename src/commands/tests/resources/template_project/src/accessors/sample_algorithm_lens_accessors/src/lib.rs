use sample_snapshot_indexer_icp_bindings::Snapshot as Snapshot_sample_snapshot_indexer_icp;
algorithm_lens_finder!(
    "sample_snapshot_indexer_icp",
    "proxy_get_last_snapshot",
    Snapshot_sample_snapshot_indexer_icp
);
use chainsight_cdk::lens::LensFinder;
use chainsight_cdk_macros::algorithm_lens_finder;
async fn _get_target_proxy(target: candid::Principal) -> candid::Principal {
    let out: ic_cdk::api::call::CallResult<(candid::Principal,)> =
        ic_cdk::api::call::call(target, "get_proxy", ()).await;
    out.unwrap().0
}
