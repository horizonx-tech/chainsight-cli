use std::collections::HashMap;

use chainsight_cdk::config::components::{CommonConfig, LensTargets};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::codegen::{canisters, components::common::SourceType, scripts},
    types::{ComponentType, Network},
};

use super::common::{
    custom_tags_interval_sec, ComponentManifest, ComponentMetadata, Datasource, DestinationType,
    GeneratedCodes, SnapshotStorage, Sources, DEFAULT_MONITOR_DURATION_SECS,
};

/// Component Manifest: Snapshot Indexer EVM
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotIndexerEVMComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: Datasource,
    pub storage: SnapshotStorage,
    pub interval: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LensTarget {
    pub identifiers: Vec<String>,
}

impl SnapshotIndexerEVMComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: Datasource,
        storage: SnapshotStorage,
        interval: u32,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::SnapshotIndexerEVM,
                description: description.to_owned(),
                tags: Some(vec![
                    "ERC-20".to_string(),
                    "Ethereum".to_string(),
                    "DAI".to_string(),
                ]),
            },
            datasource,
            storage,
            interval,
        }
    }
}
impl From<SnapshotIndexerEVMComponentManifest>
    for chainsight_cdk::config::components::SnapshotIndexerEVMConfig
{
    fn from(val: SnapshotIndexerEVMComponentManifest) -> Self {
        let SnapshotIndexerEVMComponentManifest { id, datasource, .. } = val;
        Self {
            common: CommonConfig {
                canister_name: id.clone().unwrap(),
                monitor_duration: DEFAULT_MONITOR_DURATION_SECS,
            },
            method_identifier: datasource.method.identifier,
            method_args: datasource
                .method
                .args
                .into_iter()
                .map(yaml_to_json)
                .collect(),
            abi_file_path: format!("./__interfaces/{}", datasource.method.interface.unwrap()),
        }
    }
}
impl ComponentManifest for SnapshotIndexerEVMComponentManifest {
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self> {
        let manifest = Self::load(path)?;
        Ok(Self {
            id: Some(id.to_owned()),
            ..manifest
        })
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "snapshot_indexer_evm".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::snapshot_indexer_evm::validate_manifest(self)
    }

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: canisters::snapshot_indexer_evm::generate_codes(self)?,
            types: None,
        })
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::snapshot_indexer_evm::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::SnapshotIndexerEVM
    }

    fn id(&self) -> Option<String> {
        self.id.clone()
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

    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: canisters::snapshot_indexer_evm::generate_app(self)?,
            types: None,
        })
    }

    fn get_sources(&self) -> Sources {
        let mut attr = HashMap::new();
        let mut method_identifier = self.datasource.clone().method.identifier;
        if method_identifier.contains(':') {
            method_identifier = method_identifier.split(':').collect::<Vec<&str>>()[0]
                .to_string()
                .replace(' ', "")
                .replace("()", "");
        }

        attr.insert("function_name".to_string(), json!(method_identifier));
        Sources {
            source: self.datasource.location.id.clone(),
            source_type: SourceType::Evm,
            attributes: attr,
        }
    }

    fn custom_tags(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        let (interval_key, interval_val) = custom_tags_interval_sec(self.interval);
        res.insert(interval_key, interval_val);
        res
    }
}

fn yaml_to_json(yaml_value: serde_yaml::Value) -> serde_json::Value {
    // Convert serde_yaml::Value to a JSON string
    let json_str = serde_json::to_string(&yaml_value).unwrap();

    // Convert the JSON string to serde_json::Value
    serde_json::from_str(&json_str).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::{
        codegen::components::common::{DatasourceLocation, DatasourceMethod},
        test_utils::SrcString,
    };

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_snapshot_indexer_evm
    type: snapshot_indexer_evm
    description: Description
    tags:
    - ERC-20
    - Ethereum
datasource:
    location:
        id: 6b175474e89094c44da98b954eedeac495271d0f
        args:
            network_id: 1
            rpc_url: https://mainnet.infura.io/v3/${INFURA_MAINNET_RPC_URL_KEY}
    method:
        identifier: totalSupply():(uint256)
        interface: ERC20.json
        args: []
storage:
    with_timestamp: true
interval: 3600
        "#;

        let result = serde_yaml::from_str::<SnapshotIndexerEVMComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            SnapshotIndexerEVMComponentManifest {
                id: None,
                version: "v1".to_owned(),
                metadata: ComponentMetadata {
                    label: "sample_snapshot_indexer_evm".to_owned(),
                    type_: ComponentType::SnapshotIndexerEVM,
                    description: "Description".to_string(),
                    tags: Some(vec!["ERC-20".to_string(), "Ethereum".to_string()])
                },
                datasource: Datasource {
                    location: DatasourceLocation::new_contract(
                        "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                        1,
                        "https://mainnet.infura.io/v3/${INFURA_MAINNET_RPC_URL_KEY}".to_string(),
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

        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/snapshot_indexer_evm.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(yaml).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_outputs() {
        let manifest = SnapshotIndexerEVMComponentManifest {
            id: Some("sample_snapshot_indexer_evm".to_owned()),
            version: "v1".to_owned(),
            metadata: ComponentMetadata {
                label: "Sample Snapshot Indexer Evm".to_owned(),
                type_: ComponentType::SnapshotIndexerEVM,
                description: "Description".to_string(),
                tags: Some(vec!["ERC-20".to_string(), "Ethereum".to_string()]),
            },
            datasource: Datasource {
                location: DatasourceLocation::new_contract(
                    "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                    1,
                    "https://mainnet.infura.io/v3/${INFURA_MAINNET_RPC_URL_KEY}".to_string(),
                ),
                method: DatasourceMethod {
                    identifier: "totalSupply():(uint256)".to_owned(),
                    interface: Some("ERC20.json".to_string()),
                    args: vec![],
                },
            },
            storage: SnapshotStorage {
                with_timestamp: true,
            },
            interval: 3600,
        };

        let snap_prefix = "snapshot__snapshot_indexer_evm";
        let abi = File::open("resources/ERC20.json").unwrap();
        let generated_codes = manifest
            .generate_codes(Option::Some(ethabi::Contract::load(abi).unwrap()))
            .unwrap();
        assert_display_snapshot!(
            format!("{}__canisters_lib", &snap_prefix),
            SrcString::from(generated_codes.lib)
        );
        assert!(generated_codes.types.is_none());

        let generated_user_impl_template = manifest.generate_user_impl_template().unwrap();
        assert_display_snapshot!(
            format!("{}__logics_lib", &snap_prefix),
            SrcString::from(generated_user_impl_template.lib)
        );
        assert!(generated_user_impl_template.types.is_none());

        assert_display_snapshot!(
            format!("{}__scripts", &snap_prefix),
            &manifest.generate_scripts(Network::Local).unwrap()
        );
    }
}
