use std::collections::HashMap;

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::codegen::{canisters, components::common::SourceType, scripts},
    types::{ComponentType, Network},
};

use super::{
    algorithm_lens::LensTargets,
    common::{
        custom_tags_interval_sec, ComponentManifest, ComponentMetadata, Datasource,
        DestinationType, SnapshotStorage, Sources,
    },
};

/// Component Manifest: Snapshot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotIndexerICPComponentManifest {
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: Datasource,
    pub storage: SnapshotStorage,
    pub interval: u32,
    pub lens_targets: Option<LensTargets>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LensTarget {
    pub identifiers: Vec<String>,
}

impl SnapshotIndexerICPComponentManifest {
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
                type_: ComponentType::SnapshotIndexerICP, // temp
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
            lens_targets: None,
        }
    }
}
impl ComponentManifest for SnapshotIndexerICPComponentManifest {
    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "snapshot_indexer".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::snapshot_indexer_icp::validate_manifest(self)
    }

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<TokenStream> {
        canisters::snapshot_indexer_icp::generate_codes(self)
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::snapshot_indexer_icp::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::SnapshotIndexerICP
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

    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream> {
        canisters::snapshot_indexer_icp::generate_app(self)
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
        if self.lens_targets.is_some() {
            let targets = self.lens_targets.clone().unwrap().identifiers;
            attr.insert("sources".to_string(), json!(targets));
        }

        attr.insert("function_name".to_string(), json!(method_identifier));
        Sources {
            source: self.datasource.location.id.clone(),
            source_type: SourceType::Chainsight,
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

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::{
        codegen::components::common::{CanisterIdType, DatasourceLocation, DatasourceMethod},
        test_utils::SrcString,
    };

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_snapshot_indexer_icp
    type: snapshot_indexer_icp
    description: Description
    tags:
    - ERC-20
    - Ethereum
datasource:
    location:
        id: datasource_canister_id
        args:
            id_type: canister_name
    method:
        identifier: 'get_last_snapshot : () -> (record { value : text; timestamp : nat64 })'
        args: []
storage:
    with_timestamp: true
interval: 3600
        "#;

        let result = serde_yaml::from_str::<SnapshotIndexerICPComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            SnapshotIndexerICPComponentManifest {
                version: "v1".to_owned(),
                metadata: ComponentMetadata {
                    label: "sample_snapshot_indexer_icp".to_owned(),
                    type_: ComponentType::SnapshotIndexerICP,
                    description: "Description".to_string(),
                    tags: Some(vec!["ERC-20".to_string(), "Ethereum".to_string()])
                },
                datasource: Datasource {
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
                lens_targets: None,
                interval: 3600
            }
        );
        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/snapshot_indexer_icp.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(yaml).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_outputs_icp() {
        let manifest = SnapshotIndexerICPComponentManifest {
            version: "v1".to_owned(),
            metadata: ComponentMetadata {
                label: "sample_snapshot_indexer_icp".to_owned(),
                type_: ComponentType::SnapshotIndexerICP,
                description: "Description".to_string(),
                tags: Some(vec!["ERC-20".to_string(), "Ethereum".to_string()]),
            },
            datasource: Datasource {
                location: DatasourceLocation::new_canister(
                    "datasource_canister_id".to_string(),
                    CanisterIdType::CanisterName,
                ),
                method: DatasourceMethod {
                    identifier:
                        "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                            .to_owned(),
                    interface: None,
                    args: vec![],
                },
            },
            storage: SnapshotStorage {
                with_timestamp: true,
            },
            lens_targets: None,
            interval: 3600,
        };

        assert_display_snapshot!(SrcString::from(
            &manifest.generate_codes(Option::None).unwrap()
        ));
        assert_display_snapshot!(SrcString::from(
            &manifest.generate_user_impl_template().unwrap()
        ));
        assert_display_snapshot!(&manifest.generate_scripts(Network::Local).unwrap());
    }
}
