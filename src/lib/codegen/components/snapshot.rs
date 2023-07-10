use std::{fs::OpenOptions, io::Read, path::Path};

use anyhow::bail;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{canisters, scripts},
    types::{ComponentType, Network},
};

use super::common::{ComponentManifest, ComponentMetadata, Datasource, DestinationType};

/// Component Manifest: Snapshot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotComponentManifest {
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: Datasource,
    pub storage: SnapshotStorage,
    pub interval: u32,
}

impl SnapshotComponentManifest {
    pub fn new(
        label: &str,
        description: &str,
        version: &str,
        datasource: Datasource,
        storage: SnapshotStorage,
        interval: u32,
    ) -> Self {
        Self {
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::Snapshot,
                description: description.to_owned(),
            },
            datasource,
            storage,
            interval,
        }
    }
}
impl ComponentManifest for SnapshotComponentManifest {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new().read(true).open(Path::new(path))?;
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

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<TokenStream> {
        canisters::snapshot::generate_codes(self)
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::snapshot::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Snapshot
    }

    fn metadata(&self) -> &ComponentMetadata {
        &self.metadata
    }

    fn destination_type(&self) -> Option<DestinationType> {
        None
    }

    fn required_interface(&self) -> Option<String> {
        self.datasource.method.interface.clone()
    }
    fn user_impl_required(&self) -> bool {
        false
    }
    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream> {
        bail!("Not implemented")
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotStorage {
    pub with_timestamp: bool,
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

#[cfg(test)]
mod tests {
    use crate::lib::codegen::components::common::{
        CanisterIdType, DatasourceLocation, DatasourceMethod, DatasourceType,
    };

    use super::*;

    #[test]
    fn test_to_manifest_struct_for_chain() {
        let yaml = r#"
version: v1
metadata:
    label: sample_pj_snapshot_chain
    type: snapshot
    description: Description
datasource:
    type: contract
    location:
        id: 6b175474e89094c44da98b954eedeac495271d0f
        args:
            network_id: 1
            rpc_url: https://mainnet.infura.io/v3/<YOUR_KEY>
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
                metadata: ComponentMetadata {
                    label: "sample_pj_snapshot_chain".to_owned(),
                    type_: ComponentType::Snapshot,
                    description: "Description".to_string()
                },
                datasource: Datasource {
                    type_: DatasourceType::Contract,
                    location: DatasourceLocation::new_contract(
                        "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                        1,
                        "https://mainnet.infura.io/v3/<YOUR_KEY>".to_string(),
                    ),
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
metadata:
    label: sample_pj_snapshot_icp
    type: snapshot
    description: Description
datasource:
    type: canister
    location:
        id: datasource_canister_id
        args:
            id_type: canister_name
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
                metadata: ComponentMetadata {
                    label: "sample_pj_snapshot_icp".to_owned(),
                    type_: ComponentType::Snapshot,
                    description: "Description".to_string()
                },
                datasource: Datasource {
                    type_: DatasourceType::Canister,
                    location: DatasourceLocation::new_canister(
                        "datasource_canister_id".to_string(),
                        CanisterIdType::CanisterName
                    ),
                    method: DatasourceMethod {
                        identifier:
                            "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                                .to_owned(),
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
