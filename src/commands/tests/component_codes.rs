use std::{fs, process::Command};

use crate::{
    commands::{
        generate,
        tests::common::{get_src_paths_in_resource, SNAPSHOT_INDEXER_EVM},
    },
    lib::{environment::EnvironmentImpl, logger::create_root_logger},
};

use super::common::{
    get_manifest_path_in_resource, minimum_project_folder, TestComponent, SNAPSHOT_INDEXER_ICP,
};

fn pre_process(root_path: &str, component: &TestComponent) -> anyhow::Result<()> {
    minimum_project_folder(root_path)?;
    fs::write(
        format!("{}/{}", root_path, "project.yaml"),
        component.project_yml(),
    )?;
    let src_path = format!("./{}", get_manifest_path_in_resource(component.id));
    let dst_path = format!("./{}/{}", root_path, &component.component_path());
    fs::copy(&src_path, &dst_path)?;

    Ok(())
}

fn execute(root_path: &str, component: &TestComponent) {
    // process
    let mute_logger = create_root_logger(-4);
    let env = EnvironmentImpl::new().with_logger(mute_logger);
    assert!(generate::exec(
        &env,
        generate::GenerateOpts::new(Some(root_path.to_string())),
    )
    .is_ok());

    format_code(&format!("{}/src", root_path));

    // assertions
    let (actual_bindings_path, actual_canister_path, actual_logics_path) =
        get_generated_src_paths(root_path, component.id);
    let (expected_bindings_path, expected_canister_path, expected_logics_path) =
        get_src_paths_in_resource(component.id);
    assert_eq!(
        fs::read_to_string(actual_bindings_path).unwrap(),
        fs::read_to_string(expected_bindings_path).unwrap()
    );
    assert_eq!(
        fs::read_to_string(actual_canister_path).unwrap(),
        fs::read_to_string(expected_canister_path).unwrap()
    );
    assert_eq!(
        fs::read_to_string(actual_logics_path).unwrap(),
        fs::read_to_string(expected_logics_path).unwrap()
    );
}

fn post_process(root_path: &str) -> anyhow::Result<()> {
    fs::remove_dir_all(&root_path)?;
    Ok(())
}

fn format_code(path: &str) {
    let _ = Command::new("cargo")
        .current_dir(path)
        .args(["fmt"])
        .output();
}

fn get_generated_src_paths(root_path: &str, component_id: &str) -> (String, String, String) {
    let base_path = format!("{}/src", root_path);
    (
        format!(
            "{}/bindings/{}_bindings/src/lib.rs",
            base_path, component_id
        ),
        format!("{}/canisters/{}/src/lib.rs", base_path, component_id),
        format!("{}/logics/{}/src/lib.rs", base_path, component_id),
    )
}

#[test]
fn snasphot_indexer_icp() {
    let root_path: &str = "test_component_codes__snasphot_indexer_icp";
    let component = SNAPSHOT_INDEXER_ICP;

    assert!(pre_process(root_path, &component).is_ok());
    execute(root_path, &component);
    assert!(post_process(root_path).is_ok());
}

#[test]
fn snasphot_indexer_evm() {
    let root_path: &str = "test_component_codes__snasphot_indexer_evm";
    let component = SNAPSHOT_INDEXER_EVM;

    assert!(pre_process(root_path, &component).is_ok());
    execute(root_path, &component);
    assert!(post_process(root_path).is_ok());
}
