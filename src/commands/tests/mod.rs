use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use ic_wasm::metadata::{get_metadata, list_metadata};
use insta::assert_display_snapshot;

use crate::{
    commands::{
        build::{self, BuildOpts},
        generate,
    },
    lib::{environment::EnvironmentImpl, logger::create_root_logger},
};

// NOTE: Currently only one pattern of template pj
const TEST_PROJECT_PATH: &str = "e2e/resources";

// Copy all yaml files under e2e/resources/{key} and generate project.yaml from those file names.
fn generate_project_by_resources(pj_root_path: &str, key: &str) -> anyhow::Result<Vec<String>> {
    fs::create_dir_all(format!("{}/components", pj_root_path))?;
    fs::create_dir_all(format!("{}/interfaces", pj_root_path))?;
    fs::write(format!("{}/{}", pj_root_path, ".chainsight"), "")?;

    let mut component_ids = Vec::<String>::new();
    let components_path = format!("{}/components", pj_root_path);
    for entry in fs::read_dir(format!("{}/{}", TEST_PROJECT_PATH, key))? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = format!(
            "{}/{}",
            components_path,
            entry.file_name().to_str().unwrap()
        );
        fs::copy(&src_path, &dst_path)?;

        component_ids.push(
            entry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        );
    }

    fs::write(
        format!("{}/{}", pj_root_path, "project.yaml"),
        generate_project_manifest(component_ids.as_slice()),
    )?;

    Ok(component_ids)
}

fn generate_project_manifest(ids: &[String]) -> String {
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

fn execute(root_path: &str) {
    // process
    let mute_logger = create_root_logger(-4);
    let env = EnvironmentImpl::new().with_logger(mute_logger);

    let res = generate::exec(
        &env,
        generate::GenerateOpts::new(Some(root_path.to_string())),
    );
    if let Err(e) = res {
        panic!("Failed to generate project: {:?}", e);
    }
    let res = build::exec(
        &env,
        BuildOpts {
            path: Some(root_path.to_string()),
            only_build: true,
        },
    );
    if let Err(e) = res {
        panic!("Failed to build project: {:?}", e);
    }

    assert!(res.is_ok());
}
fn assert_artifacts(root_path: &str, component_ids: &[String]) {
    let artifacts_path = format!("{}/artifacts", root_path);
    assert!(Path::new(&format!("{}/dfx.json", artifacts_path)).is_file());

    for cid in component_ids {
        assert_per_component(&artifacts_path, cid);
    }
}
fn assert_per_component(artifacts_path: &str, component_id: &String) {
    // Asserts whether Artifacts exists
    let did_path = format!("{}/{}.did", artifacts_path, component_id);
    assert!(Path::new(&did_path).is_file());
    let wasm_path = format!("{}/{}.wasm", artifacts_path, component_id);
    assert!(Path::new(&wasm_path).is_file());

    // Asserts generated .did
    assert_display_snapshot!(
        format!("{}-did", &component_id),
        fs::read_to_string(&did_path).unwrap()
    );

    // Asserts metadatas in modules
    let mut wasm_bytes = Vec::<u8>::new();
    File::open(&wasm_path)
        .unwrap()
        .read_to_end(&mut wasm_bytes)
        .unwrap();
    let module =
        ic_wasm::utils::parse_wasm(&wasm_bytes, true).expect("Failed to parse wasm to module");

    let mut metadata_names = list_metadata(&module);
    metadata_names.sort();
    let name_prefix = "icp:public "; // TODO: or icp:private
    let metadata = metadata_names
        .iter()
        .map(|fullname| {
            let name: &str = fullname
                .strip_prefix(name_prefix)
                .expect(format!("Failed about metadata - No prefix: {}", fullname).as_str());
            let data = get_metadata(&module, name)
                .expect(format!("Failed to get metadata: {}", name).as_str());
            let data_str = String::from_utf8_lossy(&data).to_string();
            (name.to_string(), data_str)
        })
        .collect::<Vec<(String, String)>>();
    assert_display_snapshot!(component_id.to_string(), format!("{:#?}", &metadata));
}

// fn post_process(root_path: &str) -> anyhow::Result<()> {
//     fs::remove_dir_all(&root_path)?;
//     Ok(())
// }

#[test]
fn test_template() {
    let root_path: &str = "test__e2e_template";
    let test_target_key = "template";
    let test = || {
        let component_ids = generate_project_by_resources(root_path, test_target_key)
            .expect("Failed to generate_project_by_resources");
        execute(root_path);
        assert_artifacts(root_path, &component_ids);
    };
    let result = std::panic::catch_unwind(test);
    // assert!(post_process(root_path).is_ok()); // NOTE: To pass on to docker testing
    assert!(result.is_ok())
}
