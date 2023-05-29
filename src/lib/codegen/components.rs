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

#[derive(Deserialize, Serialize)]
pub struct SnapshotComponentManifest {
    version: String,
    type_: ComponentType,
    label: String,
    datasource: Datasource,
    interval: u32
}
#[derive(Deserialize, Serialize)]
pub struct RelayerComponentManifest {
    version: String,
    type_: ComponentType,
    label: String,
    datasource: Datasource,
    destinations: Vec<DestinationField>
}
#[derive(Deserialize, Serialize)]
pub struct Datasource {
    type_: DatasourceType,
    id: String,
    method: DatasourceMethod
}
#[derive(Deserialize, Serialize)]
struct DatasourceMethod {
    identifier: String,
    args: Vec<String>
}
#[derive(Deserialize, Serialize)]
pub struct DestinationField {
    network_id: u16,
    oracle: String,
    key: String,
    interval: u32
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

    pub fn to_str_as_yaml(&self) -> anyhow::Result<String> {
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

    pub fn to_str_as_yaml(&self) -> anyhow::Result<String> {
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