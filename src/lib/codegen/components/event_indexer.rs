use std::collections::HashMap;

use anyhow::bail;
use chainsight_cdk::initializer::CycleManagements;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::{
        codegen::{canisters, scripts},
        utils::component_ids_manager::ComponentIdsManager,
    },
    types::{ComponentType, Network},
};

use super::{
    codegen::CodeGenerator,
    common::{
        custom_tags_interval_sec, ComponentManifest, ComponentMetadata, CycleManagementsManifest,
        GeneratedCodes, SourceType, Sources, TimerSettings,
    },
};

/// Component Manifest: Event Indexer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventIndexerComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: EventIndexerDatasource,
    pub timer_settings: TimerSettings,
    pub cycles: Option<CycleManagementsManifest>,
}

impl From<EventIndexerComponentManifest>
    for chainsight_cdk::config::components::EventIndexerConfig
{
    fn from(val: EventIndexerComponentManifest) -> Self {
        Self {
            common: chainsight_cdk::config::components::CommonConfig {
                canister_name: val.id.clone().unwrap(),
            },
            def: chainsight_cdk::config::components::EventIndexerEventDefinition {
                identifier: val.datasource.event.identifier,
                abi_file_path: format!(
                    "./__interfaces/{}",
                    val.datasource.event.interface.unwrap()
                ),
            },
        }
    }
}

impl EventIndexerComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: EventIndexerDatasource,
        interval: u32,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
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
            timer_settings: TimerSettings {
                interval_sec: interval,
                delay_sec: None,
                is_round_start_timing: None,
            },
            cycles: None,
        }
    }
}

pub struct EventIndexerCodeGenerator {
    manifest: EventIndexerComponentManifest,
}
impl EventIndexerCodeGenerator {
    pub fn new(manifest: EventIndexerComponentManifest) -> Self {
        Self { manifest }
    }
}
impl CodeGenerator for EventIndexerCodeGenerator {
    fn generate_code(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: canisters::event_indexer::generate_codes(&self.manifest)?,
            types: None,
        })
    }

    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        bail!("not implemented")
    }
    fn manifest(&self) -> Box<dyn ComponentManifest> {
        Box::new(self.manifest.clone())
    }
    fn generate_component_setup_args(
        &self,
        network: &Network,
        _comp_id_mgr: &ComponentIdsManager,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let args = scripts::event_indexer::generate_component_setup_args(&self.manifest, network)?;
        Ok(Some(args))
    }
}
impl ComponentManifest for EventIndexerComponentManifest {
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self> {
        let manifest = Self::load(path)?;
        Ok(Self {
            id: Some(id.to_owned()),
            ..manifest
        })
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "event_indexer".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::event_indexer::validate_manifest(self)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::EventIndexer
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

    fn custom_tags(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        let (interval_key, interval_val) =
            custom_tags_interval_sec(self.timer_settings.interval_sec);
        res.insert(interval_key, interval_val);
        res
    }
    fn timer_settings(&self) -> Option<TimerSettings> {
        Some(self.timer_settings.clone())
    }
    fn cycle_managements(&self) -> CycleManagements {
        self.cycles.clone().unwrap_or_default().into()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventIndexerDatasource {
    pub id: String,
    pub event: EventIndexerEventDefinition,
    pub network: SourceNetwork,
    pub from: u64,
    pub contract_type: Option<String>,
    pub batch_size: Option<u64>,
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
            batch_size: None,
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
                rpc_url: "https://eth.llamarpc.com".to_string(),
                chain_id: 1,
            },
            from: 17660942,
            contract_type: Some("ERC-20".to_string()),
            batch_size: None,
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
    use std::fs::File;

    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::test_utils::SrcString;

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_event_indexer
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
        rpc_url: https://eth.llamarpc.com
        chain_id: 1
    from: 17660942
timer_settings:
    interval_sec: 3600
        "#;

        let result = serde_yaml::from_str::<EventIndexerComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            EventIndexerComponentManifest {
                id: None,
                version: "v1".to_string(),
                metadata: ComponentMetadata {
                    label: "sample_event_indexer".to_string(),
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
                        rpc_url: "https://eth.llamarpc.com".to_string(),
                        chain_id: 1,
                    },
                    from: 17660942,
                    contract_type: Some("ERC20".to_string()),
                    batch_size: None,
                },
                timer_settings: TimerSettings {
                    interval_sec: 3600,
                    delay_sec: None,
                    is_round_start_timing: None,
                },
                cycles: None,
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

    #[test]
    fn test_snapshot_outputs() {
        let manifest = EventIndexerComponentManifest {
            id: Some("sample_event_indexer".to_string()),
            version: "v1".to_string(),
            metadata: ComponentMetadata {
                label: "Sample Event Indexer".to_string(),
                type_: ComponentType::EventIndexer,
                description: "Description".to_string(),
                tags: Some(vec![
                    "Ethereum".to_string(),
                    "ERC-20".to_string(),
                    "Transfer".to_string(),
                ]),
            },
            datasource: EventIndexerDatasource {
                id: "0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string(),
                event: EventIndexerEventDefinition {
                    identifier: "Transfer".to_string(),
                    interface: Some("ERC20.json".to_string()),
                },
                network: SourceNetwork {
                    rpc_url: "https://eth.llamarpc.com".to_string(),
                    chain_id: 1,
                },
                from: 17660942,
                contract_type: Some("ERC20".to_string()),
                batch_size: None,
            },
            timer_settings: TimerSettings {
                interval_sec: 3600,
                delay_sec: None,
                is_round_start_timing: None,
            },
            cycles: None,
        };

        let snap_prefix = "snapshot__event_indexer";
        let abi = File::open("resources/ERC20.json").unwrap();
        let generated_codes = EventIndexerCodeGenerator::new(manifest.clone())
            .generate_code(Option::Some(ethabi::Contract::load(abi).unwrap()))
            .unwrap();
        assert_display_snapshot!(
            format!("{}__canisters_lib", &snap_prefix),
            SrcString::from(generated_codes.lib)
        );
        assert!(generated_codes.types.is_none());

        assert!(EventIndexerCodeGenerator::new(manifest.clone())
            .generate_user_impl_template()
            .is_err());
    }
}
