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
struct SnapshotComponentManifest {
    version: String,
    type_: ComponentType,
    label: String,
    datasource: Datasource,
    interval: u32
}
#[derive(Deserialize, Serialize)]
struct RelayerComponentManifest {
    version: String,
    type_: ComponentType,
    label: String,
    datasource: Datasource,
    destinations: Vec<DestinationField>
}
#[derive(Deserialize, Serialize)]
struct Datasource {
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
struct DestinationField {
    network_id: u16,
    oracle: String,
    key: String,
    interval: u32
}

// temp
pub fn generate_manifest_for_snapshot(project_name: &str) -> Result<std::string::String, serde_yaml::Error> {
    let data = SnapshotComponentManifest {
        version: "v1".to_owned(),
        type_: ComponentType::Snapshot,
        label: format!("{}-snapshot", project_name).to_owned(),
        datasource: Datasource {
            type_: DatasourceType::Contract,
            id: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_owned(),
            method: DatasourceMethod {
                identifier: "totalSupply()".to_owned(),
                args: vec![]
            },
        },
        interval: 3600,
    };
    serde_yaml::to_string(&data)
}
pub fn generate_manifest_for_relayer(project_name: &str) -> Result<std::string::String, serde_yaml::Error> {
    let data = RelayerComponentManifest {
        version: "v1".to_owned(),
        type_: ComponentType::Relayer,
        label: format!("{}-relayer", project_name).to_owned(),
        datasource: Datasource {
            type_: DatasourceType::Canister,
            id: "xxx-xxx-xxx".to_owned(),
            method: DatasourceMethod {
                identifier: "total_supply()".to_owned(),
                args: vec![]
            },
        },
        destinations: vec![
            DestinationField {
                network_id: 1,
                oracle: "0xaaaaaaaaaaaaaaaaaaaaa".to_owned(),
                key: "5fd4d8f912a7be9759c2d039168362925359f379c0e92d4bdbc7534806faa5bb".to_owned(),
                interval: 3600,
            },
        ],
    };
    serde_yaml::to_string(&data)
}