use std::collections::HashMap;

use anyhow::Ok;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::codegen::{
        canisters::{self},
        components::common::custom_tags_interval_sec,
        oracle::get_oracle_address,
        scripts,
    },
    types::{ComponentType, Network},
};

use super::{
    algorithm_lens::LensTargets,
    common::{
        ComponentManifest, ComponentMetadata, Datasource, DestinationType, SourceType, Sources,
    },
};

/// Component Manifest: Relayer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RelayerComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: Datasource,
    pub destination: DestinationField, // TODO: multiple destinations
    pub interval: u32,
    pub lens_targets: Option<LensTargets>,
}

impl RelayerComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: Datasource,
        destination: DestinationField,
        interval: u32,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::Relayer,
                description: description.to_owned(),
                tags: Some(vec!["Oracle".to_string(), "snapshot".to_string()]),
            },
            datasource,
            destination,
            lens_targets: None,
            interval,
        }
    }
}
impl ComponentManifest for RelayerComponentManifest {
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self> {
        let manifest = Self::load(path)?;
        Ok(Self {
            id: Some(id.to_owned()),
            ..manifest
        })
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "relayer".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        canisters::relayer::validate_manifest(self)
    }

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<TokenStream> {
        canisters::relayer::generate_codes(self)
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::relayer::generate_scripts(self, network)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Relayer
    }

    fn id(&self) -> Option<String> {
        self.id.clone()
    }

    fn metadata(&self) -> &ComponentMetadata {
        &self.metadata
    }

    fn destination_type(&self) -> Option<DestinationType> {
        Some(self.destination.type_)
    }

    fn required_interface(&self) -> Option<String> {
        self.datasource.method.interface.clone()
    }

    fn get_sources(&self) -> Sources {
        let mut attributes = HashMap::new();
        if self.lens_targets.is_some() {
            let targets = self.lens_targets.clone().unwrap().identifiers;
            attributes.insert("sources".to_string(), json!(targets));
        }
        Sources {
            source: self.datasource.clone().location.id,
            source_type: SourceType::Chainsight,
            attributes,
        }
    }
    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream> {
        canisters::relayer::generate_app(self)
    }
    fn custom_tags(&self) -> HashMap<String, String> {
        #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
        struct Attributes {
            chain_id: u32,
        }
        #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
        struct Destination {
            destination_type: String,
            destination: String,
            attributes: Attributes,
        }
        let mut res = HashMap::new();
        let dest = Destination {
            destination_type: "evm".to_string(),
            destination: self.destination.oracle_address.clone(),
            attributes: Attributes {
                chain_id: self.destination.network_id,
            },
        };
        res.insert(
            "chainsight:destination".to_string(),
            serde_json::to_string(&dest).unwrap(),
        );
        let (interval_key, interval_val) = custom_tags_interval_sec(self.interval);
        res.insert(interval_key, interval_val);
        res
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DestinationField {
    pub network_id: u32,
    #[serde(rename = "type")]
    pub type_: DestinationType,
    pub oracle_address: String,
    pub rpc_url: String,
}

impl DestinationField {
    pub fn new(
        network_id: u32,
        destination_type: DestinationType,
        oracle_address: String,
        rpc_url: String,
    ) -> Self {
        Self {
            network_id,
            type_: destination_type,
            oracle_address,
            rpc_url,
        }
    }
}
impl Default for DestinationField {
    fn default() -> Self {
        let network_id = 80001; // NOTE: (temp) polygon mumbai
        let oracle_type = DestinationType::Uint256Oracle;
        Self::new(
            network_id,
            oracle_type,
            get_oracle_address(network_id, oracle_type),
            "https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL}".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::{
        codegen::components::common::{CanisterIdType, DatasourceLocation, DatasourceMethod},
        test_utils::SrcString,
    };

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_relayer
    type: relayer
    description: Description
    tags:
    - Oracle
    - snapshot
datasource:
    type: canister
    location:
        id: datasource_canister_id
        args:
            id_type: canister_name
    method:
        identifier: 'get_last_snapshot : () -> (record { value : text; timestamp : nat64 })'
        interface: null
        args: []
destination:
    network_id: 80001
    type: uint256
    oracle_address: 0x0539a0EF8e5E60891fFf0958A059E049e43020d9
    rpc_url: https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL}
interval: 3600
        "#;

        let result = serde_yaml::from_str::<RelayerComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            RelayerComponentManifest {
                id: None,
                version: "v1".to_string(),
                metadata: ComponentMetadata {
                    label: "sample_relayer".to_string(),
                    type_: ComponentType::Relayer,
                    description: "Description".to_string(),
                    tags: Some(vec!["Oracle".to_string(), "snapshot".to_string()])
                },
                datasource: Datasource {
                    location: DatasourceLocation::new_canister(
                        "datasource_canister_id".to_string(),
                        CanisterIdType::CanisterName
                    ),
                    method: DatasourceMethod {
                        identifier:
                            "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                                .to_string(),
                        interface: None,
                        args: vec![]
                    }
                },
                destination: DestinationField {
                    network_id: 80001,
                    type_: DestinationType::Uint256Oracle,
                    oracle_address: "0x0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                    rpc_url: "https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL}"
                        .to_string(),
                },
                lens_targets: None,
                interval: 3600
            }
        );
        let schema =
            serde_json::from_str(include_str!("../../../../resources/schema/relayer.json"))
                .expect("Invalid json");
        let instance = serde_yaml::from_str(yaml).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_outputs() {
        let manifest = RelayerComponentManifest {
            id: Some("sample_relayer".to_string()),
            version: "v1".to_string(),
            metadata: ComponentMetadata {
                label: "Sample Relayer".to_string(),
                type_: ComponentType::Relayer,
                description: "Description".to_string(),
                tags: Some(vec!["Oracle".to_string(), "snapshot".to_string()]),
            },
            datasource: Datasource {
                location: DatasourceLocation::new_canister(
                    "datasource_canister_id".to_string(),
                    CanisterIdType::CanisterName,
                ),
                method: DatasourceMethod {
                    identifier:
                        "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                            .to_string(),
                    interface: None,
                    args: vec![],
                },
            },
            destination: DestinationField {
                network_id: 80001,
                type_: DestinationType::StringOracle,
                oracle_address: "0x0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                rpc_url: "https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL}".to_string(),
            },
            lens_targets: None,
            interval: 3600,
        };

        // FIXME failed to load oracle abi at ./__interfaces/{}.json
        // assert_display_snapshot!(SrcString::from(
        //     &manifest.generate_codes(Option::None).unwrap()
        // ));
        assert_display_snapshot!(SrcString::from(
            &manifest.generate_user_impl_template().unwrap()
        ));
        assert_display_snapshot!(&manifest.generate_scripts(Network::Local).unwrap());
    }
}
