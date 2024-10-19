use std::collections::BTreeMap;

use super::dfx::DfxWrapperNetwork;

pub type ComponentIds = BTreeMap<String, BTreeMap<String, String>>; // name -> network -> id
pub struct ComponentIdsManager {
    filename: String,
    filepath: Option<String>,
    network: DfxWrapperNetwork,
    components: ComponentIds,
}

impl ComponentIdsManager {
    pub fn new(network: &DfxWrapperNetwork) -> Self {
        let (filename, filepath) = Self::filepath(network);
        Self {
            filename,
            filepath,
            network: network.clone(),
            components: BTreeMap::new(),
        }
    }

    pub fn load(network: &DfxWrapperNetwork, dir_path: &str) -> anyhow::Result<Self> {
        let (filename, filepath) = Self::filepath(network);
        let dir_path = filepath
            .clone()
            .map_or(dir_path.to_string(), |p| format!("{}/{}", dir_path, p));
        let json = std::fs::read_to_string(format!("{}/{}", dir_path, filename))?;
        let components: ComponentIds = serde_json::from_str(&json)?;
        Ok(Self {
            filename,
            filepath,
            network: network.clone(),
            components,
        })
    }

    fn filepath(network: &DfxWrapperNetwork) -> (String, Option<String>) {
        let filename = "canister_ids.json".to_string();
        match network {
            DfxWrapperNetwork::IC => (filename, None),
            _ => (filename, Some(format!(".dfx/{}", network.to_path()))),
        }
    }

    pub fn save(&self, dir_path: &str) -> anyhow::Result<()> {
        let dir_path = self
            .filepath
            .clone()
            .map_or(dir_path.to_string(), |p| format!("{}/{}", dir_path, p));
        std::fs::create_dir_all(dir_path.clone())?;
        let json = serde_json::to_string_pretty(&self.components)?;
        std::fs::write(format!("{}/{}", dir_path, &self.filename), json)?;
        Ok(())
    }

    pub fn add(&mut self, name: String, id: String) {
        if self.components.get(&name).is_none() {
            self.components.insert(name.clone(), BTreeMap::new());
        };
        self.components
            .get_mut(&name)
            .unwrap()
            .insert(self.network.to_path(), id);
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.components
            .get(name)
            .map_or(None, |m| m.get(&self.network.to_path()))
            .cloned()
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    pub fn get_all_entries(&self) -> Vec<(String, String)> {
        self.components
            .iter()
            .map(|(k, v)| (k.clone(), v.get(&self.network.to_path()).unwrap().clone()))
            .collect()
    }
}
