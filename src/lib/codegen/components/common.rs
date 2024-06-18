use std::{
    collections::{BTreeMap, HashMap},
    fs::OpenOptions,
    io::Read,
    path::Path,
};

use anyhow::{bail, Context};
use chainsight_cdk::initializer::{CycleManagement, CycleManagements};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    lib::utils::{env::load_env, serializer::ordered_map},
    types::ComponentType,
};

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum DestinationType {
    #[serde(rename = "uint256")]
    Uint256,
    #[serde(rename = "uint128")]
    Uint128,
    #[serde(rename = "uint64")]
    Uint64,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "custom")]
    Custom,
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]

pub enum SourceType {
    #[serde(rename = "evm")]
    Evm,
    #[serde(rename = "chainsight")]
    Chainsight,
    #[serde(rename = "https")]
    Https,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub struct Sources {
    pub source_type: SourceType,
    pub source: String,
    #[serde(serialize_with = "ordered_map")]
    pub attributes: HashMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ComponentMetadata {
    pub label: String,
    #[serde(rename = "type")]
    pub type_: ComponentType,
    pub description: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DatasourceForCanister {
    pub location: DatasourceLocationForCanister,
    pub method: DatasourceMethod,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DatasourceLocationForCanister {
    pub id: String,
}
impl Default for DatasourceLocationForCanister {
    fn default() -> Self {
        Self {
            id: "sample_snapshot_indexer_evm".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DatasourceMethod {
    pub identifier: String,
    pub interface: Option<String>,
    pub args: Vec<serde_yaml::Value>,
}
impl Default for DatasourceMethod {
    fn default() -> Self {
        Self {
            identifier: "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                .to_string(),
            interface: None,
            args: vec![],
        }
    }
}

pub struct GeneratedCodes {
    pub lib: String,
    pub types: Option<String>,
}

/// Common Trait for Manifest of Data Processing Component
pub trait ComponentManifest: std::fmt::Debug {
    /// Get a structure representing the Component from the manifest
    /// Note: assuming use of serde_yaml
    fn load(path: &str) -> anyhow::Result<Self>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        let mut file = OpenOptions::new()
            .read(true)
            .open(Path::new(path))
            .with_context(|| format!("Failed to open file: {}", path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("Failed to read file: {}", path))?;
        let contents =
            load_env(&contents).with_context(|| format!("Failed to load env for {}", path))?;
        let data: Self = serde_yaml::from_str(&contents)
            .with_context(|| format!("Failed to parse yaml: {}", path))?;
        Ok(data)
    }

    /// Get a structure representing the Component with id from the manifest
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self>
    where
        Self: Sized + serde::de::DeserializeOwned;

    /// Output Component Manifest as yaml format string
    /// Note: assuming use of serde_yaml
    fn to_str_as_yaml(&self) -> anyhow::Result<String>
    where
        Self: Sized;

    fn yaml_str_with_configs(&self, yaml: String, schema_file_name: String) -> String {
        let url_prefix =
            "https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/"
                .to_string();
        let schema_url = format!("{}{}{}", url_prefix, schema_file_name, ".json");

        format!("# yaml-language-server: $schema={}\n{}", schema_url, yaml)
    }
    /// Check Manifest format/value
    fn validate_manifest(&self) -> anyhow::Result<()>;

    /// Get the Component's Type
    fn component_type(&self) -> ComponentType;

    /// Get the Component's Metadata
    fn id(&self) -> Option<String>;

    /// Get the Component's Metadata
    fn metadata(&self) -> &ComponentMetadata;

    /// Get DestinationType if Destination is defined
    fn destination_type(&self) -> Option<DestinationType>;

    /// Get custom tags to add to wasm metadata
    fn custom_tags(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Get the required interface for this component
    /// ex: abi (.json), candid (.candid)
    fn required_interface(&self) -> Option<String>;

    /// Sources of data provided by this component
    fn get_sources(&self) -> Sources;

    /// Generate bindings with candid files
    fn generate_bindings(&self) -> anyhow::Result<BTreeMap<String, String>> {
        Ok(BTreeMap::new())
    }

    /// Label of this component on which the component depends
    /// NOTE: only used by alhorithm_lens
    fn dependencies(&self) -> Vec<String> {
        vec![]
    }
    fn generate_dependency_accessors(&self) -> anyhow::Result<GeneratedCodes> {
        bail!("not implemented")
    }

    fn timer_settings(&self) -> Option<TimerSettings>;

    /// Get the Component's cycle management settings
    fn cycle_managements(&self) -> CycleManagements;
}

pub fn custom_tags_interval_sec(interval_sec: u32) -> (String, String) {
    (
        "chainsight:intervalSec".to_string(),
        interval_sec.to_string(),
    )
}
/// Structure for determining Indexer Type
#[derive(Deserialize)]
pub struct ComponentTypeInManifest {
    pub metadata: ComponentMetadata,
}
impl ComponentTypeInManifest {
    pub fn load(component_manifest_path: &str) -> anyhow::Result<ComponentTypeInManifest> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(Path::new(component_manifest_path))
            .with_context(|| format!("Failed to open file: {}", component_manifest_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("Failed to read file: {}", component_manifest_path))?;
        let data: Self = serde_yaml::from_str(&contents)
            .with_context(|| format!("Failed to parse yaml: {}", component_manifest_path))?;
        Ok(data)
    }

    /// Determine Component Type from Component Manifest
    pub fn determine_type(component_manifest_path: &str) -> anyhow::Result<ComponentType> {
        let data = Self::load(component_manifest_path)?;
        Ok(data.metadata.type_)
    }
}

/// Settings for Periodic tasks
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimerSettings {
    /// Interval in seconds
    pub interval_sec: u32,
    /// Delay in seconds, if not set, default to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_sec: Option<u32>,
    /// Whether to round execution timing by interval or not
    /// If interval is 60min and this is true, the task will be executed at 00:00, 01:00, 02:00, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_round_start_timing: Option<bool>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CycleManagementManifest {
    pub initial_supply: Option<u128>,
    pub refueling_amount: Option<u128>,
    pub refueling_threshold: Option<u128>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CycleManagementsManifest {
    pub refueling_interval: Option<u64>,
    pub vault_intial_supply: Option<u128>,
    pub indexer: Option<CycleManagementManifest>,
    pub db: Option<CycleManagementManifest>,
    pub proxy: Option<CycleManagementManifest>,
}

impl From<CycleManagementsManifest> for CycleManagements {
    fn from(val: CycleManagementsManifest) -> CycleManagements {
        let indexer = val.indexer.unwrap_or_default();
        let db = val.db.unwrap_or_default();
        let proxy = val.proxy.unwrap_or_default();
        CycleManagements {
            refueling_interval: val.refueling_interval.unwrap_or(86400),
            vault_intial_supply: val.vault_intial_supply.unwrap_or(500_000_000_000),
            indexer: CycleManagement {
                initial_supply: indexer.initial_supply.unwrap_or(0),
                refueling_amount: indexer.refueling_amount.unwrap_or(3_000_000_000_000),
                refueling_threshold: indexer.refueling_threshold.unwrap_or(1_500_000_000_000),
            },
            db: CycleManagement {
                initial_supply: db.initial_supply.unwrap_or(1_000_000_000_000),
                refueling_amount: db.refueling_amount.unwrap_or(1_000_000_000_000),
                refueling_threshold: db.refueling_threshold.unwrap_or(500_000_000_000),
            },
            proxy: CycleManagement {
                initial_supply: proxy.initial_supply.unwrap_or(500_000_000_000),
                refueling_amount: proxy.refueling_amount.unwrap_or(500_000_000_000),
                refueling_threshold: proxy.refueling_threshold.unwrap_or(500_000_000_000),
            },
        }
    }
}
