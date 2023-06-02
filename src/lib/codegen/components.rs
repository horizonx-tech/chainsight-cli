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
    pub destinations: Vec<DestinationField>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Datasource {
    pub type_: DatasourceType,
    pub id: String,
    pub method: DatasourceMethod
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasourceMethod {
    pub interface: String,
    pub identifier: String,
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
    pub network_id: u16,
    pub oracle: String,
    pub key: String,
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
    pub fn new(component_label: &str, version: &str, datasource: Datasource, destinations: Vec<DestinationField>) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Relayer,
            label: component_label.to_owned(),
            datasource,
            destinations,
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
    pub fn new_contract(
        interface: Option<String>,
        identifier: Option<String>,
        response_type: Option<String>,
        custom_struct: Option<Vec<DatasourceMethodCustomStruct>>,
        custom_type: Option<Vec<DatasourceMethodCustomType>>,
    ) -> Self {
        let interface = interface.unwrap_or("ERC20.json".to_string());
        let identifier = identifier.unwrap_or("totalSupply()".to_string());
        let response_type = response_type.unwrap_or("ic_web3::types::U256".to_string());
        Self {
            type_: DatasourceType::Contract,
            id: "0000000000000000000000000000000000000000".to_string(), // temp
            method: DatasourceMethod {
                interface,
                identifier,
                args: vec![],
                response_types: vec![response_type],
                custom_struct,
                custom_type,
            },
        }
    }
    
    // temp
    pub fn new_canister(
        interface: Option<String>,
        identifier: Option<String>,
        response_type: Option<String>,
        custom_struct: Option<Vec<DatasourceMethodCustomStruct>>,
        custom_type: Option<Vec<DatasourceMethodCustomType>>,
    ) -> Self {
        let interface = interface.unwrap_or("Interface.candid".to_string());
        let identifier = identifier.unwrap_or("get_last_snapshot()".to_string());
        let response_type = response_type.unwrap_or("ResponseType".to_string());
        let custom_struct = custom_struct.unwrap_or(vec![
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
        }]);
        let custom_type = custom_type.unwrap_or(vec![
            DatasourceMethodCustomType {
                name: "ResponseValueType".to_string(),
                types: vec!["String".to_string()],
            },
        ]);
        Self {
            type_: DatasourceType::Canister,
            id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(), // temp
            method: DatasourceMethod {
                interface,
                identifier,
                args: vec![],
                response_types: vec![response_type],
                custom_struct: Some(custom_struct),
                custom_type: Some(custom_type),
            },
        }
    }
}

impl DestinationField {
    // temp
    pub fn new(network_id: u16, interval: u32) -> Self {
        Self {
            network_id,
            oracle: "0000000000000000000000000000000000000000".to_owned(), // temp
            key: "5fd4d8f912a7be9759c2d039168362925359f379c0e92d4bdbc7534806faa5bb".to_owned(), // temp
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
