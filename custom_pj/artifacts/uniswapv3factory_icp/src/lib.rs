use chainsight_cdk_macros::{
    cross_canister_call_func, did_export, manage_single_state, manage_vec_state,
    monitoring_canister_metrics, setup_func, timer_task_func,
};
monitoring_canister_metrics!(60);
manage_single_state!("target_canister", String, false);
setup_func!({ target_canister: String });
manage_vec_state!("snapshot", Snapshot, true);
timer_task_func!("set_task", "execute_task", true);
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize)]
pub struct Snapshot {
    pub value: SnapshotValue,
    pub timestamp: u64,
}
type SnapshotValue = StringValueWithTimestamp;
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize)]
pub struct StringValueWithTimestamp {
    pub value: String,
    pub timestamp: u64,
}
type CallCanisterArgs = (u64);
type CallCanisterResponse = (StringValueWithTimestamp);
cross_canister_call_func!("get_snapshot", CallCanisterArgs, CallCanisterResponse);
async fn execute_task() {
    let current_ts_sec = ic_cdk::api::time() / 1000000;
    let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
    let res = call_get_snapshot(target_canister, (0u64 as u64)).await;
    if let Err(err) = res {
        ic_cdk::println!("error: {:?}", err);
        return;
    }
    let datum = Snapshot {
        value: res.unwrap().clone(),
        timestamp: current_ts_sec,
    };
    add_snapshot(datum.clone());
    ic_cdk::println!("ts={}, value={:?}", datum.timestamp, datum.value);
}
did_export!("uniswapv3factory_icp");
