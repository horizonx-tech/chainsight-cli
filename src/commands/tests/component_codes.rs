use std::{fs, process::Command};

use crate::{
    commands::{
        generate,
        tests::common::{
            get_src_paths_in_resource, ALGORITHM_INDEXER, EVENT_INDEXER, RELAYER,
            SNAPSHOT_INDEXER_EVM, SNAPSHOT_INDEXER_HTTPS,
        },
    },
    lib::{environment::EnvironmentImpl, logger::create_root_logger},
};

use super::common::{
    generate_project_manifest, get_manifest_path_in_resource, minimum_project_folder,
    TestComponent, SNAPSHOT_INDEXER_ICP,
};

fn pre_process(root_path: &str, component: &[&TestComponent]) -> anyhow::Result<()> {
    minimum_project_folder(root_path)?;

    let mut component_ids = Vec::<String>::new();
    for c in component {
        let src_path = format!("./{}", get_manifest_path_in_resource(c.id));
        let dst_path = format!("./{}/{}", root_path, &c.component_path());
        fs::copy(&src_path, &dst_path)?;

        component_ids.push(c.id.to_string());
    }

    fs::write(
        format!("{}/{}", root_path, "project.yaml"),
        generate_project_manifest(component_ids.as_slice()),
    )?;

    Ok(())
}

fn execute(root_path: &str, components: &[&TestComponent]) {
    // process
    let mute_logger = create_root_logger(-4);
    let env = EnvironmentImpl::new().with_logger(mute_logger);
    assert!(generate::exec(
        &env,
        generate::GenerateOpts::new(Some(root_path.to_string())),
    )
    .is_ok());

    // NOTE: to compare formatted codes
    format_code(&format!("{}/src", root_path));

    for c in components {
        asserts_per_component(&root_path, c);
    }
}

fn asserts_per_component(root_path: &str, c: &TestComponent) {
    let (_, actual_bindings_path, actual_canister_path, actual_logics_path) =
        get_generated_src_paths(root_path, c.id);
    let (_, expected_bindings_path, expected_canister_path, expected_logics_path) =
        get_src_paths_in_resource(c.id);

    // assertions
    assert_eq!(
        fs::read_to_string(actual_bindings_path).unwrap(),
        fs::read_to_string(expected_bindings_path).unwrap(),
        "fail to compare /bindings: {}",
        c.id
    );
    assert_eq!(
        fs::read_to_string(actual_canister_path).unwrap(),
        fs::read_to_string(expected_canister_path).unwrap(),
        "fail to compare /canisters: {}",
        c.id
    );
    assert_eq!(
        fs::read_to_string(actual_logics_path).unwrap(),
        fs::read_to_string(expected_logics_path).unwrap(),
        "fail to compare /logics: {}",
        c.id
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

fn get_generated_src_paths(
    root_path: &str,
    component_id: &str,
) -> (String, String, String, String) {
    let base_path = format!("{}/src", root_path);
    (
        format!(
            "{}/accessors/{}_accessors/src/lib.rs",
            base_path, component_id
        ),
        format!(
            "{}/bindings/{}_bindings/src/lib.rs",
            base_path, component_id
        ),
        format!("{}/canisters/{}/src/lib.rs", base_path, component_id),
        format!("{}/logics/{}/src/lib.rs", base_path, component_id),
    )
}

#[test]
fn test() {
    let root_path: &str = "test__component_codes";
    let components = [
        &ALGORITHM_INDEXER,
        // &ALGORITHM_LENS,
        &EVENT_INDEXER,
        &SNAPSHOT_INDEXER_ICP,
        &SNAPSHOT_INDEXER_EVM,
        &SNAPSHOT_INDEXER_HTTPS,
        &RELAYER,
    ];

    assert!(pre_process(root_path, &components).is_ok());
    execute(root_path, &components);
    assert!(post_process(root_path).is_ok());
}
