use std::collections::{BTreeMap, HashMap};

use chainsight_cdk::{
    config::components::{CommonConfig, LensParameter, LensTargets},
    initializer::CycleManagements,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::codegen::{canisters, components::common::SourceType, scripts},
    types::{ComponentType, Network},
};

use super::{
    codegen::CodeGenerator,
    common::{
        custom_tags_interval_sec, ComponentManifest, ComponentMetadata, CycleManagementsManifest,
        DatasourceForCanister, DestinationType, GeneratedCodes, Sources, TimerSettings,
    },
    utils::{
        generate_method_identifier, generate_types_from_bindings, get_did_by_component_id,
        is_lens_with_args,
    },
};

/// Component Manifest: Snapshot Indexer ICP
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotIndexerICPComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: DatasourceForCanister,
    pub is_target_component: Option<bool>,
    pub lens_targets: Option<LensTargets>,
    pub timer_settings: TimerSettings,
    pub cycles: Option<CycleManagementsManifest>,
}

impl SnapshotIndexerICPComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: DatasourceForCanister,
        interval: u32,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::SnapshotIndexerICP,
                description: description.to_owned(),
                tags: Some(vec![
                    "ERC-20".to_string(),
                    "Ethereum".to_string(),
                    "DAI".to_string(),
                ]),
            },
            datasource,
            timer_settings: TimerSettings {
                interval_sec: interval,
                delay_sec: None,
                is_round_start_timing: None,
            },
            is_target_component: None,
            lens_targets: None,
            cycles: None,
        }
    }
}
impl From<SnapshotIndexerICPComponentManifest>
    for chainsight_cdk::config::components::SnapshotIndexerICPConfig
{
    fn from(val: SnapshotIndexerICPComponentManifest) -> Self {
        let SnapshotIndexerICPComponentManifest {
            id,
            datasource: DatasourceForCanister {
                method, location, ..
            },
            is_target_component,
            lens_targets,
            ..
        } = val;

        let lens_parameter = if lens_targets.is_some() {
            let interface = if method.interface.is_some() {
                method.interface.clone()
            } else {
                get_did_by_component_id(&location.id)
            };
            let identifier = generate_method_identifier(&method.identifier, &interface)
                .unwrap_or_else(|e| panic!("{}", e.to_string()));

            let with_args = is_lens_with_args(identifier);
            Some(LensParameter { with_args })
        } else {
            None
        };

        Self {
            common: CommonConfig {
                canister_name: id.clone().unwrap(),
            },
            method_identifier: method.identifier,
            is_target_component: is_target_component.unwrap_or(true), // NOTE: default target is canister in the platform (= component)
            lens_parameter,
        }
    }
}

pub struct SnapshotIndexerICPCodeGenerator {
    manifest: SnapshotIndexerICPComponentManifest,
}
impl SnapshotIndexerICPCodeGenerator {
    pub fn new(manifest: SnapshotIndexerICPComponentManifest) -> Self {
        Self { manifest }
    }
}
impl CodeGenerator for SnapshotIndexerICPCodeGenerator {
    fn generate_code(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::snapshot_indexer_icp::generate_codes(&self.manifest)?;

        let types = generate_types_from_bindings(
            &self.manifest.id.clone().unwrap(),
            &self.manifest.datasource.method.identifier,
        )?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
        })
    }
    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::snapshot_indexer_icp::generate_scripts(&self.manifest, network)
    }
    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::snapshot_indexer_icp::generate_app(&self.manifest)?;

        let types = generate_types_from_bindings(
            &self.manifest.id.clone().unwrap(),
            &self.manifest.datasource.method.identifier,
        )?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
        })
    }
    fn manifest(&self) -> Box<dyn ComponentManifest> {
        Box::new(self.manifest.clone())
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
        let (interval_key, interval_val) =
            custom_tags_interval_sec(self.timer_settings.interval_sec);
        res.insert(interval_key, interval_val);
        res
    }
    fn generate_bindings(&self) -> anyhow::Result<BTreeMap<String, String>> {
        let SnapshotIndexerICPComponentManifest {
            datasource: DatasourceForCanister {
                location, method, ..
            },
            ..
        } = self;

        let interface = if method.interface.is_some() {
            method.interface.clone()
        } else {
            get_did_by_component_id(&location.id)
        };

        let identifier = generate_method_identifier(&method.identifier, &interface)?;
        let lib = identifier.compile()?;

        Ok(BTreeMap::from([("lib".to_string(), lib)]))
    }
    fn cycle_managements(&self) -> CycleManagements {
        self.cycles.clone().unwrap_or_default().into()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::{
        codegen::components::common::{DatasourceLocationForCanister, DatasourceMethod},
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
timer_settings:
    interval_sec: 3600
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
                datasource: DatasourceForCanister {
                    location: DatasourceLocationForCanister {
                        id: "datasource_canister_id".to_string(),
                    },
                    method: DatasourceMethod {
                        identifier:
                            "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                                .to_owned(),
                        interface: None,
                        args: vec![]
                    }
                },
                is_target_component: None,
                lens_targets: None,
                timer_settings: TimerSettings {
                    interval_sec: 3600,
                    delay_sec: None,
                    is_round_start_timing: None,
                },
                cycles: None,
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
            datasource: DatasourceForCanister {
                location: DatasourceLocationForCanister {
                    id: "datasource_canister_id".to_string(),
                },
                method: DatasourceMethod {
                    identifier:
                        "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                            .to_owned(),
                    interface: None,
                    args: vec![],
                },
            },
            is_target_component: None,
            lens_targets: None,
            timer_settings: TimerSettings {
                interval_sec: 3600,
                delay_sec: None,
                is_round_start_timing: None,
            },
            cycles: None,
        };

        let snap_prefix = "snapshot__snapshot_indexer_icp";
        let generated_codes = SnapshotIndexerICPCodeGenerator::new(manifest.clone())
            .generate_code(Option::None)
            .unwrap();
        assert_display_snapshot!(
            format!("{}__canisters_lib", &snap_prefix),
            SrcString::from(generated_codes.lib)
        );
        assert_display_snapshot!(
            format!("{}__canisters_types", &snap_prefix),
            generated_codes.types.unwrap()
        );

        let generated_user_impl_template = SnapshotIndexerICPCodeGenerator::new(manifest.clone())
            .generate_user_impl_template()
            .unwrap();

        assert_display_snapshot!(
            format!("{}__logics_lib", &snap_prefix),
            SrcString::from(generated_user_impl_template.lib)
        );
        assert_display_snapshot!(
            format!("{}__logics_types", &snap_prefix),
            generated_user_impl_template.types.unwrap()
        );

        assert_display_snapshot!(
            format!("{}__scripts", &snap_prefix),
            &SnapshotIndexerICPCodeGenerator::new(manifest)
                .generate_scripts(Network::Local)
                .unwrap()
        );
    }
}
