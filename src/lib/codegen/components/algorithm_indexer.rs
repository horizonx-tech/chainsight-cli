use std::collections::HashMap;

use chainsight_cdk::{
    config::components::{AlgorithmIndexerConfig, AlgorithmInputType, CommonConfig},
    initializer::CycleManagements,
};
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{canisters, scripts},
    types::{ComponentType, Network},
    utils::serializer::ordered_map,
};

use super::common::{
    custom_tags_interval_sec, ComponentManifest, ComponentMetadata, CycleManagementsManifest,
    GeneratedCodes, SourceType, Sources,
};

/// Component Manifest: Algorithm Indexer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmIndexerComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: AlgorithmIndexerDatasource,
    pub output: Vec<AlgorithmIndexerOutput>,
    pub interval: u32,
    pub cycles: Option<CycleManagementsManifest>,
}

impl From<AlgorithmIndexerComponentManifest>
    for chainsight_cdk::config::components::AlgorithmIndexerConfig
{
    fn from(val: AlgorithmIndexerComponentManifest) -> Self {
        AlgorithmIndexerConfig {
            common: CommonConfig {
                canister_name: val.id.unwrap(),
            },
            indexing: chainsight_cdk::indexer::IndexingConfig {
                start_from: val.datasource.from,
                chunk_size: None,
            },
            input: chainsight_cdk::config::components::AlgorithmIndexerInput {
                method_name: val.datasource.method,
                response_type: val.datasource.input.name,
                source_type: val.datasource.source_type,
            },
        }
    }
}

impl AlgorithmIndexerComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: AlgorithmIndexerDatasource,
        output: Vec<AlgorithmIndexerOutput>,
        interval: u32,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::AlgorithmIndexer,
                description: description.to_owned(),
                tags: Some(vec!["Ethereum".to_string(), "Account".to_string()]),
            },
            datasource,
            output,
            interval,
            cycles: None,
        }
    }
}
impl ComponentManifest for AlgorithmIndexerComponentManifest {
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self> {
        let manifest = Self::load(path)?;
        Ok(Self {
            id: Some(id.to_owned()),
            ..manifest
        })
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "algorithm_indexer".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::algorithm_indexer::validate_manifest(self)
    }

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: canisters::algorithm_indexer::generate_codes(self)?,
            types: None,
        })
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::algorithm_indexer::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AlgorithmIndexer
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
            lib: canisters::algorithm_indexer::generate_app(self)?,
            types: None,
        })
    }

    fn get_sources(&self) -> Sources {
        Sources {
            source_type: SourceType::Chainsight,
            source: self.datasource.clone().principal,
            attributes: HashMap::new(),
        }
    }
    fn custom_tags(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        let (interval_key, interval_val) = custom_tags_interval_sec(self.interval);
        res.insert(interval_key, interval_val);
        res
    }
    fn cycle_managements(&self) -> CycleManagements {
        self.cycles.clone().unwrap_or_default().into()
    }
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum InputType {
    U128,
    U64,
    U32,
    U16,
    U8,
    String,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]

pub struct InputStruct {
    pub name: String,
    #[serde(serialize_with = "ordered_map")]
    pub fields: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmIndexerOutput {
    pub name: String,
    #[serde(serialize_with = "ordered_map")]
    pub fields: HashMap<String, String>,
    pub output_type: AlgorithmOutputType,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AlgorithmOutputType {
    #[serde(rename = "key_values")]
    KeyValues,
    #[serde(rename = "key_value")]
    KeyValue,
}

impl Default for AlgorithmIndexerOutput {
    fn default() -> Self {
        let mut sample_fields = HashMap::new();
        sample_fields.insert("address".to_string(), "String".to_string());
        Self {
            name: "Account".to_string(),
            fields: sample_fields,
            output_type: AlgorithmOutputType::KeyValue,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmIndexerDatasource {
    pub principal: String,
    pub input: InputStruct,
    pub from: u64,
    pub method: String,
    pub source_type: AlgorithmInputType,
}

impl Default for AlgorithmIndexerDatasource {
    fn default() -> Self {
        let mut sample_fields = HashMap::new();
        sample_fields.insert("from".to_string(), "String".to_string());
        sample_fields.insert("to".to_string(), "String".to_string());
        sample_fields.insert("value".to_string(), "String".to_string());

        Self {
            principal: "be2us-64aaa-aaaaa-qaabq-cai".to_string(),
            input: InputStruct {
                name: "Transfer".to_string(),
                fields: sample_fields,
            },
            source_type: AlgorithmInputType::EventIndexer,
            method: "proxy_call".to_string(),
            from: 17660942,
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
    label: sample_algorithm_indexer
    type: algorithm_indexer
    description: Description
    tags: 
    - Ethereum
    - Account
datasource:
    principal: ahw5u-keaaa-aaaaa-qaaha-cai
    from: 17660942
    input:
        name: Transfer
        fields:
            from: String
            to: String
            value: String
    method: proxy_call
    source_type: event_indexer
output:
- name: SampleOutput
  output_type: key_value
  fields:
    result: String
    value: String
interval: 3600
"#;

        let result = serde_yaml::from_str::<AlgorithmIndexerComponentManifest>(yaml);
        let component = result.unwrap();

        assert_eq!(
            component,
            AlgorithmIndexerComponentManifest {
                id: None,
                version: "v1".to_string(),
                metadata: ComponentMetadata {
                    label: "sample_algorithm_indexer".to_string(),
                    type_: ComponentType::AlgorithmIndexer,
                    description: "Description".to_string(),
                    tags: Some(vec!["Ethereum".to_string(), "Account".to_string()])
                },
                datasource: AlgorithmIndexerDatasource {
                    input: InputStruct {
                        name: "Transfer".to_string(),
                        fields: HashMap::from([
                            ("from".to_string(), "String".to_string()),
                            ("to".to_string(), "String".to_string()),
                            ("value".to_string(), "String".to_string()),
                        ])
                    },
                    principal: "ahw5u-keaaa-aaaaa-qaaha-cai".to_string(),
                    from: 17660942,
                    method: "proxy_call".to_string(),
                    source_type: AlgorithmInputType::EventIndexer
                },
                output: vec!(AlgorithmIndexerOutput {
                    name: "SampleOutput".to_string(),
                    fields: HashMap::from([
                        ("result".to_string(), "String".to_string()),
                        ("value".to_string(), "String".to_string()),
                    ]),
                    output_type: AlgorithmOutputType::KeyValue
                }),
                interval: 3600,
                cycles: None,
            }
        );
        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/algorithm_indexer.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(yaml).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_outputs() {
        let id = "sample_algorithm_indexer".to_string();
        let manifest = AlgorithmIndexerComponentManifest {
            id: Some(id.clone()),
            version: "v1".to_string(),
            metadata: ComponentMetadata {
                label: "Sample Algorithm Indexer".to_string(),
                type_: ComponentType::AlgorithmIndexer,
                description: "Description".to_string(),
                tags: Some(vec!["Ethereum".to_string(), "Account".to_string()]),
            },
            datasource: AlgorithmIndexerDatasource {
                input: InputStruct {
                    name: "Transfer".to_string(),
                    fields: HashMap::from([
                        ("from".to_string(), "String".to_string()),
                        ("to".to_string(), "String".to_string()),
                        ("value".to_string(), "String".to_string()),
                    ]),
                },
                principal: "ahw5u-keaaa-aaaaa-qaaha-cai".to_string(),
                from: 17660942,
                method: "proxy_call".to_string(),
                source_type: AlgorithmInputType::EventIndexer,
            },
            output: vec![AlgorithmIndexerOutput {
                name: "SampleOutput".to_string(),
                fields: HashMap::from([
                    ("result".to_string(), "String".to_string()),
                    ("value".to_string(), "String".to_string()),
                ]),
                output_type: AlgorithmOutputType::KeyValue,
            }],
            interval: 3600,
            cycles: None,
        };

        let snap_prefix = "snapshot__algorithm_indexer";
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
            format!("{}__scripts", &snap_prefix),
            &manifest.generate_scripts(Network::Local).unwrap()
        );
    }
}
