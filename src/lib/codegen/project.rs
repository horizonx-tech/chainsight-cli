use std::{fs::OpenOptions, io::Read, path::Path};

use serde::{Deserialize, Serialize};

use crate::types::ComponentType;

use super::components::{
    algorithm_indexer::AlgorithmIndexerComponentManifest,
    algorithm_lens::AlgorithmLensComponentManifest,
    common::{ComponentManifest, ComponentTypeInManifest},
    event_indexer::EventIndexerComponentManifest,
    relayer::RelayerComponentManifest,
    snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
    snapshot_indexer_https::SnapshotIndexerHTTPSComponentManifest,
    snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
};

/// Manifest to express Chainsight Project
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectManifestData {
    pub version: String,
    pub label: String,
    pub components: Vec<ProjectManifestComponentField>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectManifestComponentField {
    pub component_path: String,
    // pub canister_id: Option<String> // NOTE: Currently not in use
}

impl ProjectManifestData {
    pub fn new(
        project_name: &str,
        version: &str,
        components: &[ProjectManifestComponentField],
    ) -> Self {
        Self {
            version: version.to_owned(),
            label: project_name.to_owned(),
            components: components.to_vec(),
        }
    }
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let mut file = OpenOptions::new().read(true).open(Path::new(path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = serde_yaml::from_str(&contents)?;
        Ok(data)
    }

    pub fn to_str_as_yaml(&self) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(&self)?;
        Ok(yaml)
    }

    pub fn add_components(
        &mut self,
        components: &[ProjectManifestComponentField],
    ) -> anyhow::Result<()> {
        for component in components {
            self.components.push(component.clone());
        }
        Ok(())
    }

    pub fn load_component_manifests(
        &self,
        project_path: &str,
    ) -> anyhow::Result<Vec<Box<dyn ComponentManifest>>> {
        let mut manifests = vec![];
        for component in self.components.iter() {
            manifests.push(component.load_manifest(project_path)?);
        }
        Ok(manifests)
    }
}

impl ProjectManifestComponentField {
    pub fn new(component_path: &str, _canister_id: Option<String>) -> Self {
        Self {
            component_path: component_path.to_owned(),
            // canister_id // NOTE: Currently not in use
        }
    }

    pub fn load_manifest(&self, project_path: &str) -> anyhow::Result<Box<dyn ComponentManifest>> {
        let relative_component_path = &self.component_path;
        let component_path = format!("{}/{}", project_path, relative_component_path);
        let component_type = ComponentTypeInManifest::determine_type(&component_path)?;

        let id = Path::new(&component_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        let manifest: Box<dyn ComponentManifest> =
            match component_type {
                ComponentType::EventIndexer => Box::new(
                    EventIndexerComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::AlgorithmIndexer => Box::new(
                    AlgorithmIndexerComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::SnapshotIndexerICP => Box::new(
                    SnapshotIndexerICPComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::SnapshotIndexerEVM => Box::new(
                    SnapshotIndexerEVMComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::Relayer => {
                    Box::new(RelayerComponentManifest::load_with_id(&component_path, id)?)
                }
                ComponentType::AlgorithmLens => Box::new(
                    AlgorithmLensComponentManifest::load_with_id(&component_path, id)?,
                ),
                ComponentType::SnapshotIndexerHTTPS => Box::new(
                    SnapshotIndexerHTTPSComponentManifest::load_with_id(&component_path, id)?,
                ),
            };

        Ok(manifest)
    }
}
