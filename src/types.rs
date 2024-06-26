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

    /// To get events from other indexer and convert it into another format
    #[serde(rename = "algorithm_indexer")]
    AlgorithmIndexer,

    /// To periodically take and store snapshots from other Canisters
    #[serde(rename = "snapshot_indexer_icp")]
    SnapshotIndexerICP,

    /// To periodically take and store snapshots from Contract
    #[serde(rename = "snapshot_indexer_evm")]
    SnapshotIndexerEVM,

    /// To periodically take and store snapshots using HTTPS Outcall
    #[serde(rename = "snapshot_indexer_https")]
    SnapshotIndexerHTTPS,

    /// To relay data to other blockchains
    #[serde(rename = "relayer")]
    Relayer,

    /// To calculate using data obtained from the specified Source and process into an arbitrary format
    #[serde(rename = "algorithm_lens")]
    AlgorithmLens,
}

impl ComponentType {
    pub fn all() -> &'static [ComponentType] {
        &[
            ComponentType::EventIndexer,
            ComponentType::AlgorithmIndexer,
            ComponentType::SnapshotIndexerICP,
            ComponentType::SnapshotIndexerEVM,
            ComponentType::Relayer,
            ComponentType::AlgorithmLens,
            ComponentType::SnapshotIndexerHTTPS,
        ]
    }
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentType::EventIndexer => write!(f, "event_indexer"),
            ComponentType::AlgorithmIndexer => write!(f, "algorithm_indexer"),
            ComponentType::SnapshotIndexerICP => write!(f, "snapshot_indexer_icp"),
            ComponentType::SnapshotIndexerEVM => write!(f, "snapshot_indexer_evm"),
            ComponentType::Relayer => write!(f, "relayer"),
            ComponentType::AlgorithmLens => write!(f, "algorithm_lens"),
            ComponentType::SnapshotIndexerHTTPS => write!(f, "snapshot_indexer_https"),
        }
    }
}

/// Supported Network Types
///
/// IC is equivalent to '--network ic' in dfx
#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum Network {
    Local,
    IC, // ref: https://internetcomputer.org/docs/current/developer-docs/setup/deploy-mainnet#step-2--check-the-current-status-of-the-ic-and-your-ability-to-connect-to-it-by-running-the-following-command-for-the-network-alias-ic
}

impl Network {
    pub fn to_url(&self, port: Option<u16>) -> String {
        match self {
            Network::Local => format!("http://localhost:{}", port.unwrap_or(4943)),
            Network::IC => "https://ic0.app/".to_string(),
        }
    }

    pub fn to_sdk_env(&self) -> chainsight_cdk::core::Env {
        match self {
            Network::Local => chainsight_cdk::core::Env::LocalDevelopment,
            Network::IC => chainsight_cdk::core::Env::Production,
        }
    }
}
