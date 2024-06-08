use std::collections::BTreeMap;

use super::dfx::DfxWrapperNetwork;

pub type ComponentIds = BTreeMap<String, String>;
pub struct ComponentIdsManager {
    filename: String,
    components: ComponentIds,
}

impl ComponentIdsManager {
    pub fn new(network: &DfxWrapperNetwork) -> Self {
        Self {
            filename: Self::filename(network),
            components: BTreeMap::new(),
        }
    }

    pub fn load(network: &DfxWrapperNetwork, dir_path: &str) -> anyhow::Result<Self> {
        let filename = Self::filename(network);
        let path = format!("{}/{}", dir_path, filename);
        let json = std::fs::read_to_string(&path)?;
        let components: ComponentIds = serde_json::from_str(&json)?;
        Ok(Self {
            filename,
            components,
        })
    }

    fn filename(network: &DfxWrapperNetwork) -> String {
        let prefix = "component_ids";
        match network {
            DfxWrapperNetwork::IC => format!("{}_ic.json", prefix),
            _ => format!(
                "{}_{}.json",
                prefix,
                network
                    .value()
                    .replace(":", "_")
                    .replace(".", "_")
                    .replace("/", "_")
            ),
        }
    }

    pub fn save(&self, dir_path: &str) -> anyhow::Result<()> {
        let path = format!("{}/{}", dir_path, self.filename);
        let json = serde_json::to_string_pretty(&self.components)?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    pub fn add(&mut self, name: String, id: String) {
        self.components.insert(name, id);
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.components.get(name).cloned()
    }

    pub fn get_all_entries(&self) -> Vec<(String, String)> {
        self.components
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}
