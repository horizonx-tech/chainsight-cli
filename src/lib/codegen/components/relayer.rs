use std::{collections::HashMap, fs::OpenOptions, io::Read, path::Path};

use anyhow::Ok;
use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{
        canisters, components::common::custom_tags_interval_sec, oracle::get_oracle_address,
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
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: Datasource,
    pub destination: DestinationField, // TODO: multiple destinations
    pub interval: u32,
    pub lens_targets: Option<LensTargets>,
}

impl RelayerComponentManifest {
    pub fn new(
        label: &str,
        description: &str,
        version: &str,
        datasource: Datasource,
        destination: DestinationField,
        interval: u32,
    ) -> Self {
        Self {
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

    fn metadata(&self) -> &ComponentMetadata {
        &self.metadata
    }

    fn destination_type(&self) -> Option<DestinationType> {
        Some(self.destination.type_)
    }

    fn required_interface(&self) -> Option<String> {
        self.datasource.method.interface.clone()
    }
    fn user_impl_required(&self) -> bool {
        true
    }
    fn get_sources(&self) -> Sources {
        Sources {
            source: self.datasource.clone().location.id,
            source_type: SourceType::Chainsight,
            attributes: HashMap::new(),
        }
    }
    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream> {
        Ok(quote! {
            use crate::{CallCanisterResponse};
            pub type CallCanisterArgs = ();
            pub fn call_args() -> CallCanisterArgs {
                todo!()
            }
            pub fn filter(_: &CallCanisterResponse) -> bool {
                true
            }
        })
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
            "https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::codegen::components::common::{
        CanisterIdType, DatasourceLocation, DatasourceMethod, DatasourceType,
    };

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
metadata:
    label: sample_pj_relayer
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
    oracle_address: 0539a0EF8e5E60891fFf0958A059E049e43020d9
    rpc_url: https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>
interval: 3600
        "#;

        let result = serde_yaml::from_str::<RelayerComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            RelayerComponentManifest {
                version: "v1".to_string(),
                metadata: ComponentMetadata {
                    label: "sample_pj_relayer".to_string(),
                    type_: ComponentType::Relayer,
                    description: "Description".to_string(),
                    tags: Some(vec!["Oracle".to_string(), "snapshot".to_string()])
                },
                datasource: Datasource {
                    type_: DatasourceType::Canister,
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
                    oracle_address: "0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                    rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>".to_string(),
                },
                lens_targets: None,
                interval: 3600
            }
        );
    }
}
