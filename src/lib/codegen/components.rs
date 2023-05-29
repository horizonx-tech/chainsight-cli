use std::{fs::OpenOptions, io::Read, path::Path};

use serde::{Serialize, Deserialize};

use crate::types::ComponentType;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum DatasourceType {
    #[serde(rename = "canister")]
    Canister,
    #[serde(rename = "contract")]
    Contract,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SnapshotComponentManifest {
    pub version: String,
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub interval: u32
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RelayerComponentManifest {
    pub version: String,
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub destinations: Vec<DestinationField>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Datasource {
    pub type_: DatasourceType,
    pub id: String,
    pub method: DatasourceMethod
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DatasourceMethod {
    pub identifier: String,
    pub args: Vec<String>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DestinationField {
    pub network_id: u16,
    pub oracle: String,
    pub key: String,
    pub interval: u32
}

pub trait ComponentManifest: std::fmt::Debug {
    fn load(path: &str) -> anyhow::Result<Self> where Self: Sized;
    fn to_str_as_yaml(&self) -> anyhow::Result<String> where Self: Sized;
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
}

impl Datasource {
    // temp
    pub fn new_canister() -> Self {
        Self {
            type_: DatasourceType::Canister,
            id: "xxx-xxx-xxx".to_owned(),
            method: DatasourceMethod {
                identifier: "total_supply()".to_owned(),
                args: vec![]
            },
        }
    }

    // temp
    pub fn new_contract() -> Self {
        Self {
            type_: DatasourceType::Contract,
            id: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_owned(), // temp
            method: DatasourceMethod {
                identifier: "totalSupply()".to_owned(), // temp
                args: vec![]
            },
        }
    }
}

impl DestinationField {
    // temp
    pub fn new(network_id: u16, interval: u32) -> Self {
        Self {
            network_id,
            oracle: "0xaaaaaaaaaaaaaaaaaaaaa".to_owned(), // temp
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
