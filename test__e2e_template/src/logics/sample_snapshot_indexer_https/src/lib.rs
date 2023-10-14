use candid::{Decode, Encode};
use chainsight_cdk_macros::StableMemoryStorable;
#[derive(
    Debug,
    Clone,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    StableMemoryStorable,
)]
pub struct SnapshotValue {
    pub dummy: u64,
}
