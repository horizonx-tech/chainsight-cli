use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{path::Path, fs};
use std::fmt::Debug;

use anyhow::{Ok, bail};
use clap::Parser;
use slog::{info, error, debug, Logger};

use crate::lib::codegen::components::common::{ComponentManifest, ComponentTypeInManifest};
use crate::lib::codegen::components::event_indexer::EventIndexerComponentManifest;
use crate::lib::codegen::components::relayer::RelayerComponentManifest;
use crate::lib::codegen::components::snapshot::SnapshotComponentManifest;
use crate::lib::codegen::oracle::get_oracle_attributes;
use crate::{lib::{environment::EnvironmentImpl, utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME}, codegen::{project::ProjectManifestData}}, types::ComponentType};

#[derive(Debug, Parser)]
#[command(name = "build")]
/// Build your ChainSight's project
pub struct BuildOpts {
    #[arg(long)]
    path: Option<String>,
    #[arg(long, conflicts_with = "only_build")]
    only_codegen: bool,
    #[arg(long, conflicts_with = "only_codegen")]
    only_build: bool,
}

const GLOBAL_ERROR_MSG: &str = "Fail build command";

pub fn exec(env: &EnvironmentImpl, opts: BuildOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        error!(
            log,
            r#"{}"#,
            msg
        );
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    info!(
        log,
        r#"Building project..."#
    );

    let project_path_str = project_path.unwrap_or(".".to_string());
    let artifacts_path_str = format!("{}/artifacts", &project_path_str);

    // load component definitions from manifests
    let project_manifest = ProjectManifestData::load(&format!("{}/{}", &project_path_str, PROJECT_MANIFEST_FILENAME))?;
    let mut component_data = vec![];
    for component in project_manifest.components.clone() {
        // TODO: need validations
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", &project_path_str, relative_component_path);
        let component_type = ComponentTypeInManifest::determine_type(&component_path)?;

        let data: Box<dyn ComponentManifest> = match component_type {
            ComponentType::EventIndexer => Box::new(EventIndexerComponentManifest::load(&component_path)?),
            ComponentType::Snapshot => Box::new(SnapshotComponentManifest::load(&component_path)?),
            ComponentType::Relayer => Box::new(RelayerComponentManifest::load(&component_path)?),
        };
        component_data.push(data);
    };

    if opts.only_build {
        info!(log, r#"Skip codegen"#);
    } else {
        // generate codes
        exec_codegen(log, &project_path_str, &artifacts_path_str, &component_data)?;
    }

    if opts.only_codegen {
        info!(log, r#"Skip build"#);
    } else {
        // build codes generated
        let component_names = &component_data.iter().map(|data| data.label().to_string()).collect::<Vec<String>>();
        execute_codebuild(log, &artifacts_path_str, component_names.clone())?;
    }

    info!(
        log,
        r#"Project "{}" codes/resources generated successfully"#,
        project_manifest.label
    );

    info!(
        log,
        r#"Project "{}" builded successfully"#,
        project_manifest.label
    );
    Ok(())
}

fn exec_codegen(log: &Logger, project_path_str: &str, artifacts_path_str: &str, component_data: &Vec<Box<dyn ComponentManifest>>) -> anyhow::Result<()> {
    // generate /artifacts
    let artifacts_path = Path::new(&artifacts_path_str);
    if artifacts_path.exists() {
        fs::remove_dir_all(&artifacts_path)?;
    }
    fs::create_dir(&artifacts_path)?;

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

            let user_if_file_path_str = format!("{}/interfaces/{}", &project_path_str, &interface_file);
            let user_if_file_path = Path::new(&user_if_file_path_str);
            if user_if_file_path.exists() {
                fs::copy(&user_if_file_path, dst_interface_path)?;
                info!(
                    log,
                    r#"Interface file "{}" copied by user's interface"#,
                    &interface_file
                );
                let abi_file = File::open(&user_if_file_path)?;
                interface_contract = Some(ethabi::Contract::load(abi_file)?);
            } else if let Some(contents) = buildin_interface(&interface_file) {
                fs::write(&dst_interface_path, contents)?;
                info!(
                    log,
                    r#"Interface file "{}" copied by builtin interface"#,
                    &interface_file
                );
                let contract: ethabi::Contract = serde_json::from_str(contents)?;
                interface_contract = Some(contract);
            } else {
                error!(
                    log,
                    r#"Interface file "{}" not found"#,
                    &interface_file
                );
                bail!(GLOBAL_ERROR_MSG.to_string())
            }
        }

        let label = data.label();
        project_labels.push(label.to_string());
        let canister_pj_path_str = format!("{}/artifacts/{}", &project_path_str, label);
        let canister_code_path_str = format!("{}/src", &canister_pj_path_str);
        fs::create_dir_all(Path::new(&canister_code_path_str))?;
        let lib_path_str = format!("{}/lib.rs", &canister_code_path_str);
        let mut lib_file = File::create(&lib_path_str)?;
        lib_file.write_all(data.generate_codes(interface_contract)?.to_string().as_bytes())?;

        // generate project's Cargo.toml
        fs::write(
            format!("{}/Cargo.toml", &canister_pj_path_str),
            &canister_project_cargo_toml(label)
        )?;

        // copy and move oracle interface
        if let Some(value) = data.destination_type() {
            let (_, json_name, json_contents) = get_oracle_attributes(&value);
            fs::write(
                format!("{}/{}", &interfaces_path_str, &json_name),
                json_contents
            )?;
            info!(
                log,
                r#"Interface file "{}" copied by builtin interface"#,
                &json_name
            );
        }
    }
    fs::write(
        format!("{}/Cargo.toml", &artifacts_path_str),
        &root_cargo_toml(project_labels.clone())
    )?;
    fs::write(
        format!("{}/dfx.json", &artifacts_path_str),
        &dfx_json(project_labels)
    )?;
    fs::write(
        format!("{}/Makefile.toml", &artifacts_path_str),
        &makefile_toml()
    )?;

    anyhow::Ok(())
}

fn root_cargo_toml(members: Vec<String>) -> String {
    let members = members.iter().map(|member| format!("\t\"{}\",", member)).collect::<Vec<String>>().join("\n");

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
chainsight-cdk-macros = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/chainsight-sdk.git\", rev = \"17e65f7759c4fa8a6c3f397ef5ef7a454d276cea\" }}
chainsight-cdk = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/chainsight-sdk.git\", rev = \"17e65f7759c4fa8a6c3f397ef5ef7a454d276cea\" }}", members);

    txt
}

fn canister_project_cargo_toml(project_name: &str) -> String {
    let txt = format!("[package]
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
chainsight-cdk.workspace = true", project_name);

    txt
}

fn dfx_json(project_labels: Vec<String>) -> String {
    let canisters = project_labels.iter().map(|label| format!("\t\t\t\"{}\": {{
\t\t\t\t\"type\": \"rust\",
\t\t\t\t\"package\": \"{}\",
\t\t\t\t\"candid\": \"{}/{}.did\"
\t\t\t}}", label, label, label, label)).collect::<Vec<String>>().join(",\n");

    let result = format!(r#"{{
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
}}"#, canisters);

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

fn execute_codebuild(log: &Logger, builded_project_path_str: &str, component_names: Vec<String>) -> anyhow::Result<()> {
    let builded_project_path = Path::new(&builded_project_path_str);

    let description = "Generate interfaces (.did files)";
    info!(log, "{}", description);
    let output = Command::new("cargo")
        .current_dir(&builded_project_path)
        .args(["make", "did"])
        .output()
        .expect("failed to execute process: cargo make did");
    if output.status.success() {
        debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
        info!(log, "{} successfully", description);
    } else {
        error!(log, "{} failed", description);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    // Regenerate artifacts folder
    let build_artifact_path_str = format!("{}/artifacts", builded_project_path_str);
    let build_artifact_path = Path::new(&build_artifact_path_str);
    if build_artifact_path.exists() {
        fs::remove_dir_all(&build_artifact_path)?;
    }
    fs::create_dir(&build_artifact_path)?;
    // Copy .did to artifacts folder
    for component_name in &component_names {
        let src_path = format!("{}/{}/{}.did", builded_project_path_str, component_name, component_name);
        let dst_path = format!("{}/{}.did", build_artifact_path_str, component_name);
        fs::copy(src_path, dst_path)?;
    }

    let description = "Compile canisters' codes";
    info!(log, "{}", description);
    let output = Command::new("cargo")
        .current_dir(&builded_project_path)
        .args(["build", "--target", "wasm32-unknown-unknown", "--release", "--workspace"])
        .output()
        .expect("failed to execute process: cargo build --target wasm32-unknown-unknown --workspace");
    if output.status.success() {
        debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
        info!(log, "{} successfully", description);
    } else {
        error!(log, "{} failed", description);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    let description = "Shrink/Optimize canisters' modules";
    info!(log, "{}", description);
    for component_name in &component_names {
        let wasm_path = format!("target/wasm32-unknown-unknown/release/{}.wasm", component_name);
        let output_path = format!("artifacts/{}.wasm", component_name);
        let output = Command::new("ic-wasm")
            .current_dir(&builded_project_path)
            .args([&wasm_path, "-o", &output_path, "shrink"])
            .output()
            .expect("failed to execute process: ic_wasm shrink");
        if output.status.success() {
            debug!(log, "{}", std::str::from_utf8(&output.stdout).unwrap_or(&"fail to parse stdout"));
            info!(log, "{} `{}` successfully", component_name, description);
        } else {
            debug!(log, "{}", std::str::from_utf8(&output.stderr).unwrap_or(&"fail to parse stdout"));
            error!(log, "{} `{}` failed", component_name, description);
            bail!(GLOBAL_ERROR_MSG.to_string())
        }
    }

    anyhow::Ok(())
}
