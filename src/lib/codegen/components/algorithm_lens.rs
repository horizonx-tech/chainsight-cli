use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use candid::Principal;
use chainsight_cdk::{config::components::CommonConfig, convert::candid::CanisterMethodIdentifier};
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{canisters, scripts},
    types::{ComponentType, Network},
};

use super::common::{ComponentManifest, ComponentMetadata, GeneratedCodes, SourceType, Sources};

/// Component Manifest: Algorithm Lens
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmLensComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: AlgorithmLensDataSource,
}

impl AlgorithmLensComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: AlgorithmLensDataSource,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::AlgorithmLens,
                description: description.to_owned(),
                tags: Some(vec!["Ethereum".to_string(), "Account".to_string()]),
            },
            datasource,
        }
    }
}
impl From<AlgorithmLensComponentManifest>
    for chainsight_cdk::config::components::AlgorithmLensConfig
{
    fn from(val: AlgorithmLensComponentManifest) -> Self {
        Self {
            common: CommonConfig {
                canister_name: val.id.clone().unwrap(),
                monitor_duration: 60,
            },
            target_count: val.datasource.methods.len(),
        }
    }
}

impl ComponentManifest for AlgorithmLensComponentManifest {
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self> {
        let manifest = Self::load(path)?;
        Ok(Self {
            id: Some(id.to_owned()),
            ..manifest
        })
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "algorithm_lens".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::algorithm_lens::validate_manifest(self)
    }

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: canisters::algorithm_lens::generate_codes(self)?,
            types: None,
        })
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::algorithm_lens::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AlgorithmLens
    }

    fn id(&self) -> Option<String> {
        self.id.clone()
    }

    fn metadata(&self) -> &ComponentMetadata {
        &self.metadata
    }

    fn destination_type(&self) -> Option<super::common::DestinationType> {
        None
    }

    fn required_interface(&self) -> Option<String> {
        None
    }

    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: canisters::algorithm_lens::generate_app(self)?,
            types: None,
        })
    }

    fn get_sources(&self) -> Sources {
        Sources {
            source_type: SourceType::Chainsight,
            source: Principal::anonymous().to_string(), // TEMP?
            attributes: HashMap::new(),
        }
    }
    fn custom_tags(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn generate_bindings(&self) -> anyhow::Result<BTreeMap<String, String>> {
        let AlgorithmLensComponentManifest {
            datasource: AlgorithmLensDataSource { methods, .. },
            ..
        } = self;

        let mut bindings: BTreeMap<String, String> = BTreeMap::new();
        for method in methods {
            let mod_name = method.id.to_string();
            let codes = if method.candid_file_path.is_empty() {
                let identifier = CanisterMethodIdentifier::new(&method.identifier)?;
                identifier.compile()
            } else {
                let did_str = fs::read_to_string(&method.candid_file_path)?;
                let identifier =
                    CanisterMethodIdentifier::new_with_did(&method.identifier, did_str)?;
                identifier.compile()
            };
            bindings.insert(mod_name, codes);
        }

        let lib = bindings
            .keys()
            .map(|v| format!(r#"pub mod {};"#, v))
            .collect::<Vec<String>>()
            .join("\n");
        bindings.insert("lib".to_string(), lib);

        Ok(bindings)
    }

    fn dependencies(&self) -> Vec<String> {
        self.datasource
            .methods
            .iter()
            .map(|e| e.id.clone())
            .collect()
    }
    fn generate_dependency_accessors(&self) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::algorithm_lens::generate_dependencies_accessor(self)?;
        Ok(GeneratedCodes { lib, types: None })
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AlgorithmLensOutputType {
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "primitive")]
    Primitive,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmLensDataSource {
    pub methods: Vec<AlgorithmLensDataSourceMethod>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmLensDataSourceMethod {
    pub id: String,
    pub identifier: String,
    pub candid_file_path: String, // TODO: to Optional
}

impl Default for AlgorithmLensDataSource {
    fn default() -> Self {
        Self {
            methods: vec![AlgorithmLensDataSourceMethod {
                id: "sample_snapshot_indexer_icp".to_string(),
                identifier: "get_last_snapshot : () -> (Snapshot)".to_string(),
                candid_file_path: "interfaces/sample.did".to_string(),
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::test_utils::SrcString;

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_algorithm_lens
    type: algorithm_lens
    description: Description
    tags: 
    - Ethereum
    - Account
datasource:
    methods:
    - id: last_snapshot
      identifier: 'get_last_snapshot : () -> (Snapshot)'
      candid_file_path: "interfaces/sample.did"
output:
    name: SampleOutput
    type: struct
    fields:
      result: String
      value: String
"#;

        let result = serde_yaml::from_str::<AlgorithmLensComponentManifest>(yaml);
        let component = result.unwrap();
        let mut output_types = HashMap::new();
        output_types.insert("result".to_string(), "String".to_string());
        output_types.insert("value".to_string(), "String".to_string());

        assert_eq!(
            component,
            AlgorithmLensComponentManifest {
                id: None,
                version: "v1".to_string(),
                metadata: ComponentMetadata {
                    label: "sample_algorithm_lens".to_string(),
                    type_: ComponentType::AlgorithmLens,
                    description: "Description".to_string(),
                    tags: Some(vec!["Ethereum".to_string(), "Account".to_string()])
                },
                datasource: AlgorithmLensDataSource {
                    methods: vec![AlgorithmLensDataSourceMethod {
                        id: "last_snapshot".to_string(),
                        identifier: "get_last_snapshot : () -> (Snapshot)".to_string(),
                        candid_file_path: "interfaces/sample.did".to_string(),
                    }],
                },
            }
        );
        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/algorithm_lens.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(yaml).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_manifests() {
        let yaml = r#"
version: v1
metadata:
    label: sample_algorithm_lens
    type: algorithm_lens
    description: Description
    tags:
    - Ethereum
    - Account
datasource:
    methods:
    - id: last_snapshot
      identifier: 'get_last_snapshot_1 : () -> (Snapshot)'
      candid_file_path: "interfaces/sample_1.did"
    - id: last_snapshot
      identifier: 'get_last_snapshot_2 : () -> (text)'
      candid_file_path: "interfaces/sample_2.did"
"#;
        let result = serde_yaml::from_str::<AlgorithmLensComponentManifest>(yaml).unwrap();
        let err = result.validate_manifest().unwrap_err();
        assert_eq!(
            err.to_string(),
            "duplicated id found in datasource.methods: last_snapshot"
        );
    }

    #[test]
    fn test_snapshot_outputs() {
        let manifest = AlgorithmLensComponentManifest {
            id: Some("sample_algorithm_lens".to_string()),
            version: "v1".to_string(),
            metadata: ComponentMetadata {
                label: "Sample Algorithm Lens".to_string(),
                type_: ComponentType::AlgorithmLens,
                description: "Description".to_string(),
                tags: Some(vec!["Ethereum".to_string(), "Account".to_string()]),
            },
            datasource: AlgorithmLensDataSource {
                methods: vec![AlgorithmLensDataSourceMethod {
                    id: "last_snapshot_value".to_string(),
                    identifier:
                        "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                            .to_string(),
                    candid_file_path: "interfaces/sample.did".to_string(),
                }],
            },
        };

        let snap_prefix = "snapshot__algorithm_lens";
        let generated_codes = manifest.generate_codes(Option::None).unwrap();
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
            format!("{}__accessors_lib", &snap_prefix),
            SrcString::from(manifest.generate_dependency_accessors().unwrap().lib)
        );

        assert_display_snapshot!(
            format!("{}__scripts", &snap_prefix),
            &manifest.generate_scripts(Network::Local).unwrap()
        );
    }
}
