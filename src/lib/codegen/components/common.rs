use std::{fs::OpenOptions, path::Path, io::Read};

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::types::ComponentType;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum DatasourceType {
    #[serde(rename = "canister")]
    Canister,
    #[serde(rename = "contract")]
    Contract,
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum DestinactionType {
    #[serde(rename = "uint256")]
    Uint256Oracle,
    #[serde(rename = "string")]
    StringOracle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Datasource {
    #[serde(rename = "type")]
    pub type_: DatasourceType,
    pub id: String,
    pub method: DatasourceMethod
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethod {
    pub identifier: String,
    pub interface: Option<String>,
    pub args: Vec<serde_yaml::Value>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethodCustomStruct {
    pub name: String,
    pub fields: Vec<DatasourceMethodCustomStructField>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethodCustomStructField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String
}
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct DatasourceMethodCustomType {
    pub name: String,
    pub types: Vec<String>
}

impl Datasource {
    // temp: use Default trait
    pub fn default_contract() -> Self {
        Self {
            type_: DatasourceType::Contract,
            id: "0000000000000000000000000000000000000000".to_string(), // temp
            method: DatasourceMethod {
                identifier: "totalSupply():(uint256)".to_string(),
                interface: Some("ERC20.json".to_string()),
                args: vec![],
            },
        }
    }
    pub fn new_contract(
        identifier: String,
        interface: Option<String>,
    ) -> Self {
        Self {
            type_: DatasourceType::Contract,
            id: "0000000000000000000000000000000000000000".to_string(), // temp
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
            },
        }
    }
    
    // temp: use Default trait
    pub fn default_canister() -> Self {
        Self {
            type_: DatasourceType::Canister,
            id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(), // temp
            method: DatasourceMethod {
                identifier: "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })".to_string(),
                interface: None,
                args: vec![],
            },
        }
    }

    pub fn new_canister(
        identifier: String,
        interface: Option<String>,
    ) -> Self {
        Self {
            type_: DatasourceType::Canister,
            id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(), // temp
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
            },
        }
    }
}

/// Determine indexer type from manifest
pub trait ComponentManifest: std::fmt::Debug {
    fn load(path: &str) -> anyhow::Result<Self> where Self: Sized;
    fn to_str_as_yaml(&self) -> anyhow::Result<String> where Self: Sized;
    fn validate_manifest(&self) -> anyhow::Result<()>;
    fn generate_codes(&self) -> anyhow::Result<TokenStream>;
}

#[derive(Deserialize)]
pub struct CommonComponentManifest {
    pub version: String,
    #[serde(rename = "type")]
    pub type_: ComponentType,
    pub label: String,
}
pub fn get_type_from_manifest(component_manifest_path: &str) -> anyhow::Result<ComponentType> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&Path::new(component_manifest_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: CommonComponentManifest = serde_yaml::from_str(&contents)?;
    Ok(data.type_)
}
