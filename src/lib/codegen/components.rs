use std::{fs::OpenOptions, io::Read, path::Path};

use proc_macro2::TokenStream;
use serde::{Serialize, Deserialize};

use crate::types::ComponentType;

use super::canisters;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum DatasourceType {
    #[serde(rename = "canister")]
    Canister,
    #[serde(rename = "contract")]
    Contract,
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum DestinactionType {
    #[serde(rename = "uint256")]
    Uint256Oracle,
    #[serde(rename = "string")]
    StringOracle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapshotComponentManifest {
    pub version: String,
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub interval: u32
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerComponentManifest {
    pub version: String,
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub destination: DestinationField, // TODO: multiple destinations
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Datasource {
    pub type_: DatasourceType,
    pub id: String,
    pub method: DatasourceMethod
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethod {
    pub identifier: String,
    pub interface: Option<String>,
    pub args: Vec<DatasourceMethodArg>,
    pub response_types: Vec<String>,
    pub custom_struct: Option<Vec<DatasourceMethodCustomStruct>>,
    pub custom_type: Option<Vec<DatasourceMethodCustomType>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethodArg {
    pub type_: String,
    pub value: serde_yaml::Value,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethodCustomStruct {
    pub name: String,
    pub fields: Vec<DatasourceMethodCustomStructField>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethodCustomStructField {
    pub name: String,
    pub type_: String
}
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct DatasourceMethodCustomType {
    pub name: String,
    pub types: Vec<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DestinationField {
    pub network_id: u32,
    pub type_: DestinactionType,
    pub oracle_address: String,
    pub rpc_url: String,
    pub interval: u32
}

pub trait ComponentManifest: std::fmt::Debug {
    fn load(path: &str) -> anyhow::Result<Self> where Self: Sized;
    fn to_str_as_yaml(&self) -> anyhow::Result<String> where Self: Sized;
    fn generate_codes(&self) -> anyhow::Result<TokenStream>;
}

impl SnapshotComponentManifest {
    pub fn new(component_label: &str, version: &str, datasource: Datasource, interval: u32) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Snapshot,
            label: component_label.to_owned(),
            datasource,
            interval,
        }
    }
}
impl ComponentManifest for SnapshotComponentManifest {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&Path::new(path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = serde_yaml::from_str(&contents)?;
        Ok(data)
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(yaml)
    }

    fn generate_codes(&self) -> anyhow::Result<TokenStream> {
        canisters::generate_snapshot_codes(self)
    }
}
impl RelayerComponentManifest {
    pub fn new(component_label: &str, version: &str, datasource: Datasource, destination: DestinationField) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Relayer,
            label: component_label.to_owned(),
            datasource,
            destination,
        }
    }
}
impl ComponentManifest for RelayerComponentManifest {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&Path::new(path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = serde_yaml::from_str(&contents)?;
        Ok(data)
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(yaml)
    }

    fn generate_codes(&self) -> anyhow::Result<TokenStream> {
        canisters::generate_relayer_codes(self)
    }
}

impl Datasource {
    // temp
    pub fn default_contract() -> Self {
        Self {
            type_: DatasourceType::Contract,
            id: "0000000000000000000000000000000000000000".to_string(), // temp
            method: DatasourceMethod {
                identifier: "totalSupply()".to_string(),
                interface: Some("ERC20.json".to_string()),
                args: vec![],
                response_types: vec!["ic_web3::types::U256".to_string()],
                custom_struct: None,
                custom_type: None,
            },
        }
    }
    pub fn new_contract(
        identifier: String,
        interface: Option<String>,
        response_type: String,
        custom_struct: Option<Vec<DatasourceMethodCustomStruct>>,
        custom_type: Option<Vec<DatasourceMethodCustomType>>,
    ) -> Self {
        Self {
            type_: DatasourceType::Contract,
            id: "0000000000000000000000000000000000000000".to_string(), // temp
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
                response_types: vec![response_type],
                custom_struct,
                custom_type,
            },
        }
    }
    
    // temp
    pub fn default_canister() -> Self {
        Self {
            type_: DatasourceType::Canister,
            id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(), // temp
            method: DatasourceMethod {
                identifier: "get_last_snapshot()".to_string(),
                interface: None,
                args: vec![],
                response_types: vec!["ResponseType".to_string()],
                custom_struct: Some(vec![
                    DatasourceMethodCustomStruct {
                        name: "ResponseType".to_string(),
                        fields: vec![
                            DatasourceMethodCustomStructField {
                                name: "value".to_string(),
                                type_: "ResponseValueType".to_string(),
                            },
                            DatasourceMethodCustomStructField {
                                name: "timestamp".to_string(),
                                type_: "u64".to_string(),
                            },
                    ],
                }]),
                custom_type: Some(vec![
                    DatasourceMethodCustomType {
                        name: "ResponseValueType".to_string(),
                        types: vec!["String".to_string()],
                    },
                ]),
            },
        }
    }

    pub fn new_canister(
        identifier: String,
        interface: Option<String>,
        response_type: String,
        custom_struct: Option<Vec<DatasourceMethodCustomStruct>>,
        custom_type: Option<Vec<DatasourceMethodCustomType>>,
    ) -> Self {
        Self {
            type_: DatasourceType::Canister,
            id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(), // temp
            method: DatasourceMethod {
                identifier,
                interface,
                args: vec![],
                response_types: vec![response_type],
                custom_struct,
                custom_type,
            },
        }
    }
}

impl DestinationField {
    pub fn default() -> Self {
        // temp: polygon mumbai, Uint256Oracle
        Self::new(
            80001,
            DestinactionType::Uint256Oracle,
            "0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
            "https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>".to_string(),
            3600
        )
    }

    pub fn new(network_id: u32, destination_type: DestinactionType, oracle_address: String, rpc_url: String, interval: u32) -> Self {
        Self {
            network_id,
            type_: destination_type,
            oracle_address,
            rpc_url,
            interval,
        }
    }
}

#[derive(Deserialize)]
pub struct CommonComponentManifest {
    pub version: String,
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
