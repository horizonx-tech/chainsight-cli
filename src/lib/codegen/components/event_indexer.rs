use std::{fs::OpenOptions, io::Read, path::Path};

use anyhow::bail;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::canisters,
    types::{ComponentType, Network},
};

use super::common::{ComponentManifest, ComponentMetadata};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventIndexerComponentManifest {
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: EventIndexerDatasource,
    pub interval: u32,
}

impl EventIndexerComponentManifest {
    pub fn new(
        component_label: &str,
        version: &str,
        datasource: EventIndexerDatasource,
        interval: u32,
    ) -> Self {
        Self {
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: component_label.to_owned(),
                type_: ComponentType::EventIndexer,
            },
            datasource,
            interval,
        }
    }
}
impl ComponentManifest for EventIndexerComponentManifest {
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
        canisters::event_indexer::validate_manifest(self)
    }

    fn generate_codes(
        &self,
        interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<TokenStream> {
        let interface_contract =
            interface_contract.ok_or_else(|| anyhow::anyhow!("interface contract is not found"))?;
        canisters::event_indexer::generate_codes(self, interface_contract)
    }

    fn generate_scripts(&self, _network: Network) -> anyhow::Result<String> {
        bail!("not implemented")
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::EventIndexer
    }

    fn metadata(&self) -> &ComponentMetadata {
        &self.metadata
    }

    fn destination_type(&self) -> Option<super::common::DestinactionType> {
        None
    }

    fn required_interface(&self) -> Option<String> {
        self.datasource.event.interface.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventIndexerDatasource {
    // pub id: String, // NOTE: Currently not in use
    pub event: EventIndexerEventDefinition,
}
impl EventIndexerDatasource {
    pub fn new(_id: String, event: EventIndexerEventDefinition) -> Self {
        Self {
            // id,
            event,
        }
    }

    pub fn default() -> Self {
        Self {
            // id: "0000000000000000000000000000000000000000".to_string(),
            event: EventIndexerEventDefinition::new(
                "Transfer".to_string(),
                Some("ERC20.json".to_string()),
            ),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventIndexerEventDefinition {
    pub identifier: String,
    pub interface: Option<String>,
}
impl EventIndexerEventDefinition {
    pub fn new(identifier: String, interface: Option<String>) -> Self {
        Self {
            identifier,
            interface,
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
type: event_indexer
label: sample_pj_event_indexer
datasource:
    id: 0000000000000000000000000000000000000000
    event:
        identifier: Transfer
        interface: ERC20.json
interval: 3600
        "#;

        let result = serde_yaml::from_str::<EventIndexerComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            EventIndexerComponentManifest {
                version: "v1".to_string(),
                metadata: ComponentMetadata {
                    type_: ComponentType::EventIndexer,
                    label: "sample_pj_event_indexer".to_string(),
                },
                datasource: EventIndexerDatasource {
                    // id: "0000000000000000000000000000000000000000".to_string(),
                    event: EventIndexerEventDefinition {
                        identifier: "Transfer".to_string(),
                        interface: Some("ERC20.json".to_string())
                    }
                },
                interval: 3600
            }
        );
    }
}
