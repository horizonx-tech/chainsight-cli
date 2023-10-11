use candid::{Decode, Encode};
use chainsight_cdk_macros::{
    chainsight_common, define_transform_for_web3, define_web3_ctx, did_export, init_in,
    manage_single_state, prepare_stable_structure, setup_func, snapshot_web3_source,
    stable_memory_for_vec, timer_task_func, StableMemoryStorable,
};
use ic_web3_rs::types::Address;
use std::str::FromStr;
init_in!();
chainsight_common!(3600);
define_web3_ctx!();
define_transform_for_web3!();
manage_single_state!("target_addr", String, false);
setup_func ! ({ target_addr : String , web3_ctx_param : chainsight_cdk :: web3 :: Web3CtxParam });
prepare_stable_structure!();
stable_memory_for_vec!("snapshot", Snapshot, 0, true);
timer_task_func!("set_task", "execute_task", true);
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
type SnapshotValue = (String);
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
ic_solidity_bindgen::contract_abi!("./__interfaces/ERC20.json");
snapshot_web3_source!("total_supply");
async fn execute_task() {
    let current_ts_sec = ic_cdk::api::time() / 1000000;
    let res = ERC20::new(
        Address::from_str(&get_target_addr()).unwrap(),
        &web3_ctx().unwrap(),
    )
    .total_supply(None)
    .await
    .unwrap();
    let datum = Snapshot {
        value: (res.to_string()),
        timestamp: current_ts_sec,
    };
    add_snapshot(datum.clone());
    ic_cdk::println!("ts={}, snapshot={:?}", datum.timestamp, datum.value);
}
did_export!("sample_snapshot_indexer_evm");
