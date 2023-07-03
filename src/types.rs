use std::fmt;

use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Network {
    Local,
    IC, // ref: https://internetcomputer.org/docs/current/developer-docs/setup/deploy-mainnet#step-2--check-the-current-status-of-the-ic-and-your-ability-to-connect-to-it-by-running-the-following-command-for-the-network-alias-ic
}
