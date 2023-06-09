use chainsight_cdk_macros::{
    cross_canister_call_func, define_get_ethereum_address, define_transform_for_web3,
    define_web3_ctx, did_export, manage_single_state, monitoring_canister_metrics, setup_func,
    timer_task_func,
};
use ic_web3::types::{Address, U256};
use std::str::FromStr;
monitoring_canister_metrics!(60);
define_web3_ctx!();
define_transform_for_web3!();
manage_single_state!("target_addr", String, false);
manage_single_state!("target_canister", String, false);
setup_func ! ({ target_canister : String , target_addr : String , web3_ctx_param : Web3CtxParam });
define_get_ethereum_address!();
timer_task_func!("set_task", "sync", true);
ic_solidity_bindgen::contract_abi!("./__interfaces/Uint256Oracle.json");
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize)]
pub struct StringValueWithTimestamp {
    pub value: String,
    pub timestamp: u64,
}
type CallCanisterArgs = ();
type CallCanisterResponse = (StringValueWithTimestamp);
cross_canister_call_func!("get_last_snapshot", CallCanisterArgs, CallCanisterResponse);
async fn sync() {
    let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
    let res = call_get_last_snapshot(target_canister, ()).await;
    if let Err(err) = res {
        ic_cdk::println!("error: {:?}", err);
        return;
    }
    let datum = res.unwrap();
    Uint256Oracle::new(
        Address::from_str(&get_target_addr()).unwrap(),
        &web3_ctx().unwrap(),
    )
    .update_state(U256::from_str(&datum.value).unwrap())
    .await
    .unwrap();
    ic_cdk::println!("ts={}, value={:?}", datum.timestamp, datum.value);
}
did_export!("initial_pj_relayer");
