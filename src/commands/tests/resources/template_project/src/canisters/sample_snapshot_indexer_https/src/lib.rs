use candid::{Decode, Encode};
use chainsight_cdk::core::HttpsSnapshotIndexerSourceAttrs;
use chainsight_cdk::web2::{HttpsSnapshotParam, Web2HttpsSnapshotIndexer};
use chainsight_cdk_macros::{
    chainsight_common, did_export, init_in, prepare_stable_structure, snapshot_https_source,
    stable_memory_for_vec, timer_task_func, StableMemoryStorable,
};
use std::collections::HashMap;
init_in!();
chainsight_common!(60);
use sample_snapshot_indexer_https::*;
#[derive(
    Debug,
    Clone,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    StableMemoryStorable,
)]
#[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)]
pub struct Snapshot {
    pub value: SnapshotValue,
    pub timestamp: u64,
}
prepare_stable_structure!();
stable_memory_for_vec!("snapshot", Snapshot, 0, true);
timer_task_func!("set_task", "index", true);
const URL: &str = "https://api.coingecko.com/api/v3/simple/price";
fn get_attrs() -> HttpsSnapshotIndexerSourceAttrs {
    HttpsSnapshotIndexerSourceAttrs {
        queries: HashMap::from([
            ("ids".to_string(), "dai".to_string()),
            ("vs_currencies".to_string(), "usd".to_string()),
        ]),
    }
}
async fn index() {
    let indexer = Web2HttpsSnapshotIndexer::new(URL.to_string());
    let res = indexer
        .get::<String, SnapshotValue>(HttpsSnapshotParam {
            headers: vec![("Content-Type".to_string(), "application/json".to_string())]
                .into_iter()
                .collect(),
            queries: vec![
                ("ids".to_string(), "dai".to_string()),
                ("vs_currencies".to_string(), "usd".to_string()),
            ]
            .into_iter()
            .collect(),
        })
        .await
        .unwrap();
    let snapshot = Snapshot {
        value: res,
        timestamp: ic_cdk::api::time() / 1000000,
    };
    let _ = add_snapshot(snapshot.clone());
}
snapshot_https_source!();
fn _get_last_snapshot_value() -> SnapshotValue {
    get_last_snapshot().value
}
fn _get_top_snapshot_values(n: u64) -> Vec<SnapshotValue> {
    get_top_snapshots(n)
        .iter()
        .map(|s| s.value.clone())
        .collect()
}
fn _get_snapshot_value(idx: u64) -> SnapshotValue {
    get_snapshot(idx).value
}
#[ic_cdk::query]
#[candid::candid_method(query)]
pub fn get_last_snapshot_value() -> SnapshotValue {
    _get_last_snapshot_value()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
pub fn get_top_snapshot_values(n: u64) -> Vec<SnapshotValue> {
    _get_top_snapshot_values(n)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
pub fn get_snapshot_value(idx: u64) -> SnapshotValue {
    _get_snapshot_value(idx)
}
#[ic_cdk::update]
#[candid::candid_method(update)]
pub async fn proxy_get_last_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
    use chainsight_cdk::rpc::Receiver;
    chainsight_cdk::rpc::ReceiverProviderWithoutArgs::<SnapshotValue>::new(
        proxy(),
        _get_last_snapshot_value,
    )
    .reply(input)
    .await
}
#[ic_cdk::update]
#[candid::candid_method(update)]
pub async fn proxy_get_top_snapshot_values(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
    use chainsight_cdk::rpc::Receiver;
    chainsight_cdk::rpc::ReceiverProvider::<u64, Vec<SnapshotValue>>::new(
        proxy(),
        _get_top_snapshot_values,
    )
    .reply(input)
    .await
}
#[ic_cdk::update]
#[candid::candid_method(update)]
pub async fn proxy_get_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
    use chainsight_cdk::rpc::Receiver;
    chainsight_cdk::rpc::ReceiverProvider::<u64, SnapshotValue>::new(proxy(), _get_snapshot_value)
        .reply(input)
        .await
}
did_export!("sample_snapshot_indexer_https");
