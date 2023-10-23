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
pub struct HttpHeader { pub value: String, pub name: String }

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
  pub status: candid::Nat,
  pub body: serde_bytes::ByteBuf,
  pub headers: Vec<HttpHeader>,
}

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
enum Result_ { Ok, Err(InitError) }

#[derive(CandidType, Deserialize)]
enum Result1 { Ok, Err(String) }

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
  pub attributes: Box<Web3EventIndexerSourceAttrs>,
  pub source_type: SourceType,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer { pub to: String, pub value: Box<U256>, pub from: String }

#[derive(CandidType, Deserialize)]
pub struct TransformArgs {
  pub context: serde_bytes::ByteBuf,
  pub response: HttpResponse,
}

#[derive(CandidType, Deserialize)]
pub struct U256 { pub value: String }

#[derive(CandidType, Deserialize)]
pub struct Web3CtxParam {
  pub env: Env,
  pub url: String,
  pub from: Option<String>,
  pub chain_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Web3EventIndexerSourceAttrs {
  pub chain_id: u64,
  pub event_name: String,
}

