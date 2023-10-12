use candid::CandidType;
use chainsight_cdk::{
    core::U256,
    indexer::{Event, Indexer, IndexingConfig},
    storage::Data,
    web3::Web3CtxParam,
};
use chainsight_cdk_macros::{
    chainsight_common, define_get_ethereum_address, define_transform_for_web3, define_web3_ctx,
    did_export, init_in, manage_single_state, setup_func, timer_task_func, web3_event_indexer,
    ContractEvent, Persist,
};
use ic_solidity_bindgen::types::EventLog;
use ic_web3_rs::{
    ethabi::Address,
    futures::{future::BoxFuture, FutureExt},
    transports::ic_http_client::CallOptions,
};
use serde::Serialize;
use std::{collections::HashMap, str::FromStr};
chainsight_common!(3600);
define_web3_ctx!();
define_transform_for_web3!();
define_get_ethereum_address!();
timer_task_func!("set_task", "index", true);
manage_single_state!("target_addr", String, false);
setup_func ! ({ target_addr : String , web3_ctx_param : Web3CtxParam , config : IndexingConfig , });
init_in!();
ic_solidity_bindgen::contract_abi!("./__interfaces/ERC20.json");
web3_event_indexer!(Transfer);
#[derive(Clone, Debug, Default, candid :: CandidType, ContractEvent, Serialize, Persist)]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub value: U256,
}
impl chainsight_cdk::indexer::Event<EventLog> for Transfer {
    fn tokenize(&self) -> chainsight_cdk::storage::Data {
        self._tokenize()
    }
    fn untokenize(data: chainsight_cdk::storage::Data) -> Self {
        Transfer::_untokenize(data)
    }
}
fn get_logs(
    from: u64,
    to: u64,
    call_options: CallOptions,
) -> BoxFuture<'static, Result<HashMap<u64, Vec<EventLog>>, chainsight_cdk::indexer::Error>> {
    async move {
        let res = ERC20::new(
            Address::from_str(get_target_addr().as_str()).unwrap(),
            &web3_ctx().unwrap(),
        )
        .event_transfer(from, to, call_options)
        .await;
        match res {
            Ok(logs) => Ok(logs),
            Err(e) => Err(chainsight_cdk::indexer::Error::OtherError(e.to_string())),
        }
    }
    .boxed()
}
did_export!("sample_event_indexer");