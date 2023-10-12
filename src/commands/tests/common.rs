use std::fs;

// NOTE: Currently only one pattern of template pj
pub const TEST_PROJECT_PATH: &str = "src/commands/tests/resources/template_project";
pub const ALGORITHM_INDEXER: TestComponent = TestComponent {
    id: "sample_algorithm_indexer",
};
pub const _ALGORITHM_LENS: TestComponent = TestComponent {
    id: "sample_algorithm_lens",
};
pub const EVENT_INDEXER: TestComponent = TestComponent {
    id: "sample_event_indexer",
};
pub const RELAYER: TestComponent = TestComponent {
    id: "sample_relayer",
};
pub const SNAPSHOT_INDEXER_EVM: TestComponent = TestComponent {
    id: "sample_snapshot_indexer_evm",
};
pub const SNAPSHOT_INDEXER_HTTPS: TestComponent = TestComponent {
    id: "sample_snapshot_indexer_https",
};
pub const SNAPSHOT_INDEXER_ICP: TestComponent = TestComponent {
    id: "sample_snapshot_indexer_icp",
};

pub struct TestComponent {
    pub id: &'static str,
}
impl TestComponent {
    pub fn component_path(&self) -> String {
        format!("components/{}.yaml", self.id)
    }
}

pub fn minimum_project_folder(root_path: &str) -> anyhow::Result<()> {
    fs::create_dir_all(format!("{}/components", root_path))?;
    fs::create_dir_all(format!("{}/interfaces", root_path))?;

    fs::write(format!("{}/{}", root_path, ".chainsight"), "")?;

    Ok(())
}

pub fn generate_project_manifest(ids: &[String]) -> String {
    let component_rows = ids
        .iter()
        .map(|id| format!("- component_path: components/{}.yaml", id))
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        r#"version: v1
label: project
components:
{}"#,
        component_rows
    )
}

pub fn get_manifest_path_in_resource(id: &str) -> String {
    format!("{}/components/{}.yaml", TEST_PROJECT_PATH, id)
}

pub fn get_src_paths_in_resource(id: &str) -> (String, String, String, String) {
    let src_path = format!("{}/src", TEST_PROJECT_PATH);
    let lib_path_from_mod_root = "src/lib.rs";
    (
        format!(
            "{}/accessors/{}_accessors/{}",
            src_path, id, lib_path_from_mod_root
        ),
        format!(
            "{}/bindings/{}_bindings/{}",
            src_path, id, lib_path_from_mod_root
        ),
        format!("{}/canisters/{}/{}", src_path, id, lib_path_from_mod_root),
        format!("{}/logics/{}/{}", src_path, id, lib_path_from_mod_root),
    )
}
