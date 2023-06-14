use std::{fs::OpenOptions, path::Path, io::Read};

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{types::ComponentType, lib::codegen::canisters};

use super::common::{Datasource, ComponentManifest, DestinactionType};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotComponentManifest {
    pub version: String,
    #[serde(rename = "type")]
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub storage: SnapshotStorage,
    pub interval: u32
}

impl SnapshotComponentManifest {
    pub fn new(component_label: &str, version: &str, datasource: Datasource, storage: SnapshotStorage, interval: u32) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Snapshot,
            label: component_label.to_owned(),
            datasource,
            storage,
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

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::snapshot::validate_manifest(self)
    }

    fn generate_codes(&self, _interface_contract: Option<ethabi::Contract>) -> anyhow::Result<TokenStream> {
        canisters::snapshot::generate_codes(self)
    }

    fn label(&self) -> &str {
        self.label.as_str()
    }

    fn destination_type(&self) -> Option<DestinactionType> {
        None
    }

    fn required_interface(&self) -> Option<String> {
        self.datasource.method.interface.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotStorage {
    pub with_timestamp: bool,
}
impl SnapshotStorage {
    pub fn new(with_timestamp: bool) -> Self {
        Self {
            with_timestamp,
        }
    }
}
impl Default for SnapshotStorage {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::codegen::components::common::{DatasourceMethod, DatasourceType};

    use super::*;

    #[test]
    fn test_to_manifest_struct_for_chain() {
        let yaml = r#"
version: v1
type: snapshot
label: sample_pj_snapshot_chain
datasource:
    type: contract
    id: 0000000000000000000000000000000000000000
    method:
        identifier: totalSupply():(uint256)
        interface: ERC20.json
        args: []
storage:
    with_timestamp: true
interval: 3600
        "#;

        let result = serde_yaml::from_str::<SnapshotComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            SnapshotComponentManifest {
                version: "v1".to_owned(),
                type_: ComponentType::Snapshot,
                label: "sample_pj_snapshot_chain".to_owned(),
                datasource: Datasource {
                    type_: DatasourceType::Contract,
                    id: "0000000000000000000000000000000000000000".to_owned(),
                    method: DatasourceMethod {
                        identifier: "totalSupply():(uint256)".to_owned(),
                        interface: Some("ERC20.json".to_string()),
                        args: vec![]
                    }
                },
                storage: SnapshotStorage {
                    with_timestamp: true,
                },
                interval: 3600
            }
        );
    }

    #[test]
    fn test_to_manifest_struct_for_icp() {
        let yaml = r#"
version: v1
type: snapshot
label: sample_pj_snapshot_icp
datasource:
    type: canister
    id: xxxxx-xxxxx-xxxxx-xxxxx-xxx
    method:
        identifier: 'get_last_snapshot : () -> (record { value : text; timestamp : nat64 })'
        interface: null
        args: []
storage:
    with_timestamp: true
interval: 3600
        "#;

        let result = serde_yaml::from_str::<SnapshotComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            SnapshotComponentManifest {
                version: "v1".to_owned(),
                type_: ComponentType::Snapshot,
                label: "sample_pj_snapshot_icp".to_owned(),
                datasource: Datasource {
                    type_: DatasourceType::Canister,
                    id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_owned(),
                    method: DatasourceMethod {
                        identifier: "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })".to_owned(),
                        interface: None,
                        args: vec![]
                    }
                },
                storage: SnapshotStorage {
                    with_timestamp: true,
                },
                interval: 3600
            }
        );
    }
}
