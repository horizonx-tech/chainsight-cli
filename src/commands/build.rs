use std::fs::File;
use std::io::Write;
use std::{path::Path, fs};
use std::fmt::Debug;

use anyhow::{Ok, bail};
use clap::Parser;
use slog::{info, error};

use crate::lib::codegen::components::common::{get_type_from_manifest, DestinactionType, ComponentManifest};
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

    // generate /artifacts
    let project_path_str = project_path.unwrap_or(".".to_string());
    let artifacts_path_str = format!("{}/artifacts", &project_path_str);
    let artifacts_path = Path::new(&artifacts_path_str);
    if artifacts_path.exists() {
        fs::remove_dir_all(&artifacts_path)?;
    }
    fs::create_dir(&artifacts_path)?;

    // generate /artifacts/__interfaces
    let interfaces_path_str = format!("{}/__interfaces", &artifacts_path_str);
    fs::create_dir(&interfaces_path_str)?;

    // generate canister codes & project folder (/artifacts/__interfaces/{project})
    let project_manifest = ProjectManifestData::load(&format!("{}/{}", &project_path_str, PROJECT_MANIFEST_FILENAME))?;
    let mut project_labels: Vec<String> = vec![];
    for component in project_manifest.components {
        // TODO: need validations
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", &project_path_str, relative_component_path);
        let component_type = get_type_from_manifest(&component_path)?;

        let (label, interface_path, destination_type, data): (String, Option<String>, Option<DestinactionType>, Box<dyn ComponentManifest>) = match component_type {
            ComponentType::Snapshot => {
                let manifest = SnapshotComponentManifest::load(&component_path).unwrap();
                (
                    manifest.clone().label,
                    manifest.clone().datasource.method.interface,
                    None,
                    Box::new(manifest),
                )
            },
            ComponentType::Relayer => {
                let manifest = RelayerComponentManifest::load(&component_path).unwrap();
                (
                    manifest.clone().label,
                    manifest.clone().datasource.method.interface,
                    Some(manifest.clone().destination.type_),
                    Box::new(manifest),
                )
            },
        };

        project_labels.push(label.clone());
        let canister_pj_path_str = format!("{}/artifacts/{}", &project_path_str, &label);
        let canister_code_path_str = format!("{}/src", &canister_pj_path_str);
        fs::create_dir_all(Path::new(&canister_code_path_str))?;
        let lib_path_str = format!("{}/lib.rs", &canister_code_path_str);
        let mut lib_file = File::create(&lib_path_str)?;
        lib_file.write_all(data.generate_codes()?.to_string().as_bytes())?;

        // generate project's Cargo.toml
        fs::write(
            format!("{}/Cargo.toml", &canister_pj_path_str),
            &canister_project_cargo_toml(&label)
        )?;

        // copy and move any necessary interfaces to canister
        if let Some(interface_file) = interface_path {
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
            } else if let Some(contents) = buildin_interface(&interface_file) {
                fs::write(&dst_interface_path, contents)?;
                info!(
                    log,
                    r#"Interface file "{}" copied by builtin interface"#,
                    &interface_file
                );
            } else {
                error!(
                    log,
                    r#"Interface file "{}" not found"#,
                    &interface_file
                );
                bail!(GLOBAL_ERROR_MSG.to_string())
            }
        }

        // copy and move oracle interface
        if let Some(value) = destination_type {
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

fn root_cargo_toml(members: Vec<String>) -> String {
    let members = members.iter().map(|member| format!("\t\"{}\",", member)).collect::<Vec<String>>().join("\n");

    let txt = format!("[workspace]
members = [
{}
]

[workspace.dependencies]
candid = \"0.8\"
ic-cdk = \"0.7\"
ic-cdk-macros = \"0.6.10\"
ic-cdk-timers = \"0.1\"
serde = \"1.0.163\"
hex = \"0.4.3\"

ic-web3 = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/ic-web3.git\", rev = \"fa982360c8420c88887606355eff0b7b48208a01\" }}
ic-solidity-bindgen = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/ic-solidity-bindgen.git\", rev = \"36a7e1044c261a91a715fbdc12d95afa69eb0620\" }}
ic-solidity-bindgen-macros = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/ic-solidity-bindgen.git\", rev = \"36a7e1044c261a91a715fbdc12d95afa69eb0620\" }}
chainsight-cdk-macros = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/chainsight-sdk.git\", rev = \"843683fa588da751e2ac613a4dbb16cb250f0d05\" }}
chainsight-cdk = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/chainsight-sdk.git\", rev = \"843683fa588da751e2ac613a4dbb16cb250f0d05\" }}

[patch.'https://github.com/horizonx-tech/ic-web3']
ic-web3 = {{ git = \"ssh://<your host in .ssh/config>/horizonx-tech/ic-web3.git\", rev = \"fa982360c8420c88887606355eff0b7b48208a01\" }}", members);

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

ic-web3.workspace = true
ic-solidity-bindgen.workspace = true
ic-solidity-bindgen-macros.workspace = true
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
