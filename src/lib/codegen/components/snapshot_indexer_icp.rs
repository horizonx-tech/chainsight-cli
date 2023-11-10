use std::collections::{BTreeMap, HashMap};

use chainsight_cdk::{
    config::components::{CommonConfig, LensTargets},
    convert::candid::{read_did_to_string_without_service, CanisterMethodIdentifier},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::codegen::{canisters, components::common::SourceType, scripts},
    types::{ComponentType, Network},
};

use super::{
    common::{
        custom_tags_interval_sec, ComponentManifest, ComponentMetadata, Datasource,
        DestinationType, GeneratedCodes, Sources, DEFAULT_MONITOR_DURATION_SECS,
    },
    utils::generate_types_from_bindings,
};

/// Component Manifest: Snapshot Indexer ICP
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotIndexerICPComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: Datasource,
    pub lens_targets: Option<LensTargets>,
    pub interval: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LensTarget {
    pub identifiers: Vec<String>,
}

impl SnapshotIndexerICPComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: Datasource,
        interval: u32,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
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
            interval,
            lens_targets: None,
        }
    }
}
impl From<SnapshotIndexerICPComponentManifest>
    for chainsight_cdk::config::components::SnapshotIndexerICPConfig
{
    fn from(val: SnapshotIndexerICPComponentManifest) -> Self {
        let SnapshotIndexerICPComponentManifest {
            id,
            datasource,
            lens_targets,
            ..
        } = val;
        Self {
            common: CommonConfig {
                canister_name: id.clone().unwrap(),
                monitor_duration: DEFAULT_MONITOR_DURATION_SECS,
            },
            method_identifier: datasource.method.identifier,
            lens_targets,
        }
    }
}
impl ComponentManifest for SnapshotIndexerICPComponentManifest {
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self> {
        let manifest = Self::load(path)?;
        Ok(Self {
            id: Some(id.to_owned()),
            ..manifest
        })
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "snapshot_indexer_icp".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::snapshot_indexer_icp::validate_manifest(self)
    }

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::snapshot_indexer_icp::generate_codes(self)?;

        let types = generate_types_from_bindings(
            &self.id.clone().unwrap(),
            &self.datasource.method.identifier,
        )?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
        })
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::snapshot_indexer_icp::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::SnapshotIndexerICP
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
        None
    }

    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::snapshot_indexer_icp::generate_app(self)?;

        Ok(GeneratedCodes { lib, types: None })
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

    fn generate_bindings(&self) -> anyhow::Result<BTreeMap<String, String>> {
        let SnapshotIndexerICPComponentManifest {
            datasource: Datasource { method, .. },
            ..
        } = self;
        let interface = method.interface.clone();
        let lib = if let Some(path) = interface {
            let did_str = read_did_to_string_without_service(path)?;
            let identifier = CanisterMethodIdentifier::new_with_did(&method.identifier, did_str)?;
            identifier.compile()?
        } else {
            let identifier = CanisterMethodIdentifier::new(&method.identifier)?;
            identifier.compile()?
        };

        Ok(BTreeMap::from([("lib".to_string(), lib)]))
    }
}

#[cfg(test)]
mod tests {
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
    label: sample_snapshot_indexer_icp
    type: snapshot_indexer_icp
    description: Description
    tags:
    - ERC-20
    - Ethereum
datasource:
    location:
        id: datasource_canister_id
    method:
        identifier: 'get_last_snapshot : () -> (record { value : text; timestamp : nat64 })'
        args: []
interval: 3600
        "#;

        let result = serde_yaml::from_str::<SnapshotIndexerICPComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            SnapshotIndexerICPComponentManifest {
                id: None,
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
                    ),
                    method: DatasourceMethod {
                        identifier:
                            "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                                .to_owned(),
                        interface: None,
                        args: vec![]
                    }
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
    fn test_snapshot_outputs() {
        let manifest = SnapshotIndexerICPComponentManifest {
            id: Some("sample_snapshot_indexer_icp".to_owned()),
            version: "v1".to_owned(),
            metadata: ComponentMetadata {
                label: "sample_snapshot_indexer_icp".to_owned(),
                type_: ComponentType::SnapshotIndexerICP,
                description: "Description".to_string(),
                tags: Some(vec!["ERC-20".to_string(), "Ethereum".to_string()]),
            },
            datasource: Datasource {
                location: DatasourceLocation::new_canister("datasource_canister_id".to_string()),
                method: DatasourceMethod {
                    identifier:
                        "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                            .to_owned(),
                    interface: None,
                    args: vec![],
                },
            },
            lens_targets: None,
            interval: 3600,
        };

        let snap_prefix = "snapshot__snapshot_indexer_icp";
        let generated_codes = manifest.generate_codes(Option::None).unwrap();
        assert_display_snapshot!(
            format!("{}__canisters_lib", &snap_prefix),
            SrcString::from(generated_codes.lib)
        );
        assert_display_snapshot!(
            format!("{}__canisters_types", &snap_prefix),
            generated_codes.types.unwrap()
        );

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
