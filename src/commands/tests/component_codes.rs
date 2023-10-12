use std::{fs, io::Write, process::Command};

use syn::File;

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

fn execute(root_path: &str) {
    // process
    let mute_logger = create_root_logger(-4);
    let env = EnvironmentImpl::new().with_logger(mute_logger);
    assert!(generate::exec(
        &env,
        generate::GenerateOpts::new(Some(root_path.to_string())),
    )
    .is_ok());
}

fn assert_eq_with_rustfmt(root_path: &str, components: &[&TestComponent]) {
    for c in components {
        assert_eq_with_rustfmt_per_component(&root_path, c);
    }
}
fn assert_eq_with_rustfmt_per_component(root_path: &str, c: &TestComponent) {
    let (_, actual_bindings_path, actual_canister_path, actual_logics_path) =
        get_generated_src_paths(root_path, c.id);
    let (_, expected_bindings_path, expected_canister_path, expected_logics_path) =
        get_src_paths_in_resource(c.id);

    // assertions
    let actual = fs::read_to_string(actual_bindings_path).unwrap();
    let expected = fs::read_to_string(expected_bindings_path).unwrap();
    assert_eq!(
        format_with_rustfmt(actual),
        format_with_rustfmt(expected),
        "fail to compare /bindings: {}",
        c.id
    );
    let actual = fs::read_to_string(actual_canister_path).unwrap();
    let expected = fs::read_to_string(expected_canister_path).unwrap();
    assert_eq!(
        format_with_rustfmt(actual),
        format_with_rustfmt(expected),
        "fail to compare /canisters: {}",
        c.id
    );
    let actual = fs::read_to_string(actual_logics_path).unwrap();
    let expected = fs::read_to_string(expected_logics_path).unwrap();
    assert_eq!(
        format_with_rustfmt(actual),
        format_with_rustfmt(expected),
        "fail to compare /logics: {}",
        c.id
    );
}
fn format_with_rustfmt(code: String) -> String {
    let mut child = Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to run rustfmt");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(code.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).expect("Output is not valid UTF-8")
}

fn assert_eq_by_asts(root_path: &str, components: &[&TestComponent]) {
    for c in components {
        assert_eq_by_asts_per_component(&root_path, c);
    }
}
fn assert_eq_by_asts_per_component(root_path: &str, c: &TestComponent) {
    let (_, actual_bindings_path, actual_canister_path, actual_logics_path) =
        get_generated_src_paths(root_path, c.id);
    let (_, expected_bindings_path, expected_canister_path, expected_logics_path) =
        get_src_paths_in_resource(c.id);

    let actual = fs::read_to_string(actual_bindings_path).unwrap();
    let expected = fs::read_to_string(expected_bindings_path).unwrap();
    assert_eq!(
        syn::parse_str::<File>(&actual).expect("Failed to parse generated code"),
        syn::parse_str::<File>(&expected).expect("Failed to parse expected code"),
        "fail to compare /bindings: {}",
        c.id
    );
    let actual = fs::read_to_string(actual_canister_path).unwrap();
    let expected = fs::read_to_string(expected_canister_path).unwrap();
    assert_eq!(
        syn::parse_str::<File>(&actual).expect("Failed to parse generated code"),
        syn::parse_str::<File>(&expected).expect("Failed to parse expected code"),
        "fail to compare /canisters: {}",
        c.id
    );
    let actual = fs::read_to_string(actual_logics_path).unwrap();
    let expected = fs::read_to_string(expected_logics_path).unwrap();
    assert_eq!(
        syn::parse_str::<File>(&actual).expect("Failed to parse generated code"),
        syn::parse_str::<File>(&expected).expect("Failed to parse expected code"),
        "fail to compare /logics: {}",
        c.id
    );
}

fn assert_eq_to_remove_spaces_newlines(root_path: &str, components: &[&TestComponent]) {
    for c in components {
        assert_eq_to_remove_spaces_newlines_per_component(&root_path, c);
    }
}
fn assert_eq_to_remove_spaces_newlines_per_component(root_path: &str, c: &TestComponent) {
    let (_, actual_bindings_path, actual_canister_path, actual_logics_path) =
        get_generated_src_paths(root_path, c.id);
    let (_, expected_bindings_path, expected_canister_path, expected_logics_path) =
        get_src_paths_in_resource(c.id);

    let actual = fs::read_to_string(actual_bindings_path).unwrap();
    let expected = fs::read_to_string(expected_bindings_path).unwrap();
    assert_eq!(
        remove_newlines(remove_spaces(actual)),
        remove_newlines(remove_spaces(expected)),
        "fail to compare /bindings: {}",
        c.id
    );
    let actual = fs::read_to_string(actual_canister_path).unwrap();
    let expected = fs::read_to_string(expected_canister_path).unwrap();
    assert_eq!(
        remove_newlines(remove_spaces(actual)),
        remove_newlines(remove_spaces(expected)),
        "fail to compare /canisters: {}",
        c.id
    );
    let actual = fs::read_to_string(actual_logics_path).unwrap();
    let expected = fs::read_to_string(expected_logics_path).unwrap();
    assert_eq!(
        remove_newlines(remove_spaces(actual)),
        remove_newlines(remove_spaces(expected)),
        "fail to compare /logics: {}",
        c.id
    );
}
fn remove_spaces(code: String) -> String {
    code.chars().filter(|c| !c.is_whitespace()).collect()
}
fn remove_newlines(code: String) -> String {
    code.chars().filter(|&c| c != '\n').collect()
}

fn post_process(root_path: &str) -> anyhow::Result<()> {
    fs::remove_dir_all(&root_path)?;
    Ok(())
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
fn test_with_fmt() {
    let root_path: &str = "test__component_codes_with_fmt";
    let components = &[
        &ALGORITHM_INDEXER,
        // &ALGORITHM_LENS,
        &EVENT_INDEXER,
        &SNAPSHOT_INDEXER_ICP,
        &SNAPSHOT_INDEXER_EVM,
        &SNAPSHOT_INDEXER_HTTPS,
        &RELAYER,
    ];

    assert!(pre_process(root_path, components).is_ok());
    execute(root_path);
    assert_eq_with_rustfmt(root_path, components);
    assert!(post_process(root_path).is_ok());
}

// #[test]
// fn test_with_ast() {
//     let root_path: &str = "test__component_codes_with_ast";
//     let components = &[
//         &ALGORITHM_INDEXER,
//         // &ALGORITHM_LENS,
//         &EVENT_INDEXER,
//         &SNAPSHOT_INDEXER_ICP,
//         &SNAPSHOT_INDEXER_EVM,
//         &SNAPSHOT_INDEXER_HTTPS,
//         &RELAYER,
//     ];

//     assert!(pre_process(root_path, components).is_ok());
//     execute(root_path);
//     assert_eq_by_asts(root_path, components);
//     assert!(post_process(root_path).is_ok());
// }

// #[test]
// fn test_with_removing_spaces_newlines() {
//     let root_path: &str = "test__component_codes_with_removing_spaces_newlines";
//     let components = &[
//         &ALGORITHM_INDEXER,
//         // &ALGORITHM_LENS,
//         &EVENT_INDEXER,
//         &SNAPSHOT_INDEXER_ICP,
//         &SNAPSHOT_INDEXER_EVM,
//         &SNAPSHOT_INDEXER_HTTPS,
//         &RELAYER,
//     ];

//     assert!(pre_process(root_path, components).is_ok());
//     execute(root_path);
//     assert_eq_to_remove_spaces_newlines(root_path, components);
//     assert!(post_process(root_path).is_ok());
// }
