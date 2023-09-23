use std::{collections::HashMap, fs::OpenOptions, io::Read, path::Path};

use anyhow::bail;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::codegen::{canisters, scripts},
    types::{ComponentType, Network},
};

use super::common::{
    custom_tags_interval_sec, ComponentManifest, ComponentMetadata, SourceType, Sources,
};

/// Component Manifest: Event Indexer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventIndexerComponentManifest {
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: EventIndexerDatasource,
    pub interval: u32,
}

impl EventIndexerComponentManifest {
    pub fn new(
        label: &str,
        description: &str,
        version: &str,
        datasource: EventIndexerDatasource,
        interval: u32,
    ) -> Self {
        Self {
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::EventIndexer,
                description: description.to_owned(),
                tags: Some(vec![
                    "Ethereum".to_string(),
                    "ERC-20".to_string(),
                    "Transfer".to_string(),
                ]),
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
        Ok(self.yaml_str_with_configs(yaml, "event_indexer".to_string()))
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

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::event_indexer::generate_scripts(self, network)
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
    fn get_sources(&self) -> Sources {
        #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
        struct Attributes {
            chain_id: u64,
            event_name: String,
            contract_type: String,
        }
        let contract_type = self.clone().datasource.contract_type.unwrap_or_else(|| {
            self.datasource
                .event
                .interface
                .clone()
                .map(|s| s.replace(".json", ""))
                .unwrap()
        });

        let mut attr = HashMap::new();
        attr.insert(
            "chain_id".to_string(),
            json!(self.datasource.network.chain_id),
        );
        attr.insert(
            "event_name".to_string(),
            json!(self.datasource.event.identifier),
        );
        attr.insert("contract_type".to_string(), json!(contract_type));

        Sources {
            source_type: SourceType::Evm,
            source: self.datasource.clone().id,
            attributes: attr,
        }
    }

    fn required_interface(&self) -> Option<String> {
        self.datasource.event.interface.clone()
    }
    fn user_impl_required(&self) -> bool {
        false
    }
    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream> {
        bail!("not implemented")
    }
    fn custom_tags(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        let (interval_key, interval_val) = custom_tags_interval_sec(self.interval);
        res.insert(interval_key, interval_val);
        res
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventIndexerDatasource {
    pub id: String,
    pub event: EventIndexerEventDefinition,
    pub network: SourceNetwork,
    pub from: u64,
    pub contract_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SourceNetwork {
    pub rpc_url: String,
    pub chain_id: u64,
}

impl EventIndexerDatasource {
    pub fn new(
        id: String,
        event: EventIndexerEventDefinition,
        network: SourceNetwork,
        from: u64,
        contract_type: Option<String>,
    ) -> Self {
        Self {
            id,
            event,
            network,
            from,
            contract_type,
        }
    }

    pub fn default() -> Self {
        Self {
            id: "0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string(),
            event: EventIndexerEventDefinition::new(
                "Transfer".to_string(),
                Some("ERC20.json".to_string()),
            ),
            network: SourceNetwork {
                rpc_url: "https://mainnet.infura.io/v3/<YOUR_KEY>".to_string(),
                chain_id: 1,
            },
            from: 17660942,
            contract_type: Some("ERC-20".to_string()),
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
    use jsonschema::JSONSchema;

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_pj_event_indexer
    type: event_indexer
    description: Description
    tags:
    - Ethereum
    - ERC-20
    - Transfer
datasource:
    id: 0x6B175474E89094C44Da98b954EedeAC495271d0F
    event:
        identifier: Transfer
        interface: ERC20.json
    contract_type: ERC20
    network: 
        rpc_url: https://mainnet.infura.io/v3/<YOUR_KEY>
        chain_id: 1
    from: 17660942
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
                    label: "sample_pj_event_indexer".to_string(),
                    type_: ComponentType::EventIndexer,
                    description: "Description".to_string(),
                    tags: Some(vec![
                        "Ethereum".to_string(),
                        "ERC-20".to_string(),
                        "Transfer".to_string()
                    ])
                },
                datasource: EventIndexerDatasource {
                    id: "0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string(),
                    event: EventIndexerEventDefinition {
                        identifier: "Transfer".to_string(),
                        interface: Some("ERC20.json".to_string())
                    },
                    network: SourceNetwork {
                        rpc_url: "https://mainnet.infura.io/v3/<YOUR_KEY>".to_string(),
                        chain_id: 1,
                    },
                    from: 17660942,
                    contract_type: Some("ERC20".to_string())
                },
                interval: 3600
            }
        );
        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/event_indexer.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(yaml).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }
}
