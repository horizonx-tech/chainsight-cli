use std::collections::{BTreeMap, HashMap};

use anyhow::Ok;
use chainsight_cdk::{
    config::components::{CommonConfig, LensParameter, LensTargets},
    convert::candid::{read_did_to_string_without_service, CanisterMethodIdentifier},
    initializer::CycleManagements,
};
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
    common::{
        ComponentManifest, ComponentMetadata, CycleManagementsManifest, Datasource,
        DestinationType, GeneratedCodes, SourceType, Sources,
    },
    utils::generate_types_from_bindings,
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
    pub cycles: Option<CycleManagementsManifest>,
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
            cycles: None,
        }
    }
}

impl From<RelayerComponentManifest> for chainsight_cdk::config::components::RelayerConfig {
    fn from(val: RelayerComponentManifest) -> Self {
        let oracle_type = match val.destination_type() {
            Some(DestinationType::Uint256) => "uint256".to_string(),
            Some(DestinationType::Uint128) => "uint128".to_string(),
            Some(DestinationType::Uint64) => "uint64".to_string(),
            Some(DestinationType::String) => "string".to_string(),
            _ => panic!("Invalid oracle type"),
        };
        let lens_parameter = if val.lens_targets.is_some() {
            Some(LensParameter { with_args: false }) // todo: consider with_args
        } else {
            None
        };

        Self {
            common: CommonConfig {
                canister_name: val.id.clone().unwrap(),
            },
            destination: val.destination.oracle_address,
            method_identifier: val.datasource.method.identifier,
            oracle_type,
            abi_file_path: "__interfaces/Oracle.json".to_string(),
            lens_parameter,
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
    ) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::relayer::generate_codes(self)?;

        let types = generate_types_from_bindings(
            &self.id.clone().unwrap(),
            &self.datasource.method.identifier,
        )?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
        })
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
        None
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
    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::relayer::generate_app(self)?;

        let types = generate_types_from_bindings(
            &self.id.clone().unwrap(),
            &self.datasource.method.identifier,
        )?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
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
        let oracle_type_str = match self.destination_type() {
            Some(DestinationType::Uint256) => "uint256",
            Some(DestinationType::Uint128) => "uint128",
            Some(DestinationType::Uint64) => "uint64",
            Some(DestinationType::String) => "string",
            _ => panic!("Invalid oracle type"),
        };
        res.insert(
            "chainsight:oracleType".to_string(),
            oracle_type_str.to_string(),
        );
        let (interval_key, interval_val) = custom_tags_interval_sec(self.interval);
        res.insert(interval_key, interval_val);
        res
    }
    fn generate_bindings(&self) -> anyhow::Result<BTreeMap<String, String>> {
        let RelayerComponentManifest {
            datasource: Datasource { method, .. },
            ..
        } = self;
        let interface = method.interface.clone();
        let lib = if let Some(path) = interface {
            let did_str = read_did_to_string_without_service(path)?;
            let identifier = CanisterMethodIdentifier::new_with_did(&method.identifier, did_str)?;
            identifier.compile()?
        } else {
            let identifier = CanisterMethodIdentifier::new(&method.identifier)?;
            identifier.compile()?
        };

        Ok(BTreeMap::from([("lib".to_string(), lib)]))
    }
    fn cycle_managements(&self) -> CycleManagements {
        self.cycles.clone().unwrap_or_default().into()
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
        let oracle_type = DestinationType::Uint256;
        Self::new(
            network_id,
            oracle_type,
            get_oracle_address(network_id),
            "https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL_KEY}".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::{
        codegen::components::common::{DatasourceLocation, DatasourceMethod},
        test_utils::SrcString,
    };

    use super::*;

    fn sample_relayer_manifest() -> RelayerComponentManifest {
        RelayerComponentManifest {
            id: Some(String::from("sample_relayer")),
            version: "v1".to_string(),
            metadata: ComponentMetadata {
                label: "Sample Relayer".to_string(),
                type_: ComponentType::Relayer,
                description: "Description".to_string(),
                tags: Some(vec!["Oracle".to_string(), "snapshot".to_string()]),
            },
            datasource: Datasource {
                location: DatasourceLocation::new_canister("datasource_canister_id".to_string()),
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
                type_: DestinationType::String,
                oracle_address: "0x0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                rpc_url: "https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL_KEY}"
                    .to_string(),
            },
            lens_targets: None,
            interval: 3600,
            cycles: None,
        }
    }

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
    location:
        id: datasource_canister_id
    method:
        identifier: 'get_last_snapshot : () -> (record { value : text; timestamp : nat64 })'
        interface: null
        args: []
destination:
    network_id: 80001
    type: uint256
    oracle_address: 0x0539a0EF8e5E60891fFf0958A059E049e43020d9
    rpc_url: https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL_KEY}
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
                    type_: DestinationType::Uint256,
                    oracle_address: "0x0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                    rpc_url: "https://polygon-mumbai.infura.io/v3/${INFURA_MUMBAI_RPC_URL_KEY}"
                        .to_string(),
                },
                lens_targets: None,
                interval: 3600,
                cycles: None,
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
        let manifest = sample_relayer_manifest();

        let snap_prefix = "snapshot__relayer";

        // FIXME failed to load oracle abi at ./__interfaces/{}.json
        // assert_display_snapshot!(SrcString::from(
        //     &manifest.generate_codes(Option::None).unwrap()
        // ));

        assert_display_snapshot!(
            format!("{}__logics_lib", &snap_prefix),
            SrcString::from(manifest.generate_user_impl_template().unwrap().lib)
        );
        assert_display_snapshot!(
            format!("{}__scripts", &snap_prefix),
            &manifest.generate_scripts(Network::Local).unwrap()
        );
    }

    #[test]
    fn test_snapshot_outputs_lens() {
        let mut manifest = sample_relayer_manifest();
        manifest.datasource.method = DatasourceMethod {
            identifier: "calculate : () -> (record { value : text; timestamp : nat64 })"
                .to_string(),
            interface: None,
            args: vec![],
        };
        manifest.lens_targets = Option::Some(LensTargets {
            identifiers: vec![
                "lens_target_canister_1".to_string(),
                "lens_target_canister_2".to_string(),
            ],
        });

        let snap_prefix = "snapshot__relayer_lens";

        // FIXME failed to load oracle abi at ./__interfaces/{}.json
        // assert_display_snapshot!(SrcString::from(
        //     &manifest.generate_codes(Option::None).unwrap()
        // ));

        let generated_user_impl_template = manifest.generate_user_impl_template().unwrap();
        assert_display_snapshot!(
            format!("{}__logics_lib", &snap_prefix),
            SrcString::from(generated_user_impl_template.lib)
        );
        assert_display_snapshot!(
            format!("{}__logics_types", &snap_prefix),
            generated_user_impl_template.types.unwrap()
        );

        assert_display_snapshot!(
            format!("{}__scripts", &snap_prefix),
            &manifest.generate_scripts(Network::Local).unwrap()
        );
    }
}
