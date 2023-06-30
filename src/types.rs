use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum ComponentType {
    #[serde(rename = "event_indexer")]
    EventIndexer,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "relayer")]
    Relayer,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentType::EventIndexer => write!(f, "event_indexer"),
            ComponentType::Snapshot => write!(f, "snapshot"),
            ComponentType::Relayer => write!(f, "relayer"),
        }
    }
}