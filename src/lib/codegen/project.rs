use std::{fs::OpenOptions, path::Path, io::{Read}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectManifestData {
    pub version: String,
    pub label: String,
    pub components: Vec<ProjectManifestComponentField>
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectManifestComponentField {
    pub component_path: String,
    // pub canister_id: Option<String> // NOTE: Currently not in use
}

impl ProjectManifestData {
    pub fn new(project_name: &str, version: &str, components: &[ProjectManifestComponentField]) -> Self {
        Self {
            version: version.to_owned(),
            label: project_name.to_owned(),
            components: components.to_vec(),
        }
    }
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&Path::new(path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = serde_yaml::from_str(&contents)?;
        Ok(data)
    }

    pub fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(yaml)
    }

    pub fn add_components(&mut self, components: &[ProjectManifestComponentField]) -> anyhow::Result<()> {
        for component in components {
            self.components.push(component.clone());
        }
        Ok(())
    }
}

impl ProjectManifestComponentField {
    pub fn new(component_path: &str, _canister_id: Option<String>) -> Self {
        Self {
            component_path: component_path.to_owned(),
            // canister_id // NOTE: Currently not in use
        }
    }
}
