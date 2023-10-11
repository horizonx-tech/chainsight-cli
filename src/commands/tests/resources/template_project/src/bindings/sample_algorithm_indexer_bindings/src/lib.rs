// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};


#[derive(CandidType, Deserialize)]
pub struct Account { pub address: String }

#[derive(CandidType, Deserialize)]
pub struct CanisterMetricsSnapshot { pub cycles: candid::Nat, pub timestamp: u64 }

#[derive(CandidType, Deserialize)]
pub enum Env { Production, Test, LocalDevelopment }

#[derive(CandidType, Deserialize)]
pub struct IndexingConfig { pub start_from: u64, pub chunk_size: Option<u64> }

#[derive(CandidType, Deserialize)]
pub enum InitError {
  InvalidDestination(String),
  InvalidPrincipal(Principal),
  InvalidContent(String),
  InvalidRequest(String),
}

#[derive(CandidType, Deserialize)]
enum Result { Ok, Err(InitError) }

#[derive(CandidType, Deserialize)]
enum Result_1 { Ok, Err(String) }

#[derive(CandidType, Deserialize)]
pub enum SourceType { evm, https, chainsight }

#[derive(CandidType, Deserialize)]
pub struct Sources {
  pub source: String,
  pub interval_sec: Option<u32>,
  pub attributes: Vec<(String,String,)>,
  pub source_type: SourceType,
}

