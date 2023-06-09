use std::{fs::OpenOptions, path::Path, io::Read};

use anyhow::Ok;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{types::ComponentType, lib::codegen::{canisters, oracle::get_oracle_address}};

use super::common::{Datasource, ComponentManifest, DestinactionType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayerComponentManifest {
    pub version: String,
    #[serde(rename = "type")]
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub destination: DestinationField, // TODO: multiple destinations
}

impl RelayerComponentManifest {
    pub fn new(component_label: &str, version: &str, datasource: Datasource, destination: DestinationField) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Relayer,
            label: component_label.to_owned(),
            datasource,
            destination,
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
        canisters::validate_relayer_manifest(self)
    }

    fn generate_codes(&self) -> anyhow::Result<TokenStream> {
        canisters::generate_relayer_codes(self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DestinationField {
    pub network_id: u32,
    #[serde(rename = "type")]
    pub type_: DestinactionType,
    pub oracle_address: String,
    pub rpc_url: String,
    pub interval: u32
}

impl DestinationField {
    pub fn new(network_id: u32, destination_type: DestinactionType, oracle_address: String, rpc_url: String, interval: u32) -> Self {
        Self {
            network_id,
            type_: destination_type,
            oracle_address,
            rpc_url,
            interval,
        }
    }
}
impl Default for DestinationField {
    fn default() -> Self {
        let network_id = 80001; // temp: polygon mumbai
        let oracle_type = DestinactionType::Uint256Oracle;
        Self::new(
            network_id,
            oracle_type,
            get_oracle_address(network_id, oracle_type),
            "https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>".to_string(),
            3600
        )
    }
}