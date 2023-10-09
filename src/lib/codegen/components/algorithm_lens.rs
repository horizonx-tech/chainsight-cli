use std::collections::HashMap;

use candid::Principal;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{canisters, scripts},
    types::{ComponentType, Network},
};

use super::common::{ComponentManifest, ComponentMetadata, SourceType, Sources};

/// Component Manifest: Algorithm Lens
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmLensComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: AlgorithmLensDataSource,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LensTargets {
    pub identifiers: Vec<String>,
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
impl ComponentManifest for AlgorithmLensComponentManifest {
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
    ) -> anyhow::Result<TokenStream> {
        canisters::algorithm_lens::generate_codes(self)
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::algorithm_lens::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AlgorithmLens
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

    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream> {
        canisters::algorithm_lens::generate_app(self)
    }

    fn get_sources(&self) -> Sources {
        Sources {
            source_type: SourceType::Chainsight,
            source: Principal::anonymous().to_string(),
            attributes: HashMap::new(),
        }
    }
    fn custom_tags(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn dependencies(&self) -> Vec<String> {
        self.datasource
            .methods
            .iter()
            .map(|e| e.label.clone())
            .collect()
    }
    fn generate_dependency_accessors(&self) -> anyhow::Result<TokenStream> {
        canisters::algorithm_lens::generate_dependencies_accessor(self)
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
    pub label: String,
    pub identifier: String,
    pub candid_file_path: String,
}

impl Default for AlgorithmLensDataSource {
    fn default() -> Self {
        Self {
            methods: vec![AlgorithmLensDataSourceMethod {
                label: "sample_snapshot_indexer_icp".to_string(),
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
    - label: last_snapshot_value
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
                        label: "last_snapshot_value".to_string(),
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
    fn test_snapshot_outputs() {
        let manifest = AlgorithmLensComponentManifest {
            id: None,
            version: "v1".to_string(),
            metadata: ComponentMetadata {
                label: "sample_algorithm_lens".to_string(),
                type_: ComponentType::AlgorithmLens,
                description: "Description".to_string(),
                tags: Some(vec!["Ethereum".to_string(), "Account".to_string()]),
            },
            datasource: AlgorithmLensDataSource {
                methods: vec![AlgorithmLensDataSourceMethod {
                    label: "last_snapshot_value".to_string(),
                    identifier: "get_last_snapshot : () -> (Snapshot)".to_string(),
                    candid_file_path: "interfaces/sample.did".to_string(),
                }],
            },
        };

        assert_display_snapshot!(SrcString::from(
            &manifest.generate_codes(Option::None).unwrap()
        ));
        assert_display_snapshot!(SrcString::from(
            &manifest.generate_user_impl_template().unwrap()
        ));
        assert_display_snapshot!(SrcString::from(
            &manifest.generate_dependency_accessors().unwrap()
        ));
        assert_display_snapshot!(&manifest.generate_scripts(Network::Local).unwrap());
    }
}
