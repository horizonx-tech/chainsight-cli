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
    pub args: Vec<DatasourceMethodArg>,
    pub response: DatasourceResponse,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethodArg {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: serde_yaml::Value,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceResponse {
    #[serde(rename = "type")]
    pub type_: String,
    pub with_timestamp: Option<bool>
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
                identifier: "totalSupply()".to_string(),
                interface: Some("ERC20.json".to_string()),
                args: vec![],
                response: DatasourceResponse {
                    type_: "ic_web3::types::U256".to_string(),
                    with_timestamp: None,
                },
            },
        }
    }
    pub fn new_contract(
        identifier: String,
        interface: Option<String>,
        response: DatasourceResponse,
    ) -> Self {
        Self {
            type_: DatasourceType::Contract,
            id: "0000000000000000000000000000000000000000".to_string(), // temp
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
                response,
            },
        }
    }
    
    // temp: use Default trait
    pub fn default_canister() -> Self {
        Self {
            type_: DatasourceType::Canister,
            id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(), // temp
            method: DatasourceMethod {
                identifier: "get_last_snapshot()".to_string(),
                interface: None,
                args: vec![],
                response: DatasourceResponse {
                    type_: "String".to_string(),
                    with_timestamp: Some(true),
                },
            },
        }
    }

    pub fn new_canister(
        identifier: String,
        interface: Option<String>,
        response: DatasourceResponse,
    ) -> Self {
        Self {
            type_: DatasourceType::Canister,
            id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(), // temp
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
                response,
            },
        }
    }
}

/// Determine indexer type from manifest
pub trait ComponentManifest: std::fmt::Debug {
    fn load(path: &str) -> anyhow::Result<Self> where Self: Sized;
    fn to_str_as_yaml(&self) -> anyhow::Result<String> where Self: Sized;
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
