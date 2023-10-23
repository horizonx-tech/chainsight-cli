// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};


#[derive(CandidType, Deserialize)]
pub struct CanisterMetricsSnapshot {
  pub cycles: candid::Nat,
  pub timestamp: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Env { Production, Test, LocalDevelopment }

#[derive(CandidType, Deserialize)]
pub struct HttpsSnapshotIndexerSourceAttrs {
  pub queries: Vec<(String,String,)>,
}

#[derive(CandidType, Deserialize)]
pub enum InitError {
  InvalidDestination(String),
  InvalidPrincipal(Principal),
  InvalidContent(String),
  InvalidRequest(String),
}

#[derive(CandidType, Deserialize)]
enum Result_ { Ok, Err(InitError) }

#[derive(CandidType, Deserialize)]
pub struct Snapshot { pub value: Box<SnapshotValue>, pub timestamp: u64 }

#[derive(CandidType, Deserialize)]
pub struct SnapshotValue { pub dummy: u64 }

#[derive(CandidType, Deserialize)]
pub enum SourceType {
  #[serde(rename="evm")]
  Evm,
  #[serde(rename="https")]
  Https,
  #[serde(rename="chainsight")]
  Chainsight,
}

#[derive(CandidType, Deserialize)]
pub struct Sources {
  pub source: String,
  pub interval_sec: Option<u32>,
  pub attributes: HttpsSnapshotIndexerSourceAttrs,
  pub source_type: SourceType,
}

