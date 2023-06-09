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
ic_solidity_bindgen::contract_abi!("./__interfaces/UniswapV3Factory.json");
async fn execute_task() {
    let current_ts_sec = ic_cdk::api::time() / 1000000;
    let res = UniswapV3Factory::new(
        Address::from_str(&get_target_addr()).unwrap(),
        &web3_ctx().unwrap(),
    )
    .get_pool(
        ic_web3::types::Address::from_str("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap(),
        ic_web3::types::Address::from_str("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
        500u64 as u32,
    )
    .await
    .unwrap();
    let datum = Snapshot {
        value: (hex::encode(res)),
        timestamp: current_ts_sec,
    };
    add_snapshot(datum.clone());
    ic_cdk::println!("ts={}, snapshot={:?}", datum.timestamp, datum.value);
}
did_export!("uniswapv3factory_chain");
