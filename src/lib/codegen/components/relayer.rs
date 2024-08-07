use std::collections::{BTreeMap, HashMap};

use anyhow::Ok;
use chainsight_cdk::{
    config::components::{CommonConfig, LensParameter, LensTargets, RelayerConversionParameter},
    initializer::CycleManagements,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    lib::{
        codegen::{
            canisters, components::common::custom_tags_interval_sec, oracle::get_oracle_address,
            scripts,
        },
        utils::component_ids_manager::ComponentIdsManager,
    },
    types::{ComponentType, Network},
};

use super::{
    codegen::CodeGenerator,
    common::{
        ComponentManifest, ComponentMetadata, CycleManagementsManifest, DatasourceForCanister,
        DestinationType, GeneratedCodes, SourceType, Sources, TimerSettings,
    },
    utils::{
        generate_method_identifier, generate_types_from_bindings, get_did_by_component_id,
        is_lens_with_args,
    },
};

/// Component Manifest: Relayer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RelayerComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: DatasourceForCanister,
    pub destination: DestinationField, // TODO: multiple destinations
    pub timer_settings: TimerSettings,
    pub conversion_parameter: Option<RelayerConversionParameter>,
    pub lens_targets: Option<LensTargets>,
    pub cycles: Option<CycleManagementsManifest>,
}

impl RelayerComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: DatasourceForCanister,
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
            conversion_parameter: None,
            lens_targets: None,
            timer_settings: TimerSettings {
                interval_sec: interval,
                delay_sec: None,
                is_round_start_timing: None,
            },
            cycles: None,
        }
    }

    fn abi_file_path(&self) -> String {
        match self.destination_type() {
            Some(DestinationType::Custom) => {
                format!(
                    "__interfaces/{}",
                    self.destination
                        .interface
                        .clone()
                        .expect("missing field: interface is required for custom oracle")
                )
            }
            Some(_) => "__interfaces/Oracle.json".to_string(),
            _ => panic!("missing field: destination_type is required"),
        }
    }

    fn relay_method_name(&self) -> String {
        match self.destination_type() {
            Some(DestinationType::Custom) => self
                .destination
                .method_name
                .clone()
                .expect("missing field: method_name is required for custom oracle"),
            Some(_) => "update_state".to_string(),
            _ => panic!("missing field: destination_type is required"),
        }
    }
}

impl From<RelayerComponentManifest> for chainsight_cdk::config::components::RelayerConfig {
    fn from(val: RelayerComponentManifest) -> Self {
        let id = val.id();
        let RelayerComponentManifest {
            datasource:
                DatasourceForCanister {
                    ref method,
                    ref location,
                    ..
                },
            ref destination,
            ..
        } = val;

        let lens_parameter = if val.lens_targets.is_some() {
            let interface = if method.interface.is_some() {
                method.interface.clone()
            } else {
                get_did_by_component_id(&location.id)
            };
            let identifier = generate_method_identifier(&method.identifier, &interface)
                .unwrap_or_else(|e| panic!("{}", e.to_string()));
            let with_args = is_lens_with_args(identifier);
            Some(LensParameter { with_args })
        } else {
            None
        };

        Self {
            common: CommonConfig {
                canister_name: id.clone().unwrap(),
            },
            method_identifier: method.identifier.clone(),
            destination: destination.oracle_address.clone(),
            abi_file_path: val.abi_file_path(),
            method_name: val.relay_method_name(),
            lens_parameter,
            conversion_parameter: val.conversion_parameter,
        }
    }
}

pub struct RelayerCodeGenerator {
    manifest: RelayerComponentManifest,
}

impl RelayerCodeGenerator {
    pub fn new(manifest: RelayerComponentManifest) -> Self {
        Self { manifest }
    }
}

impl CodeGenerator for RelayerCodeGenerator {
    fn generate_code(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::relayer::generate_codes(&self.manifest)?;

        let types = generate_types_from_bindings(
            &self.manifest.id.clone().unwrap(),
            &self.manifest.datasource.method.identifier,
        )?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
        })
    }

    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        let lib = canisters::relayer::generate_app(&self.manifest)?;

        let types = generate_types_from_bindings(
            &self.manifest.id.clone().unwrap(),
            &self.manifest.datasource.method.identifier,
        )?;

        Ok(GeneratedCodes {
            lib,
            types: Some(types),
        })
    }

    fn manifest(&self) -> Box<dyn ComponentManifest> {
        Box::new(self.manifest.clone())
    }

    fn generate_component_setup_args(
        &self,
        network: &Network,
        comp_id_mgr: &ComponentIdsManager,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let args =
            scripts::relayer::generate_component_setup_args(&self.manifest, network, comp_id_mgr)?;
        Ok(Some(args))
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
        match self.destination_type() {
            Some(DestinationType::Custom) => self.destination.interface.clone(),
            _ => None,
        }
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
        let oracle_type_str = oracle_type(self.destination_type());
        res.insert(
            "chainsight:oracleType".to_string(),
            oracle_type_str.to_string(),
        );
        let (interval_key, interval_val) =
            custom_tags_interval_sec(self.timer_settings.interval_sec);
        res.insert(interval_key, interval_val);
        res
    }
    fn generate_bindings(&self) -> anyhow::Result<BTreeMap<String, String>> {
        let RelayerComponentManifest {
            datasource: DatasourceForCanister {
                location, method, ..
            },
            ..
        } = self;

        let interface = if method.interface.is_some() {
            method.interface.clone()
        } else {
            get_did_by_component_id(&location.id)
        };

        let identifier = generate_method_identifier(&method.identifier, &interface)?;
        let lib = identifier.compile()?;
        Ok(BTreeMap::from([("lib".to_string(), lib)]))
    }
    fn timer_settings(&self) -> Option<TimerSettings> {
        Some(self.timer_settings.clone())
    }
    fn cycle_managements(&self) -> CycleManagements {
        self.cycles.clone().unwrap_or_default().into()
    }
}

fn oracle_type(t: Option<DestinationType>) -> String {
    match t {
        Some(t) => {
            let val = serde_json::to_string(&t).unwrap();
            val.trim_matches('\"').to_string()
        }
        _ => panic!("Invalid oracle type"),
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DestinationField {
    pub network_id: u32,
    #[serde(rename = "type")]
    pub type_: DestinationType,
    pub oracle_address: String,
    pub rpc_url: String,
    pub method_name: Option<String>,
    pub interface: Option<String>,
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
            method_name: None,
            interface: None,
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
            "https://eth.llamarpc.com".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::{
        codegen::components::common::{
            DatasourceForCanister, DatasourceLocationForCanister, DatasourceMethod,
        },
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
            datasource: DatasourceForCanister {
                location: DatasourceLocationForCanister {
                    id: "datasource_canister_id".to_string(),
                },
                method: DatasourceMethod {
                    identifier:
                        "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                            .to_string(),
                    interface: None,
                    args: vec![],
                },
            },
            destination: DestinationField {
                network_id: 1,
                type_: DestinationType::String,
                oracle_address: "0x0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                rpc_url: "https://eth.llamarpc.com".to_string(),
                method_name: None,
                interface: None,
            },
            lens_targets: None,
            conversion_parameter: None,
            timer_settings: TimerSettings {
                interval_sec: 3600,
                delay_sec: None,
                is_round_start_timing: None,
            },
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
    network_id: 1
    type: uint256
    oracle_address: 0x0539a0EF8e5E60891fFf0958A059E049e43020d9
    rpc_url: https://eth.llamarpc.com
timer_settings:
    interval_sec: 3600
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
                datasource: DatasourceForCanister {
                    location: DatasourceLocationForCanister {
                        id: "datasource_canister_id".to_string(),
                    },
                    method: DatasourceMethod {
                        identifier:
                            "get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
                                .to_string(),
                        interface: None,
                        args: vec![]
                    }
                },
                destination: DestinationField {
                    network_id: 1,
                    type_: DestinationType::Uint256,
                    oracle_address: "0x0539a0EF8e5E60891fFf0958A059E049e43020d9".to_string(),
                    rpc_url: "https://eth.llamarpc.com".to_string(),
                    method_name: None,
                    interface: None,
                },
                lens_targets: None,
                conversion_parameter: None,
                timer_settings: TimerSettings {
                    interval_sec: 3600,
                    delay_sec: None,
                    is_round_start_timing: None,
                },
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
            SrcString::from(
                RelayerCodeGenerator::new(manifest.clone())
                    .generate_user_impl_template()
                    .unwrap()
                    .lib
            )
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

        let generated_user_impl_template = RelayerCodeGenerator::new(manifest.clone())
            .generate_user_impl_template()
            .unwrap();
        assert_display_snapshot!(
            format!("{}__logics_lib", &snap_prefix),
            SrcString::from(generated_user_impl_template.lib)
        );
        assert_display_snapshot!(
            format!("{}__logics_types", &snap_prefix),
            generated_user_impl_template.types.unwrap()
        );
    }
    #[test]
    fn test_oracle_type() {
        assert_eq!(oracle_type(Some(DestinationType::Uint256)), "uint256");
        assert_eq!(oracle_type(Some(DestinationType::Uint128)), "uint128");
        assert_eq!(oracle_type(Some(DestinationType::Uint64)), "uint64");
        assert_eq!(oracle_type(Some(DestinationType::String)), "string");
        assert_eq!(oracle_type(Some(DestinationType::Custom)), "custom");
    }
}
