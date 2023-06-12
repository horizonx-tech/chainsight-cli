use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum ComponentType {
    #[serde(rename = "event_indexer")]
    EventIndexer,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "canister")]
    Relayer,
}
