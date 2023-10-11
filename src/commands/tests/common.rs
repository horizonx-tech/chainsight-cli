use std::fs;

pub const TEST_MANIFEST_PATH: &str = "src/commands/tests/resources/manifests";
pub const TEST_SRC_PATH: &str = "src/commands/tests/resources/codes";

pub struct TestComponent {
    pub id: &'static str,
}
impl TestComponent {
    pub fn project_yml(&self) -> String {
        format!(
            r#"version: v1
label: project
components:
- component_path: {}"#,
            self.component_path()
        )
    }

    pub fn component_path(&self) -> String {
        format!("components/{}.yaml", self.id)
    }
}
pub const SNAPSHOT_INDEXER_ICP: TestComponent = TestComponent {
    id: "sample_snapshot_indexer_icp",
};
pub const SNAPSHOT_INDEXER_EVM: TestComponent = TestComponent {
    id: "sample_snapshot_indexer_evm",
};

pub fn minimum_project_folder(root_path: &str) -> anyhow::Result<()> {
    fs::create_dir_all(format!("{}/components", root_path))?;
    fs::create_dir_all(format!("{}/interfaces", root_path))?;

    fs::write(format!("{}/{}", root_path, ".chainsight"), "")?;

    Ok(())
}

pub fn get_manifest_path_in_resource(id: &str) -> String {
    format!("{}/{}.yaml", TEST_MANIFEST_PATH, id)
}

pub fn get_src_paths_in_resource(id: &str) -> (String, String, String) {
    let base_path = format!("{}/{}", TEST_SRC_PATH, id);
    (
        format!("{}/bindings.rs", base_path),
        format!("{}/canisters.rs", base_path),
        format!("{}/logics.rs", base_path),
    )
}
