use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, clap::ValueEnum)]
pub enum ComponentType {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "canister")]
    Relayer,
}
