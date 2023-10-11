use candid::CandidType;
use chainsight_cdk::core::*;
use chainsight_cdk::{indexer::IndexingConfig, storage::Data};
use chainsight_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
chainsight_common!(3600);
init_in!();
manage_single_state!("target_addr", String, false);
setup_func ! ({ target_addr : String , config : IndexingConfig });
timer_task_func!("set_task", "index", true);
use sample_algorithm_indexer::*;
algorithm_indexer ! (HashMap < u64 , Vec < Transfer >> , "proxy_call");
#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize, Persist, KeyValueStore)]
#[memory_id(1i32)]
pub struct Account {
    pub address: String,
}
did_export!("sample_algorithm_indexer");
