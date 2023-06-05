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
    pub interval: u32
}

impl SnapshotComponentManifest {
    pub fn new(component_label: &str, version: &str, datasource: Datasource, interval: u32) -> Self {
        Self {
            version: version.to_owned(),
            type_: ComponentType::Snapshot,
            label: component_label.to_owned(),
            datasource,
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

    fn generate_codes(&self) -> anyhow::Result<TokenStream> {
        canisters::generate_snapshot_codes(self)
    }
}
