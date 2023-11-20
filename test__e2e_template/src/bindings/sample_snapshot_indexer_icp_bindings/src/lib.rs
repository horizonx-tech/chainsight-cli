#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct ResponseType { pub value: String, pub timestamp: u64 }