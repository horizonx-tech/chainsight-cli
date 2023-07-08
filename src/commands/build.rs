use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{fs, path::Path};

use anyhow::{bail, Ok};
use clap::Parser;
use slog::{debug, error, info, Logger};

use crate::lib::codegen::components::common::{ComponentManifest, ComponentTypeInManifest};
use crate::lib::codegen::components::event_indexer::EventIndexerComponentManifest;
use crate::lib::codegen::components::relayer::RelayerComponentManifest;
use crate::lib::codegen::components::snapshot::SnapshotComponentManifest;
use crate::lib::codegen::oracle::get_oracle_attributes;
use crate::lib::utils::find_duplicates;
use crate::{
    lib::{
        codegen::project::ProjectManifestData,
        environment::EnvironmentImpl,
        utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME},
    },
    types::ComponentType,
};

#[derive(Debug, Parser)]
#[command(name = "build")]
/// Builds your project to generate canisters' modules for Chainsight.
pub struct BuildOpts {
    /// Specify the path of the project to be built.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    path: Option<String>,

    /// Only perform code generation.
    #[arg(long, conflicts_with = "only_build")]
    only_codegen: bool,

    /// Only perform build.
    /// Perform this steps with code already generated.
    #[arg(long, conflicts_with = "only_codegen")]
    only_build: bool,
}

const GLOBAL_ERROR_MSG: &str = "Fail 'Build' command";

pub fn exec(env: &EnvironmentImpl, opts: BuildOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        error!(log, r#"{}"#, msg);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    info!(log, r#"Building project..."#);

    let project_path_str = project_path.unwrap_or(".".to_string());
    let artifacts_path_str = format!("{}/artifacts", &project_path_str);

    // load component definitions from manifests
    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;

    // check duplicated component pathes
    {
        let component_paths = project_manifest
            .components
            .iter()
            .map(|c| c.component_path.to_string())
            .collect::<Vec<String>>();
        let duplicated_pathes = find_duplicates(&component_paths);
        if !duplicated_pathes.is_empty() {
            error!(
                log,
                r#"Duplicated component pathes found: {:?}"#, duplicated_pathes
            );
            bail!(GLOBAL_ERROR_MSG.to_string())
        }
    }

    let mut component_data = vec![];
    for component in project_manifest.components.clone() {
        // TODO: need validations
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", &project_path_str, relative_component_path);
        let component_type = ComponentTypeInManifest::determine_type(&component_path)?;

        let data: Box<dyn ComponentManifest> = match component_type {
            ComponentType::EventIndexer => {
                Box::new(EventIndexerComponentManifest::load(&component_path)?)
            }
            ComponentType::Snapshot => Box::new(SnapshotComponentManifest::load(&component_path)?),
            ComponentType::Relayer => Box::new(RelayerComponentManifest::load(&component_path)?),
        };
        component_data.push(data);
    }

    if opts.only_build {
        info!(log, r#"Skip codegen"#);
    } else {
        // generate codes
        info!(log, r#"Processing for codegen"#);
        exec_codegen(log, &project_path_str, &artifacts_path_str, &component_data)?;
    }

    if opts.only_codegen {
        info!(log, r#"Skip build"#);
    } else {
        // build codes generated
        info!(log, r#"Processing for build"#);
        execute_codebuild(log, &artifacts_path_str, &component_data)?;
    }

    info!(
        log,
        r#"Project "{}" codes/resources generated successfully"#, project_manifest.label
    );

    info!(
        log,
        r#"Project "{}" built successfully"#, project_manifest.label
    );
    Ok(())
}

fn exec_codegen(
    log: &Logger,
    project_path_str: &str,
    artifacts_path_str: &str,
    component_data: &Vec<Box<dyn ComponentManifest>>,
) -> anyhow::Result<()> {
    // generate /artifacts
    let artifacts_path = Path::new(&artifacts_path_str);
    if artifacts_path.exists() {
        fs::remove_dir_all(artifacts_path)?;
    }
    fs::create_dir(artifacts_path)?;

    // generate /artifacts/__interfaces
    let interfaces_path_str = format!("{}/__interfaces", &artifacts_path_str);
    fs::create_dir(&interfaces_path_str)?;

    // generate canister codes
    let mut project_labels: Vec<String> = vec![];
    for data in component_data {
        if let Err(msg) = data.validate_manifest() {
            error!(log, r#"{}"#, msg);
            bail!(GLOBAL_ERROR_MSG.to_string())
        }

        // Processes about interface
        // - copy and move any necessary interfaces to canister
        // - get ethabi::Contract for codegen
        let mut interface_contract: Option<ethabi::Contract> = None;
        if let Some(interface_file) = data.required_interface() {
            let dst_interface_path_str = format!("{}/{}", &interfaces_path_str, &interface_file);
            let dst_interface_path = Path::new(&dst_interface_path_str);

            let user_if_file_path_str =
                format!("{}/interfaces/{}", &project_path_str, &interface_file);
            let user_if_file_path = Path::new(&user_if_file_path_str);
            if user_if_file_path.exists() {
                fs::copy(user_if_file_path, dst_interface_path)?;
                info!(
                    log,
                    r#"Interface file "{}" copied by user's interface"#, &interface_file
                );
                let abi_file = File::open(user_if_file_path)?;
                interface_contract = Some(ethabi::Contract::load(abi_file)?);
            } else if let Some(contents) = buildin_interface(&interface_file) {
                fs::write(dst_interface_path, contents)?;
                info!(
                    log,
                    r#"Interface file "{}" copied by builtin interface"#, &interface_file
                );
                let contract: ethabi::Contract = serde_json::from_str(contents)?;
                interface_contract = Some(contract);
            } else {
                error!(log, r#"Interface file "{}" not found"#, &interface_file);
                bail!(GLOBAL_ERROR_MSG.to_string())
            }
        }

        let label = data.metadata().label.clone();
        project_labels.push(label.to_string());
        let canister_pj_path_str = format!("{}/artifacts/{}", &project_path_str, label);
        let canister_code_path_str = format!("{}/src", &canister_pj_path_str);
        fs::create_dir_all(Path::new(&canister_code_path_str))?;
        let lib_path_str = format!("{}/lib.rs", &canister_code_path_str);
        let mut lib_file = File::create(&lib_path_str)?;
        lib_file.write_all(
            data.generate_codes(interface_contract)?
                .to_string()
                .as_bytes(),
        )?;

        // generate project's Cargo.toml
        fs::write(
            format!("{}/Cargo.toml", &canister_pj_path_str),
            &canister_project_cargo_toml(&label),
        )?;

        // copy and move oracle interface
        if let Some(value) = data.destination_type() {
            let (_, json_name, json_contents) = get_oracle_attributes(&value);
            fs::write(
                format!("{}/{}", &interfaces_path_str, &json_name),
                json_contents,
            )?;
            info!(
                log,
                r#"Interface file "{}" copied by builtin interface"#, &json_name
            );
        }
    }
    fs::write(
        format!("{}/Cargo.toml", &artifacts_path_str),
        root_cargo_toml(project_labels.clone()),
    )?;
    fs::write(
        format!("{}/dfx.json", &artifacts_path_str),
        dfx_json(project_labels),
    )?;
    fs::write(
        format!("{}/Makefile.toml", &artifacts_path_str),
        makefile_toml(),
    )?;

    anyhow::Ok(())
}

fn root_cargo_toml(members: Vec<String>) -> String {
    let members = members
        .iter()
        .map(|member| format!("\t\"{}\",", member))
        .collect::<Vec<String>>()
        .join("\n");

    let txt = format!("[workspace]
members = [
{}
]

[workspace.dependencies]
candid = \"0.8\"
ic-cdk = \"0.8\"
ic-cdk-macros = \"0.6.10\"
ic-cdk-timers = \"0.1\"
serde = \"1.0.163\"
hex = \"0.4.3\"

ic-web3-rs = {{ version = \"0.1.1\" }}
ic-solidity-bindgen = {{ version = \"0.1.5\" }}
chainsight-cdk-macros = {{ git = \"https://github.com/horizonx-tech/chainsight-sdk.git\", rev = \"944449a81d52121463bb046041c833bc0158c87b\" }}
chainsight-cdk = {{ git = \"https://github.com/horizonx-tech/chainsight-sdk.git\", rev = \"944449a81d52121463bb046041c833bc0158c87b\" }}", members);

    txt
}

fn canister_project_cargo_toml(project_name: &str) -> String {
    let txt = format!(
        "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-cdk-macros.workspace = true
ic-cdk-timers.workspace = true
serde.workspace = true
hex.workspace = true

ic-web3-rs.workspace = true
ic-solidity-bindgen.workspace = true
chainsight-cdk-macros.workspace = true
chainsight-cdk.workspace = true",
        project_name
    );

    txt
}

fn dfx_json(project_labels: Vec<String>) -> String {
    let canisters = project_labels
        .iter()
        .map(|label| {
            format!(
                "\t\t\t\"{}\": {{
\t\t\t\t\"type\": \"custom\",
\t\t\t\t\"candid\": \"artifacts/{}.did\",
\t\t\t\t\"wasm\": \"artifacts/{}.wasm\",
\t\t\t\t\"metadata\": [
\t\t\t\t\t{{
\t\t\t\t\t\t\"name\": \"candid:service\",
\t\t\t\t\t\t\"visibility\": \"public\"
\t\t\t\t\t}}
\t\t\t\t]
\t\t\t}}",
                label, label, label
            )
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let result = format!(
        r#"{{
    "version": 1,
    "canisters": {{
{}
    }},
    "defaults": {{
        "build": {{
            "packtool": "",
            "args": ""
        }}
    }},
    "output_env_file": ".env"
}}"#,
        canisters
    );

    result
}

fn makefile_toml() -> String {
    let txt = "[tasks.gen]
description = \"generate .ts and .did\"
workspace = false
command = \"dfx\"
args = [\"generate\"]
dependencies = [\"did\"]

[tasks.did]
description = \"generate .did\"
workspace = false
command = \"cargo\"
args= [\"test\"]";

    txt.to_string()
}

fn buildin_interface(name: &str) -> Option<&'static str> {
    let interface = match name {
        "ERC20.json" => include_str!("../../resources/ERC20.json"),
        _ => return None,
    };

    Some(interface)
}

fn execute_codebuild(
    log: &Logger,
    built_project_path_str: &str,
    component_data: &Vec<Box<dyn ComponentManifest>>,
) -> anyhow::Result<()> {
    let built_project_path = Path::new(&built_project_path_str);

    let description = "Generate interfaces (.did files)";
    info!(log, "{}", description);
    let output = Command::new("cargo")
        .current_dir(built_project_path)
        .args(["make", "did"])
        .output()
        .expect("failed to execute process: cargo make did");
    if output.status.success() {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or("fail to parse stdout")
        );
        info!(log, "{} successfully", description);
    } else {
        error!(log, "{} failed", description);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    // Regenerate artifacts folder
    let build_artifact_path_str = format!("{}/artifacts", built_project_path_str);
    let build_artifact_path = Path::new(&build_artifact_path_str);
    if build_artifact_path.exists() {
        fs::remove_dir_all(build_artifact_path)?;
    }
    fs::create_dir(build_artifact_path)?;
    // Copy .did to artifacts folder
    for component_datum in component_data {
        let label = component_datum.metadata().label.clone();
        let src_path = format!("{}/{}/{}.did", built_project_path_str, label, label);
        let dst_path = format!("{}/{}.did", build_artifact_path_str, label);
        fs::copy(src_path, dst_path)?;
    }

    let description = "Compile canisters' codes";
    info!(log, "{}", description);
    let output = Command::new("cargo")
        .current_dir(built_project_path)
        .args([
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
            "--workspace",
        ])
        .output()
        .expect(
            "failed to execute process: cargo build --target wasm32-unknown-unknown --workspace",
        );
    if output.status.success() {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or("fail to parse stdout")
        );
        info!(log, "{} successfully", description);
    } else {
        error!(log, "{} failed", description);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    let description = "Shrink/Optimize canisters' modules";
    info!(log, "{}", description);
    for component_datum in component_data {
        let label = component_datum.metadata().label.clone();
        let wasm_path = format!("target/wasm32-unknown-unknown/release/{}.wasm", label);
        let output_path = format!("artifacts/{}.wasm", label);
        let output = Command::new("ic-wasm")
            .current_dir(built_project_path)
            .args([&wasm_path, "-o", &output_path, "shrink"])
            .output()
            .expect("failed to execute process: ic_wasm shrink");
        if output.status.success() {
            debug!(
                log,
                "{}",
                std::str::from_utf8(&output.stdout).unwrap_or("fail to parse stdout")
            );
            info!(log, "{} `{}` successfully", label, description);
        } else {
            debug!(
                log,
                "{}",
                std::str::from_utf8(&output.stderr).unwrap_or("fail to parse stdout")
            );
            error!(log, "{} `{}` failed", label, description);
            bail!(GLOBAL_ERROR_MSG.to_string())
        }
    }

    let description = "Add metadatas to canisters' modules";
    info!(log, "{}", description);
    for component_datum in component_data {
        add_metadatas_to_wasm(log, built_project_path_str, component_datum.as_ref())?;
    }

    anyhow::Ok(())
}

fn add_metadatas_to_wasm(
    log: &Logger,
    built_project_path_str: &str,
    component_datum: &dyn ComponentManifest,
) -> anyhow::Result<()> {
    let built_project_path = Path::new(&built_project_path_str);

    let label = component_datum.metadata().label.clone();
    let wasm_path = format!("artifacts/{}.wasm", label);

    // chainsight:label
    let description = "Add 'chainsight:label' metadata";
    let output = Command::new("ic-wasm")
        .current_dir(built_project_path)
        .args([
            &wasm_path,
            "-o",
            &wasm_path,
            "metadata",
            "chainsight:label",
            "-d",
            &label,
            "-v",
            "public",
        ])
        .output()
        .expect("failed to execute process: ic-wasm metadata chainsight:label");
    if output.status.success() {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or("fail to parse stdout")
        );
        info!(log, "{} `{}` successfully", label, description);
    } else {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stderr).unwrap_or("fail to parse stdout")
        );
        error!(log, "{} `{}` failed", label, description);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }
    // chainsight:component_type
    let description = "Add 'chainsight:component_type' metadata";
    let output = Command::new("ic-wasm")
        .current_dir(built_project_path)
        .args([
            &wasm_path,
            "-o",
            &wasm_path,
            "metadata",
            "chainsight:component_type",
            "-d",
            &component_datum.component_type().to_string(),
            "-v",
            "public",
        ])
        .output()
        .expect("failed to execute process: ic-wasm metadata chainsight:component_type");
    if output.status.success() {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or("fail to parse stdout")
        );
        info!(log, "{} `{}` successfully", label, description);
    } else {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stderr).unwrap_or("fail to parse stdout")
        );
        error!(log, "{} `{}` failed", label, description);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }
    // chainsight:description
    let description = "Add 'chainsight:description' metadata";
    let output = Command::new("ic-wasm")
        .current_dir(built_project_path)
        .args([
            &wasm_path,
            "-o",
            &wasm_path,
            "metadata",
            "chainsight:description",
            "-d",
            &component_datum.metadata().description,
            "-v",
            "public",
        ])
        .output()
        .expect("failed to execute process: ic-wasm metadata chainsight:description");
    if output.status.success() {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or("fail to parse stdout")
        );
        info!(log, "{} `{}` successfully", label, description);
    } else {
        debug!(
            log,
            "{}",
            std::str::from_utf8(&output.stderr).unwrap_or("fail to parse stdout")
        );
        error!(log, "{} `{}` failed", label, description);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    anyhow::Ok(())
}
