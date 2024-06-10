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
                url_to_filename_for_dfx_local(&network.value())
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

    pub fn contains_key(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    pub fn get_all_entries(&self) -> Vec<(String, String)> {
        self.components
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

fn url_to_filename_for_dfx_local(url: &str) -> String {
    url.replace(":", "_").replace(".", "_").replace("/", "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_to_filename_for_dfx_local() {
        assert_eq!(
            url_to_filename_for_dfx_local("http://localhost:8000"),
            "http___localhost_8000"
        );
        assert_eq!(
            url_to_filename_for_dfx_local("http://127.0.0.1:4943/"),
            "http___127_0_0_1_4943_"
        );
    }
}
