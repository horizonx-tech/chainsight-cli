// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};


#[derive(CandidType, Deserialize)]
pub struct CanisterMetricsSnapshot { pub cycles: candid::Nat, pub timestamp: u64 }

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
pub struct Snapshot { pub value: String, pub timestamp: u64 }

#[derive(CandidType, Deserialize)]
pub enum SourceType { evm, https, chainsight }

#[derive(CandidType, Deserialize)]
pub struct Sources {
  pub source: String,
  pub interval_sec: Option<u32>,
  pub attributes: Box<Web3AlgorithmIndexerSourceAttrs>,
  pub source_type: SourceType,
}

#[derive(CandidType, Deserialize)]
pub struct TransformArgs {
  pub context: serde_bytes::ByteBuf,
  pub response: HttpResponse,
}

#[derive(CandidType, Deserialize)]
pub struct Web3AlgorithmIndexerSourceAttrs {
  pub chain_id: u64,
  pub function_name: String,
}

#[derive(CandidType, Deserialize)]
pub struct Web3CtxParam {
  pub env: Env,
  pub url: String,
  pub from: Option<String>,
  pub chain_id: u64,
}

