use std::collections::HashMap;

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
            .map(|e| e.id.clone())
            .collect()
    }
    fn generate_dependency_accessors(&self) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::algorithm_lens::generate_dependencies_accessor(self)?;
        let types = generate_accessors_types(&self.datasource.methods)?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
        })
    }
}

fn generate_accessors_types(
    methods: &Vec<AlgorithmLensDataSourceMethod>,
) -> anyhow::Result<String> {
    // OPTIMIZE: this logics
    let mut types: Vec<String> = vec![];
    let req_ty = CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME;
    let res_ty = CanisterMethodIdentifier::RESPONSE_TYPE_NAME;
    types.push("#![allow(dead_code, unused_imports)]".to_string());
    types.push(
        "use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};".to_string(),
    );
    for method in methods {
        let method_identifier = CanisterMethodIdentifier::new(&method.identifier)
            .expect("method_identifier parse error");
        let contents = method_identifier
            .compile()
            .lines()
            .skip(2)
            .map(|line| {
                if line.contains(req_ty) {
                    return line.replace(req_ty, &format!("{}__{}", req_ty, method.id));
                }
                if line.contains(res_ty) {
                    return line.replace(res_ty, &format!("{}__{}", res_ty, method.id));
                }
                return line.to_string();
            })
            .collect::<Vec<_>>();
        types.extend(contents)
    }
    Ok(types.join("\n"))
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
    pub candid_file_path: String,
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
                    identifier: "get_last_snapshot : () -> (Snapshot)".to_string(),
                    candid_file_path: "interfaces/sample.did".to_string(),
                }],
            },
        };

        let generated_codes = manifest.generate_codes(Option::None).unwrap();
        assert_display_snapshot!(SrcString::from(&generated_codes.lib));
        assert!(generated_codes.types.is_none());

        let generated_user_impl_template = manifest.generate_user_impl_template().unwrap();
        assert_display_snapshot!(SrcString::from(&generated_user_impl_template.lib));
        assert!(generated_user_impl_template.types.is_none());

        assert_display_snapshot!(SrcString::from(
            &manifest.generate_dependency_accessors().unwrap().lib
        ));

        assert_display_snapshot!(&manifest.generate_scripts(Network::Local).unwrap());
    }
}
