use std::{collections::HashMap, fs::OpenOptions, io::Read, path::Path};

use anyhow::bail;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    lib::utils::env::load_env,
    types::{ComponentType, Network},
};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum CanisterIdType {
    #[serde(rename = "canister_name")]
    CanisterName,
    #[serde(rename = "principal_id")]
    PrincipalId,
}
#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum DestinationType {
    #[serde(rename = "uint256")]
    Uint256Oracle,
    #[serde(rename = "uint128")]
    Uint128Oracle,
    #[serde(rename = "uint64")]
    Uint64Oracle,
    #[serde(rename = "string")]
    StringOracle,
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Datasource {
    pub location: DatasourceLocation,
    pub method: DatasourceMethod,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DatasourceLocation {
    pub id: String,
    pub args: DatasourceLocationArgs,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DatasourceLocationArgs {
    // for contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_url: Option<String>,

    // for canister
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<CanisterIdType>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DatasourceMethod {
    pub identifier: String,
    pub interface: Option<String>,
    pub args: Vec<serde_yaml::Value>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotStorage {
    pub with_timestamp: bool,
}

impl Datasource {
    pub fn default_contract() -> Self {
        Self::new_contract(
            "totalSupply():(uint256)".to_string(),
            Some("ERC20.json".to_string()),
            None,
        )
    }
    pub fn new_contract(
        identifier: String,
        interface: Option<String>,
        location: Option<DatasourceLocation>,
    ) -> Self {
        let location = location.unwrap_or_else(DatasourceLocation::default_contract);
        Self {
            location,
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
            },
        }
    }

    pub fn default_canister(ident_with_ts: bool) -> Self {
        let identifier = if ident_with_ts {
            "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
        } else {
            "get_last_snapshot_value : () -> (text)"
        }
        .to_string();

        Self::new_canister(identifier, None, None)
    }

    pub fn new_canister(
        identifier: String,
        interface: Option<String>,
        location: Option<DatasourceLocation>,
    ) -> Self {
        let location = location.unwrap_or_else(DatasourceLocation::default_canister);
        Self {
            location,
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
            },
        }
    }
}

impl DatasourceLocation {
    pub fn default_contract() -> Self {
        Self::new_contract(
            "6b175474e89094c44da98b954eedeac495271d0f".to_string(), // DAI token
            1,
            "https://mainnet.infura.io/v3/${INFURA_MAINNET_RPC_URL}".to_string(),
        )
    }

    pub fn new_contract(id: String, network_id: u32, rpc_url: String) -> Self {
        Self {
            id,
            args: DatasourceLocationArgs {
                network_id: Some(network_id),
                rpc_url: Some(rpc_url),
                id_type: None,
            },
        }
    }

    pub fn default_canister() -> Self {
        Self::new_canister(
            "sample_snapshot_indexer_evm".to_string(),
            CanisterIdType::CanisterName,
        )
    }

    pub fn new_canister(id: String, id_type: CanisterIdType) -> Self {
        Self {
            id,
            args: DatasourceLocationArgs {
                network_id: None,
                rpc_url: None,
                id_type: Some(id_type),
            },
        }
    }
}

impl SnapshotStorage {
    pub fn new(with_timestamp: bool) -> Self {
        Self { with_timestamp }
    }
}
impl Default for SnapshotStorage {
    fn default() -> Self {
        Self::new(true)
    }
}

/// Common Trait for Manifest of Data Processing Component
pub trait ComponentManifest: std::fmt::Debug {
    /// Get a structure representing the Component from the manifest
    /// Note: assuming use of serde_yaml
    fn load(path: &str) -> anyhow::Result<Self>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        let mut file = OpenOptions::new().read(true).open(Path::new(path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let contents = load_env(&contents)?;
        let data: Self = serde_yaml::from_str(&contents)?;
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

    /// Generate canister codes representing Component from Component Manifest
    fn generate_codes(
        &self,
        interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<TokenStream>;

    /// Generate a script from Component Manifest containing commands to run the Component
    fn generate_scripts(&self, network: Network) -> anyhow::Result<String>;

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

    /// Template code to be added/modified by user
    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream>;

    /// Sources of data provided by this component
    fn get_sources(&self) -> Sources;

    /// Label of this component on which the component depends
    /// NOTE: only used by alhorithm_lens
    fn dependencies(&self) -> Vec<String> {
        vec![]
    }
    fn generate_dependency_accessors(&self) -> anyhow::Result<TokenStream> {
        bail!("not implemented")
    }
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
            .open(Path::new(component_manifest_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = serde_yaml::from_str(&contents)?;
        Ok(data)
    }

    /// Determine Component Type from Component Manifest
    pub fn determine_type(component_manifest_path: &str) -> anyhow::Result<ComponentType> {
        let data = Self::load(component_manifest_path)?;
        Ok(data.metadata.type_)
    }
}
