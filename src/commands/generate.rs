use std::fmt::Debug;
use std::fs::File;
use std::process::Command;
use std::{fs, path::Path};

use anyhow::{bail, Ok};
use clap::Parser;
use slog::{debug, info, Logger};

use crate::lib::codegen::bindings::generate_rs_bindings;
use crate::lib::codegen::components::algorithm_indexer::AlgorithmIndexerComponentManifest;
use crate::lib::codegen::components::algorithm_lens::AlgorithmLensComponentManifest;
use crate::lib::codegen::components::common::{ComponentManifest, ComponentTypeInManifest};
use crate::lib::codegen::components::event_indexer::EventIndexerComponentManifest;
use crate::lib::codegen::components::relayer::RelayerComponentManifest;
use crate::lib::codegen::components::snapshot_indexer::SnapshotIndexerComponentManifest;
use crate::lib::codegen::components::snapshot_indexer_https::SnapshotIndexerHTTPSComponentManifest;
use crate::lib::codegen::oracle::get_oracle_attributes;
use crate::lib::codegen::templates::{
    bindings_cargo_toml, canister_project_cargo_toml, logic_cargo_toml, root_cargo_toml,
};
use crate::lib::utils::{find_duplicates, paths};
use crate::{
    lib::{
        codegen::project::ProjectManifestData,
        environment::EnvironmentImpl,
        utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME},
    },
    types::ComponentType,
};

fn dummy_candid_blob() -> String {
    include_str!("../../resources/sample.did").to_string()
}

#[derive(Debug, Parser)]
#[command(name = "generate")]
#[clap(visible_alias = "gen")]
/// Generate codes according to project/component manifests.
pub struct GenerateOpts {
    /// Specify the path of the project.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    path: Option<String>,
}

impl GenerateOpts {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

pub fn exec(env: &EnvironmentImpl, opts: GenerateOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        bail!(format!(r#"{}"#, msg));
    }

    let project_path_str = project_path.unwrap_or(".".to_string());

    // load component definitions from manifests
    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;

    info!(
        log,
        r#"Start code generation for project '{}'"#, project_manifest.label
    );

    // check duplicated component paths
    {
        let component_paths = project_manifest
            .components
            .iter()
            .map(|c| c.component_path.to_string())
            .collect::<Vec<String>>();
        let duplicated_paths = find_duplicates(&component_paths);
        if !duplicated_paths.is_empty() {
            bail!(format!(
                r#"Duplicated component paths found: {:?}"#,
                duplicated_paths
            ));
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
            ComponentType::AlgorithmIndexer => {
                Box::new(AlgorithmIndexerComponentManifest::load(&component_path)?)
            }
            ComponentType::SnapshotIndexer => {
                Box::new(SnapshotIndexerComponentManifest::load(&component_path)?)
            }
            ComponentType::Relayer => Box::new(RelayerComponentManifest::load(&component_path)?),
            ComponentType::AlgorithmLens => {
                Box::new(AlgorithmLensComponentManifest::load(&component_path)?)
            }
            ComponentType::SnapshotIndexerHTTPS => Box::new(
                SnapshotIndexerHTTPSComponentManifest::load(&component_path)?,
            ),
        };
        component_data.push(data);
    }

    exec_codegen(log, &project_path_str, &component_data)?;

    info!(
        log,
        r#"Project '{}' codes/resources generated successfully"#, project_manifest.label
    );
    Ok(())
}

fn exec_codegen(
    log: &Logger,
    project_path_str: &str,
    component_data: &Vec<Box<dyn ComponentManifest>>,
) -> anyhow::Result<()> {
    // generate workspace
    let src_path_str = &paths::src_path_str(project_path_str);
    fs::create_dir_all(&format!("{}", src_path_str)).expect("failed to create dir: src");
    if !Path::new(&format!("{}/Cargo.toml", src_path_str)).is_file() {
        fs::write(format!("{}/Cargo.toml", src_path_str), root_cargo_toml())?;
    } else {
        info!(
            log,
            r#"Skip creating workspace: '{}/Cargo.toml' already exists"#, src_path_str
        )
    }

    // remove generated files
    let _ = fs::remove_dir_all(format!("{}/__interfaces", src_path_str));
    let _ = fs::remove_dir_all(format!("{}/bindings", src_path_str));
    let _ = fs::remove_dir_all(format!("{}/canisters", src_path_str));

    // generate /artifacts/__interfaces
    let interfaces_path_str = format!("{}/__interfaces", src_path_str);
    fs::create_dir(&interfaces_path_str)?;
    let dummy_candid_file_path = format!("{}/interfaces/{}", project_path_str, "sample.did");
    if !Path::new(&dummy_candid_file_path).is_file() {
        fs::write(&dummy_candid_file_path, dummy_candid_blob())?;
    }

    // generate canister projects
    for data in component_data {
        let label = data.metadata().label.to_string();

        if let Err(msg) = data.validate_manifest() {
            bail!(format!(r#"[{}] Invalid manifest: {}"#, label, msg));
        }
        info!(log, r#"[{}] Start processing..."#, label);

        // logic template
        let logic_path_str = &paths::logics_path_str(src_path_str, &label);
        if Path::new(logic_path_str).is_dir() {
            info!(
                log,
                r#"[{}] Skip creating logic project: '{}' already exists"#, label, logic_path_str,
            );
        } else {
            create_cargo_project(
                logic_path_str,
                Option::Some(&logic_cargo_toml(&label, data.dependencies())),
                Option::Some(
                    &data
                        .generate_user_impl_template()
                        .unwrap_or_default()
                        .to_string(),
                ),
            )
            .map_err(|err| {
                anyhow::anyhow!(r#"[{}] Failed to create logic project by: {}"#, label, err)
            })?;
        }

        // Processes about interface
        // - copy and move any necessary interfaces to canister
        // - get ethabi::Contract for codegen
        let mut interface_contract: Option<ethabi::Contract> = None;
        if let Some(interface_file) = data.required_interface() {
            let dst_interface_path_str = format!("{}/{}", &interfaces_path_str, &interface_file);
            let dst_interface_path = Path::new(&dst_interface_path_str);

            let user_if_file_path_str =
                format!("{}/interfaces/{}", project_path_str, &interface_file);
            let user_if_file_path = Path::new(&user_if_file_path_str);
            if user_if_file_path.exists() {
                fs::copy(user_if_file_path, dst_interface_path)?;
                info!(
                    log,
                    r#"[{}] Interface file '{}' copied from user's interface"#,
                    label,
                    &interface_file
                );
                let abi_file = File::open(user_if_file_path)?;
                interface_contract = Some(ethabi::Contract::load(abi_file)?);
            } else if let Some(contents) = builtin_interface(&interface_file) {
                fs::write(dst_interface_path, contents)?;
                info!(
                    log,
                    r#"[{}] Interface file '{}' copied from builtin interface"#,
                    label,
                    &interface_file
                );
                let contract: ethabi::Contract = serde_json::from_str(contents)?;
                interface_contract = Some(contract);
            } else {
                bail!(format!(
                    r#"[{}] Interface file "{}" not found"#,
                    label, &interface_file
                ));
            }
        }

        // copy and move oracle interface
        if let Some(value) = data.destination_type() {
            let (_, json_name, json_contents) = get_oracle_attributes(&value);
            fs::write(
                format!("{}/{}", &interfaces_path_str, &json_name),
                json_contents,
            )?;
            info!(
                log,
                r#"[{}] Interface file '{}' copied from builtin interface"#, label, &json_name
            );
        }

        // canister
        let canister_pj_path_str = &paths::canisters_path_str(src_path_str, &label);
        let canister_src = &data.generate_codes(interface_contract).map_err(|err| {
            anyhow::anyhow!(
                r#"[{}] Failed to generate canister code by: {}"#,
                label,
                err
            )
        })?;
        create_cargo_project(
            canister_pj_path_str,
            Option::Some(&canister_project_cargo_toml(&label)),
            Option::Some(&canister_src.to_string()),
        )
        .map_err(|err| {
            anyhow::anyhow!(
                r#"[{}] Failed to create canister project by: {}"#,
                label,
                err
            )
        })?;

        // generate dummy bindings to be able to run cargo test
        let bindings_path_str = &paths::bindings_path_str(src_path_str, &label);
        create_cargo_project(
            bindings_path_str,
            Option::Some(&bindings_cargo_toml(&label)),
            Option::None,
        )
        .map_err(|err| {
            anyhow::anyhow!(
                r#"[{}] Failed to create bindings project by: {}"#,
                label,
                err
            )
        })?;
    }

    // generate canister bindings
    let action = "Generate interfaces (.did files)";
    info!(log, "{}...", action);
    for data in component_data {
        let label = data.metadata().label.to_string();
        let canisters_path_str = paths::canisters_path_str(src_path_str, &label);

        let output = Command::new("cargo")
            .current_dir(canisters_path_str)
            .args(["test"])
            .output()
            .expect("failed to execute: cargo test");

        if output.status.success() {
            debug!(
                log,
                "{}",
                std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
            );
            info!(log, r#"[{}] Succeeded: {}"#, label, action);
        } else {
            bail!(format!(
                r#"[{}] Failed: {} by: {}"#,
                label,
                action,
                std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stdout")
            ));
        }

        // TODO handle errors
        let bindings = generate_rs_bindings(src_path_str, data)?;
        fs::write(
            Path::new(&format!(
                "{}/src/lib.rs",
                paths::bindings_path_str(src_path_str, &label)
            )),
            bindings,
        )
        .expect("failed to execute: write bindings");
    }

    anyhow::Ok(())
}

fn builtin_interface(name: &str) -> Option<&'static str> {
    let interface = match name {
        "ERC20.json" => include_str!("../../resources/ERC20.json"),
        _ => return None,
    };

    Some(interface)
}

pub fn create_cargo_project(
    path_str: &str,
    manifest: Option<&str>,
    src: Option<&str>,
) -> anyhow::Result<()> {
    fs::create_dir_all(Path::new(&format!("{}/src", path_str)))?;
    fs::write(
        Path::new(&format!("{}/Cargo.toml", path_str)),
        manifest.unwrap_or_default(),
    )?;
    fs::write(
        Path::new(&format!("{}/src/lib.rs", path_str)),
        src.unwrap_or_default(),
    )?;
    Ok(())
}
