use std::collections::HashMap;

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{canisters, scripts},
    types::{ComponentType, Network},
};

use super::common::{
    custom_tags_interval_sec, ComponentManifest, ComponentMetadata, SourceType, Sources,
};

/// Component Manifest: Algorithm Indexer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmIndexerComponentManifest {
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: AlgorithmIndexerDatasource,
    pub output: Vec<AlgorithmIndexerOutput>,
    pub interval: u32,
}

impl AlgorithmIndexerComponentManifest {
    pub fn new(
        label: &str,
        description: &str,
        version: &str,
        datasource: AlgorithmIndexerDatasource,
        output: Vec<AlgorithmIndexerOutput>,
        interval: u32,
    ) -> Self {
        Self {
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
        }
    }
}
impl ComponentManifest for AlgorithmIndexerComponentManifest {
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
    ) -> anyhow::Result<TokenStream> {
        canisters::algorithm_indexer::generate_codes(self)
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::algorithm_indexer::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::AlgorithmIndexer
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
        canisters::algorithm_indexer::generate_app(self)
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
    pub fields: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmIndexerOutput {
    pub name: String,
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AlgorithmInputType {
    #[serde(rename = "event_indexer")]
    EventIndexer,
    #[serde(rename = "key_value")]
    KeyValue,
    #[serde(rename = "key_values")]
    KeyValues,
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

    use jsonschema::JSONSchema;

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
        let mut input_types = HashMap::new();
        let mut output_types = HashMap::new();
        input_types.insert("from".to_string(), "String".to_string());
        input_types.insert("to".to_string(), "String".to_string());
        input_types.insert("value".to_string(), "String".to_string());
        output_types.insert("result".to_string(), "String".to_string());
        output_types.insert("value".to_string(), "String".to_string());

        assert_eq!(
            component,
            AlgorithmIndexerComponentManifest {
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
                        fields: input_types
                    },
                    principal: "ahw5u-keaaa-aaaaa-qaaha-cai".to_string(),
                    from: 17660942,
                    method: "proxy_call".to_string(),
                    source_type: AlgorithmInputType::EventIndexer
                },
                output: vec!(AlgorithmIndexerOutput {
                    name: "SampleOutput".to_string(),
                    fields: output_types,
                    output_type: AlgorithmOutputType::KeyValue
                }),
                interval: 3600
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
}
