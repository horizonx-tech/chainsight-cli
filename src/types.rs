use std::fmt;

use serde::{Deserialize, Serialize};

/// Data Processing Component Types
///
/// Defines the types of components used to collect/process/reference data in Chainsight.
/// Some Components are still undefined (not yet implemented) because they are still under development.
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum ComponentType {
    /// To synchronize event data
    #[serde(rename = "event_indexer")]
    EventIndexer,

    #[serde(rename = "algorithm_indexer")]
    /// get events from other indexer and convert it into another format
    AlgorithmIndexer,

    /// To periodically take and store snapshots from Contract and other Canisters
    #[serde(rename = "snapshot_indexer")]
    Snapshot,

    /// To relay data to other blockchains
    #[serde(rename = "relayer")]
    Relayer,
    #[serde(rename = "algorithm_lens")]
    AlgorithmLens,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentType::EventIndexer => write!(f, "event_indexer"),
            ComponentType::AlgorithmIndexer => write!(f, "algorithm_indexer"),
            ComponentType::Snapshot => write!(f, "snapshot_indexer"),
            ComponentType::Relayer => write!(f, "relayer"),
            ComponentType::AlgorithmLens => write!(f, "algorithm_lens"),
        }
    }
}

/// Supported Network Types
///
/// IC is equivalent to '--network ic' in dfx
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Network {
    Local,
    IC, // ref: https://internetcomputer.org/docs/current/developer-docs/setup/deploy-mainnet#step-2--check-the-current-status-of-the-ic-and-your-ability-to-connect-to-it-by-running-the-following-command-for-the-network-alias-ic
}
