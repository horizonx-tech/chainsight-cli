use std::collections::HashMap;

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{
    lib::codegen::{canisters, components::common::SourceType, scripts},
    types::{ComponentType, Network},
    utils::serializer::ordered_map,
};

use super::common::{
    custom_tags_interval_sec, ComponentManifest, ComponentMetadata, DestinationType,
    SnapshotStorage, Sources,
};
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]

pub struct SnapshotIndexerHTTPSDataSource {
    pub url: String,
    #[serde(serialize_with = "ordered_map")]
    pub headers: HashMap<String, String>,
    #[serde(serialize_with = "ordered_map")]
    pub queries: HashMap<String, String>,
}
impl Default for SnapshotIndexerHTTPSDataSource {
    fn default() -> Self {
        Self {
            url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())]
                .into_iter()
                .collect(),
            queries: vec![
                ("ids".to_string(), "dai".to_string()),
                ("vs_currencies".to_string(), "usd".to_string()),
            ]
            .into_iter()
            .collect(),
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
    pub storage: SnapshotStorage,
    pub interval: u32,
}

impl SnapshotIndexerHTTPSComponentManifest {
    pub fn new(
        id: &str,
        label: &str,
        description: &str,
        version: &str,
        datasource: SnapshotIndexerHTTPSDataSource,
        storage: SnapshotStorage,
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
            storage,
            interval,
        }
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
        Ok(())
    }

    fn generate_codes(
        &self,
        _interface_contract: Option<ethabi::Contract>,
    ) -> anyhow::Result<TokenStream> {
        canisters::snapshot_indexer_https::generate_codes(self)
    }

    fn generate_scripts(&self, network: Network) -> anyhow::Result<String> {
        scripts::snapshot_indexer_https::generate_scripts(self, network)
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

    fn generate_user_impl_template(&self) -> anyhow::Result<TokenStream> {
        canisters::snapshot_indexer_https::generate_app(self)
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
        let (interval_key, interval_val) = custom_tags_interval_sec(self.interval);
        res.insert(interval_key, interval_val);
        res
    }
}

#[cfg(test)]
mod tests {

    use insta::assert_display_snapshot;
    use jsonschema::JSONSchema;

    use crate::lib::test_utils::SrcString;

    use super::*;

    #[test]
    fn test_to_manifest_struct() {
        let yaml = r#"
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
        ids: dai
        vs_currencies: usd
storage:
    with_timestamp: true
interval: 3600
        "#;

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
                    queries: vec![
                        ("ids".to_string(), "dai".to_string()),
                        ("vs_currencies".to_string(), "usd".to_string())
                    ]
                    .into_iter()
                    .collect(),
                },
                storage: SnapshotStorage {
                    with_timestamp: true,
                },
                interval: 3600
            }
        );
        let schema = serde_json::from_str(include_str!(
            "../../../../resources/schema/snapshot_indexer_https.json"
        ))
        .expect("Invalid json");
        let instance = serde_yaml::from_str(yaml).expect("Invalid yaml");
        let compiled = JSONSchema::compile(&schema).expect("Invalid schema");
        let result = compiled.validate(&instance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_outputs() {
        let manifest = SnapshotIndexerHTTPSComponentManifest {
            id: Some("sample_snapshot_indexer_https".to_owned()),
            version: "v1".to_owned(),
            metadata: ComponentMetadata {
                label: "Sample Snapshot Indexer Https".to_owned(),
                type_: ComponentType::SnapshotIndexerHTTPS,
                description: "Description".to_string(),
                tags: Some(vec![
                    "coingecko".to_string(),
                    "DAI".to_string(),
                    "USD".to_string(),
                ]),
            },
            datasource: SnapshotIndexerHTTPSDataSource {
                url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
                headers: vec![("content-type".to_string(), "application/json".to_string())]
                    .into_iter()
                    .collect(),
                queries: vec![
                    ("ids".to_string(), "dai".to_string()),
                    ("vs_currencies".to_string(), "usd".to_string()),
                ]
                .into_iter()
                .collect(),
            },
            storage: SnapshotStorage {
                with_timestamp: true,
            },
            interval: 3600,
        };

        assert_display_snapshot!(SrcString::from(
            &manifest.generate_codes(Option::None).unwrap()
        ));
        assert_display_snapshot!(SrcString::from(
            &manifest.generate_user_impl_template().unwrap()
        ));
        assert_display_snapshot!(&manifest.generate_scripts(Network::Local).unwrap());
    }
}
