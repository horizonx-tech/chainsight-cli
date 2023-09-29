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
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: AlgorithmLensDataSource,
    pub output: AlgorithmLensOutput,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LensTargets {
    pub identifiers: Vec<String>,
}
impl AlgorithmLensComponentManifest {
    pub fn new(
        label: &str,
        description: &str,
        version: &str,
        datasource: AlgorithmLensDataSource,
        output: AlgorithmLensOutput,
    ) -> Self {
        Self {
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::AlgorithmLens,
                description: description.to_owned(),
                tags: Some(vec!["Ethereum".to_string(), "Account".to_string()]),
            },
            datasource,
            output,
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

    fn user_impl_required(&self) -> bool {
        true
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
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmLensOutput {
    pub name: Option<String>,
    pub fields: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: AlgorithmLensOutputType,
    pub type_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AlgorithmLensOutputType {
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "primitive")]
    Primitive,
}

impl Default for AlgorithmLensOutput {
    fn default() -> Self {
        let mut sample_fields = HashMap::new();
        sample_fields.insert("address".to_string(), "String".to_string());
        Self {
            name: Some("Account".to_string()),
            fields: Some(sample_fields),
            type_: AlgorithmLensOutputType::Struct,
            type_name: None,
        }
    }
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
                identifier: "get_last_snapshot_value : () -> (SnapshotValue)".to_string(),
                candid_file_path: "interfaces/sample.did".to_string(),
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use jsonschema::JSONSchema;

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
      identifier: 'get_last_snapshot_value : () -> (SnapshotValue)'
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
                        identifier: "get_last_snapshot_value : () -> (SnapshotValue)".to_string(),
                        candid_file_path: "interfaces/sample.did".to_string()
                    }],
                },
                output: AlgorithmLensOutput {
                    name: Some("SampleOutput".to_string()),
                    fields: Some(output_types),
                    type_: AlgorithmLensOutputType::Struct,
                    type_name: None,
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
}
