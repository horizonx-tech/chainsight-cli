use std::{fs::OpenOptions, path::Path, io::Read};

use anyhow::Ok;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{types::ComponentType, lib::codegen::{canisters, oracle::get_oracle_address}};

use super::common::{Datasource, ComponentManifest, DestinactionType};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RelayerComponentManifest {
    pub version: String,
    #[serde(rename = "type")]
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub destination: DestinationField, // TODO: multiple destinations
    pub interval: u32
}

impl RelayerComponentManifest {
    pub fn new(component_label: &str, version: &str, datasource: Datasource, destination: DestinationField, interval: u32) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Relayer,
            label: component_label.to_owned(),
            datasource,
            destination,
            interval
        }
    }
}
impl ComponentManifest for RelayerComponentManifest {
    fn load(path: &str) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&Path::new(path))?;
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

    fn generate_codes(&self, _interface_contract: Option<ethabi::Contract>) -> anyhow::Result<TokenStream> {
        canisters::relayer::generate_codes(self)
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::Relayer
    }

    fn label(&self) -> &str {
        self.label.as_str()
    }

    fn destination_type(&self) -> Option<DestinactionType> {
        Some(self.destination.type_)
    }

    fn required_interface(&self) -> Option<String> {
        self.datasource.method.interface.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DestinationField {
    pub network_id: u32,
    #[serde(rename = "type")]
    pub type_: DestinactionType,
    pub oracle_address: String,
    pub rpc_url: String,
}

impl DestinationField {
    pub fn new(network_id: u32, destination_type: DestinactionType, oracle_address: String, rpc_url: String) -> Self {
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
        let oracle_type = DestinactionType::Uint256Oracle;
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
    use crate::lib::codegen::components::common::{DatasourceType, DatasourceMethod};

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
version: v1
type: relayer
label: sample_pj_relayer
datasource:
    type: canister
    id: xxxxx-xxxxx-xxxxx-xxxxx-xxx
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
                type_: ComponentType::Relayer,
                label: "sample_pj_relayer".to_string(),
                datasource: Datasource {
                    type_: DatasourceType::Canister,
                    // id: "xxxxx-xxxxx-xxxxx-xxxxx-xxx".to_string(),
                    method: DatasourceMethod {
                        identifier: "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })".to_string(),
                        interface: None,
                        args: vec![]
                    }
                },
                destination: DestinationField {
                    network_id: 80001,
                    type_: DestinactionType::Uint256Oracle,
                    oracle_address: "0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                    rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>".to_string(),
                },
                interval: 3600
            }
        );
    }
}
