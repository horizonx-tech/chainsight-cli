use std::collections::{BTreeMap, HashMap};

use chainsight_cdk::{
    config::components::{CommonConfig, SnapshotIndexerHTTPSConfigQueries},
    initializer::CycleManagements,
};
use serde::{Deserialize, Serialize};

use crate::{
    lib::{
        codegen::{
            canisters::snapshot_indexer_https::{
                generate_app, generate_codes, JsonTypeGenStrategy,
            },
            components::common::SourceType,
        },
        utils::{component_ids_manager::ComponentIdsManager, url::is_supporting_ipv6_url},
    },
    types::{ComponentType, Network},
};

use super::{
    codegen::CodeGenerator,
    common::{
        custom_tags_interval_sec, ComponentManifest, ComponentMetadata, CycleManagementsManifest,
        DestinationType, GeneratedCodes, Sources, TimerSettings,
    },
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotIndexerHTTPSDataSource {
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub queries: SnapshotIndexerHTTPSDataSourceQueries,
}
impl Default for SnapshotIndexerHTTPSDataSource {
    fn default() -> Self {
        Self {
            url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
            headers: BTreeMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            queries: SnapshotIndexerHTTPSDataSourceQueries::Static(BTreeMap::from([
                ("ids".to_string(), "dai".to_string()),
                ("vs_currencies".to_string(), "usd".to_string()),
            ])),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum SnapshotIndexerHTTPSDataSourceQueries {
    #[serde(rename = "static")]
    Static(BTreeMap<String, String>),
    #[serde(rename = "dynamic")]
    Dynamic,
}
impl From<SnapshotIndexerHTTPSDataSourceQueries> for SnapshotIndexerHTTPSConfigQueries {
    fn from(val: SnapshotIndexerHTTPSDataSourceQueries) -> Self {
        match val {
            SnapshotIndexerHTTPSDataSourceQueries::Static(queries) => {
                SnapshotIndexerHTTPSConfigQueries::Const(queries)
            }
            SnapshotIndexerHTTPSDataSourceQueries::Dynamic => {
                SnapshotIndexerHTTPSConfigQueries::Func(
                    SnapshotIndexerHTTPSComponentManifest::QUERIES_FUNC_NAME.to_string(),
                )
            }
        }
    }
}

/// Component Manifest: Snapshot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotIndexerHTTPSComponentManifest {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub version: String,
    pub metadata: ComponentMetadata,
    pub datasource: SnapshotIndexerHTTPSDataSource,
    pub timer_settings: TimerSettings,
    pub cycles: Option<CycleManagementsManifest>,
}

impl SnapshotIndexerHTTPSComponentManifest {
    pub const QUERIES_FUNC_NAME: &'static str = "get_query_parameters";

    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: SnapshotIndexerHTTPSDataSource,
        interval: u32,
    ) -> Self {
        Self {
            id: Some(id.to_owned()),
            version: version.to_owned(),
            metadata: ComponentMetadata {
                label: label.to_owned(),
                type_: ComponentType::SnapshotIndexerHTTPS,
                description: description.to_owned(),
                tags: Some(vec![
                    "coingecko".to_string(),
                    "DAI".to_string(),
                    "USD".to_string(),
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
impl From<SnapshotIndexerHTTPSComponentManifest>
    for chainsight_cdk::config::components::SnapshotIndexerHTTPSConfig
{
    fn from(val: SnapshotIndexerHTTPSComponentManifest) -> Self {
        let SnapshotIndexerHTTPSComponentManifest { id, datasource, .. } = val;
        Self {
            common: CommonConfig {
                canister_name: id.clone().unwrap(),
            },
            url: datasource.url,
            headers: datasource.headers,
            queries: datasource.queries.into(),
        }
    }
}

pub struct SnapshotIndesxerHTTPSCodeGenerator {
    strategy: Box<dyn JsonTypeGenStrategy>,
    manifest: SnapshotIndexerHTTPSComponentManifest,
}

impl SnapshotIndesxerHTTPSCodeGenerator {
    pub fn new(
        manifest: SnapshotIndexerHTTPSComponentManifest,
        strategy: Box<dyn JsonTypeGenStrategy>,
    ) -> Self {
        Self { strategy, manifest }
    }
}

impl CodeGenerator for SnapshotIndesxerHTTPSCodeGenerator {
    fn generate_code(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: generate_codes(&self.manifest)?,
            types: None,
        })
    }

    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes> {
        Ok(GeneratedCodes {
            lib: generate_app(&self.manifest, self.strategy.as_ref())?,
            types: None,
        })
    }
    fn manifest(&self) -> Box<dyn ComponentManifest> {
        Box::new(self.manifest.clone())
    }
    fn generate_component_setup_args(
        &self,
        _network: &Network,
        _comp_id_mgr: &ComponentIdsManager,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(None)
    }
}

impl ComponentManifest for SnapshotIndexerHTTPSComponentManifest {
    fn load_with_id(path: &str, id: &str) -> anyhow::Result<Self> {
        let manifest = Self::load(path)?;
        Ok(Self {
            id: Some(id.to_owned()),
            ..manifest
        })
    }

    fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(self.yaml_str_with_configs(yaml, "snapshot_indexer_https".to_string()))
    }

    fn validate_manifest(&self) -> anyhow::Result<()> {
        // is_supporting_ipv6_url(&self.datasource.url)?;
        Ok(())
    }

    fn component_type(&self) -> ComponentType {
        ComponentType::SnapshotIndexerHTTPS
    }

    fn id(&self) -> Option<String> {
        self.id.clone()
    }

    fn metadata(&self) -> &ComponentMetadata {
        &self.metadata
    }

    fn destination_type(&self) -> Option<DestinationType> {
        None
    }

    fn required_interface(&self) -> Option<String> {
        None
    }
    fn get_sources(&self) -> Sources {
        Sources {
            source: self.datasource.url.clone(),
            source_type: SourceType::Https,
            attributes: HashMap::new(),
        }
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

#[cfg(test)]
mod tests {

    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::test_utils::SrcString;

    pub struct JsonTypeGenStrategyMock;

    impl JsonTypeGenStrategy for JsonTypeGenStrategyMock {
        fn generate_code(
            &self,
            struct_name: &str,
            _url: &str,
            _options: json_typegen_shared::Options,
        ) -> anyhow::Result<String> {
            let struct_str = format!(
                r#"
                #[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
                pub struct {struct_name} {{
                    pub id: String,
                    pub vs_currencies: String,
                    pub dai: String,
                }}
                "#,
                struct_name = struct_name
            );
            Ok(struct_str)
        }
    }

    use super::*;

    const MANIFEST_YAML_STATIC_QUERIES: &str = r#"
version: v1
metadata:
    label: sample_snapshot_indexer_https
    type: snapshot_indexer_https
    description: Description
    tags:
    - coingecko
    - DAI
    - USD
datasource:
    url: https://api.coingecko.com/api/v3/simple/price
    headers:
        content-type: application/json
    queries:
        type: static
        value:
            ids: dai
            vs_currencies: usd
timer_settings:
    interval_sec: 3600
    "#;

    const MANIFEST_YAML_DYNAMIC_QUERIES: &str = r#"
version: v1
metadata:
    label: sample_snapshot_indexer_https
    type: snapshot_indexer_https
    description: Description
datasource:
    url: https://api.coingecko.com/api/v3/simple/price
    headers:
        content-type: application/json
    queries:
        type: dynamic
timer_settings:
    interval_sec: 3600
    "#;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = MANIFEST_YAML_STATIC_QUERIES;
        let result = serde_yaml::from_str::<SnapshotIndexerHTTPSComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            SnapshotIndexerHTTPSComponentManifest {
                id: None,
                version: "v1".to_owned(),
                metadata: ComponentMetadata {
                    label: "sample_snapshot_indexer_https".to_owned(),
                    type_: ComponentType::SnapshotIndexerHTTPS,
                    description: "Description".to_string(),
                    tags: Some(vec![
                        "coingecko".to_string(),
                        "DAI".to_string(),
                        "USD".to_string()
                    ])
                },
                datasource: SnapshotIndexerHTTPSDataSource {
                    url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
                    headers: vec![("content-type".to_string(), "application/json".to_string())]
                        .into_iter()
                        .collect(),
                    queries: SnapshotIndexerHTTPSDataSourceQueries::Static(BTreeMap::from([
                        ("ids".to_string(), "dai".to_string()),
                        ("vs_currencies".to_string(), "usd".to_string()),
                    ])),
                },
                timer_settings: TimerSettings {
                    interval_sec: 3600,
                    delay_sec: None,
                    is_round_start_timing: None,
                },
                cycles: None,
            }
        );
    }

    #[test]
    fn test_validate_by_schema() {
        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/snapshot_indexer_https.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(MANIFEST_YAML_STATIC_QUERIES).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_outputs() {
        let mut manifest = serde_yaml::from_str::<SnapshotIndexerHTTPSComponentManifest>(
            MANIFEST_YAML_STATIC_QUERIES,
        )
        .unwrap();
        manifest.id = Some("sample_snapshot_indexer_https".to_string());

        let snap_prefix = "snapshot__snapshot_indexer_https";
        let generator = SnapshotIndesxerHTTPSCodeGenerator::new(
            manifest.clone(),
            Box::new(JsonTypeGenStrategyMock),
        );
        let generated_codes = generator.generate_code(Option::None).unwrap();
        assert_display_snapshot!(
            format!("{}__canisters_lib", &snap_prefix),
            SrcString::from(generated_codes.lib)
        );
        assert!(generated_codes.types.is_none());

        let generated_user_impl_template = SnapshotIndesxerHTTPSCodeGenerator::new(
            manifest.clone(),
            Box::new(JsonTypeGenStrategyMock),
        )
        .generate_user_impl_template()
        .unwrap();
        assert_display_snapshot!(
            format!("{}__logics_lib", &snap_prefix),
            generated_user_impl_template.lib
        );
        assert!(generated_user_impl_template.types.is_none());
    }

    #[test]
    fn test_to_manifest_struct_dynamic_queries() {
        let yaml = MANIFEST_YAML_DYNAMIC_QUERIES;
        let result = serde_yaml::from_str::<SnapshotIndexerHTTPSComponentManifest>(yaml);
        assert!(result.is_ok());
        let component = result.unwrap();
        assert_eq!(
            component,
            SnapshotIndexerHTTPSComponentManifest {
                id: None,
                version: "v1".to_owned(),
                metadata: ComponentMetadata {
                    label: "sample_snapshot_indexer_https".to_owned(),
                    type_: ComponentType::SnapshotIndexerHTTPS,
                    description: "Description".to_string(),
                    tags: None
                },
                datasource: SnapshotIndexerHTTPSDataSource {
                    url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
                    headers: vec![("content-type".to_string(), "application/json".to_string())]
                        .into_iter()
                        .collect(),
                    queries: SnapshotIndexerHTTPSDataSourceQueries::Dynamic,
                },
                timer_settings: TimerSettings {
                    interval_sec: 3600,
                    delay_sec: None,
                    is_round_start_timing: None,
                },
                cycles: None,
            }
        );
    }

    #[test]
    fn test_validate_by_schema_dynamic_queries() {
        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/snapshot_indexer_https.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(MANIFEST_YAML_DYNAMIC_QUERIES).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_outputs_dynamic_queries() {
        let mut manifest = serde_yaml::from_str::<SnapshotIndexerHTTPSComponentManifest>(
            MANIFEST_YAML_DYNAMIC_QUERIES,
        )
        .unwrap();
        manifest.id = Some("sample_snapshot_indexer_https".to_string());

        let snap_prefix = "snapshot__snapshot_indexer_https_dynamic_queries";
        let generated_codes = SnapshotIndesxerHTTPSCodeGenerator::new(
            manifest.clone(),
            Box::new(JsonTypeGenStrategyMock),
        )
        .generate_code(Option::None)
        .unwrap();
        assert_display_snapshot!(
            format!("{}__canisters_lib", &snap_prefix),
            SrcString::from(generated_codes.lib)
        );
        assert!(generated_codes.types.is_none());

        let generated_user_impl_template = SnapshotIndesxerHTTPSCodeGenerator::new(
            manifest.clone(),
            Box::new(JsonTypeGenStrategyMock),
        )
        .generate_user_impl_template()
        .unwrap();
        assert_display_snapshot!(
            format!("{}__logics_lib", &snap_prefix),
            generated_user_impl_template.lib
        );
        assert!(generated_user_impl_template.types.is_none());
    }
}
