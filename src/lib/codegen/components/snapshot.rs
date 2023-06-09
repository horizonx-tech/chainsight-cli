use std::{fs::OpenOptions, path::Path, io::Read};

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{types::ComponentType, lib::codegen::canisters};

use super::common::{Datasource, ComponentManifest};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapshotComponentManifest {
    pub version: String,
    #[serde(rename = "type")]
    pub type_: ComponentType,
    pub label: String,
    pub datasource: Datasource,
    pub storage: SnapshotStorage,
    pub interval: u32
}

impl SnapshotComponentManifest {
    pub fn new(component_label: &str, version: &str, datasource: Datasource, storage: SnapshotStorage, interval: u32) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Snapshot,
            label: component_label.to_owned(),
            datasource,
            storage,
            interval,
        }
    }
}
impl ComponentManifest for SnapshotComponentManifest {
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
        canisters::snapshot::validate_manifest(self)
    }

    fn generate_codes(&self) -> anyhow::Result<TokenStream> {
        canisters::snapshot::generate_codes(self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapshotStorage {
    pub with_timestamp: bool,
}
impl SnapshotStorage {
    pub fn new(with_timestamp: bool) -> Self {
        Self {
            with_timestamp,
        }
    }
}
impl Default for SnapshotStorage {
    fn default() -> Self {
        Self::new(true)
    }
}
