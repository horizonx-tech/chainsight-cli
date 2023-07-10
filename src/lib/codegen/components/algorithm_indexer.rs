use std::{collections::HashMap, fs::OpenOptions, io::Read, path::Path};

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{canisters, scripts},
    types::{ComponentType, Network},
};

use super::common::{ComponentManifest, ComponentMetadata};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmIndexerComponentManifest {
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: AlgorithmIndexerDatasource,
    pub output: AlgorithmIndexerOutput,
    pub interval: u32,
}

impl AlgorithmIndexerComponentManifest {
    pub fn new(
        label: &str,
        description: &str,
        version: &str,
        datasource: AlgorithmIndexerDatasource,
        output: AlgorithmIndexerOutput,
        interval: u32,
    ) -> Self {
        Self {
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::AlgorithmIndexer,
                description: description.to_owned(),
            },
            datasource,
            output,
            interval,
        }
    }
}
impl ComponentManifest for AlgorithmIndexerComponentManifest {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new().read(true).open(Path::new(path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = serde_yaml::from_str(&contents)?;
        Ok(data)
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(yaml)
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
        ComponentType::EventIndexer
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
}

impl Default for AlgorithmIndexerOutput {
    fn default() -> Self {
        let mut sample_fields = HashMap::new();
        sample_fields.insert("value".to_string(), "u128".to_string());
        Self {
            name: "SampleOutput".to_string(),
            fields: sample_fields,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AlgorithmIndexerDatasource {
    pub printipal: String,
    pub input: InputStruct,
    pub from: u64,
}

impl Default for AlgorithmIndexerDatasource {
    fn default() -> Self {
        let mut sample_fields = HashMap::new();
        sample_fields.insert("result".to_string(), "String".to_string());

        Self {
            printipal: "".to_string(),
            input: InputStruct {
                name: "SampleSource".to_string(),
                fields: sample_fields,
            },
            from: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_pj_algorithm_indexer
    type: algorithm_indexer
    description: Description
datasource:
    printipal: ahw5u-keaaa-aaaaa-qaaha-cai
    from: 0
    input:
        name: Transfer
        fields:
            from: String
            to: String
            value: String
output:
    name: SampleOutput
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
                    label: "sample_pj_algorithm_indexer".to_string(),
                    type_: ComponentType::AlgorithmIndexer,
                    description: "Description".to_string(),
                },
                datasource: AlgorithmIndexerDatasource {
                    input: InputStruct {
                        name: "Transfer".to_string(),
                        fields: input_types
                    },
                    printipal: "ahw5u-keaaa-aaaaa-qaaha-cai".to_string(),
                    from: 0
                },
                output: AlgorithmIndexerOutput {
                    name: "SampleOutput".to_string(),
                    fields: output_types
                },
                interval: 3600
            }
        );
    }
}
