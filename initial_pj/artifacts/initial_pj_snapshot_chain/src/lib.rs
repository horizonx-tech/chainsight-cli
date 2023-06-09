use chainsight_cdk_macros::{
    define_transform_for_web3, define_web3_ctx, did_export, manage_single_state, manage_vec_state,
    monitoring_canister_metrics, setup_func, timer_task_func,
};
use ic_web3::types::Address;
use std::str::FromStr;
monitoring_canister_metrics!(60);
define_web3_ctx!();
define_transform_for_web3!();
manage_single_state!("target_addr", String, false);
setup_func ! ({ target_addr : String , web3_ctx_param : Web3CtxParam });
manage_vec_state!("snapshot", Snapshot, true);
timer_task_func!("set_task", "execute_task", true);
#[derive(Debug, Clone, candid :: CandidType, candid :: Deserialize)]
pub struct Snapshot {
    pub value: SnapshotValue,
    pub timestamp: u64,
}
type SnapshotValue = (String);
ic_solidity_bindgen::contract_abi!("./__interfaces/ERC20.json");
async fn execute_task() {
    let current_ts_sec = ic_cdk::api::time() / 1000000;
    let res = ERC20::new(
        Address::from_str(&get_target_addr()).unwrap(),
        &web3_ctx().unwrap(),
    )
    .total_supply()
    .await
    .unwrap();
    let datum = Snapshot {
        value: (res.to_string()),
        timestamp: current_ts_sec,
    };
    add_snapshot(datum.clone());
    ic_cdk::println!("ts={}, snapshot={:?}", datum.timestamp, datum.value);
}
did_export!("initial_pj_snapshot_chain");
