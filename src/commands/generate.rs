use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::process::Command;
use std::{fs, path::Path};

use anyhow::{bail, Ok};
use clap::Parser;
use slog::{debug, info, Logger};

use crate::lib::codegen::components::common::{ComponentManifest, GeneratedCodes};
use crate::lib::codegen::templates::{
    accessors_cargo_toml, bindings_cargo_toml, canister_project_cargo_toml, logic_cargo_toml,
    root_cargo_toml,
};
use crate::lib::utils::env::cache_envfile;
use crate::lib::utils::{find_duplicates, paths, DOTENV_FILENAME};
use crate::lib::{
    codegen::project::ProjectManifestData,
    environment::EnvironmentImpl,
    utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME},
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
    #[arg(long, short = 'p')]
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

    // load env
    let env_file_path = format!("{}/{}", &project_path_str, DOTENV_FILENAME);
    if Path::new(&env_file_path).is_file() {
        info!(log, r#"Load env file: "{}""#, &env_file_path);
        cache_envfile(Some(&env_file_path))?;
    }

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

    let component_data = project_manifest.load_component_manifests(project_path_str.as_str())?;

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
    fs::create_dir_all(src_path_str).expect("failed to create dir: src");

    // remove generated files
    // NOTE: not remove `logics` to remain user's logic
    let _ = fs::remove_dir_all(format!("{}/__interfaces", src_path_str));
    let _ = fs::remove_dir_all(format!("{}/bindings", src_path_str));
    let _ = fs::remove_dir_all(format!("{}/canisters", src_path_str));
    let _ = fs::remove_dir_all(format!("{}/accessors", src_path_str));

    // generate /artifacts/__interfaces
    let interfaces_path_str = format!("{}/__interfaces", src_path_str);
    fs::create_dir(&interfaces_path_str)?;
    let dummy_candid_file_path = format!("{}/interfaces/{}", project_path_str, "sample.did");
    if !Path::new(&dummy_candid_file_path).is_file() {
        fs::write(&dummy_candid_file_path, dummy_candid_blob())?;
    }

    // generate canister projects
    let mut generated_component_ids = vec![];
    let mut is_exist_accessors_folder = false;
    let mut is_exist_bindings_folder = false;
    for data in component_data {
        let id = data.id().unwrap();

        if let Err(msg) = data.validate_manifest() {
            bail!(format!(r#"[{}] Invalid manifest: {}"#, id, msg));
        }
        info!(log, r#"[{}] Start processing..."#, id);

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
                    r#"[{}] Interface file '{}' copied from user's interface"#, id, &interface_file
                );
                let abi_file = File::open(user_if_file_path)?;
                interface_contract = Some(ethabi::Contract::load(abi_file)?);
            } else if let Some(contents) = builtin_interface(&interface_file) {
                fs::write(dst_interface_path, contents)?;
                info!(
                    log,
                    r#"[{}] Interface file '{}' copied from builtin interface"#,
                    id,
                    &interface_file
                );
                let contract: ethabi::Contract = serde_json::from_str(contents)?;
                interface_contract = Some(contract);
            } else {
                bail!(format!(
                    r#"[{}] Interface file "{}" not found"#,
                    id, &interface_file
                ));
            }
        }

        // copy and move oracle interface
        if data.destination_type().is_some() {
            let json_name = "Oracle.json";
            let json_contents = include_str!("../../resources/Oracle.json");
            fs::write(
                format!("{}/{}", &interfaces_path_str, &json_name),
                json_contents,
            )?;
            info!(
                log,
                r#"[{}] Interface file '{}' copied from builtin interface"#, id, &json_name
            );
        }

        // Generate /bindings/(component)
        let bindings = data.generate_bindings()?;
        if !bindings.is_empty() {
            // generate dummy bindings to be able to run cargo test
            let bindings_path_str = &paths::bindings_path_str(src_path_str, &id);
            create_cargo_project(
                bindings_path_str,
                Some(&bindings_cargo_toml(&id)),
                Some(CargoProjectSrc::new_with_mods(bindings.clone())),
            )
            .map_err(|err| {
                anyhow::anyhow!(r#"[{}] Failed to create bindings project by: {}"#, id, err)
            })?;

            is_exist_bindings_folder = true;
        }

        // Generate /logics/(component)
        let logic_path_str = &paths::logics_path_str(src_path_str, &id);
        if Path::new(logic_path_str).is_dir() {
            info!(
                log,
                r#"[{}] Skip creating logic project: '{}' already exists"#, id, logic_path_str,
            );
        } else {
            let codes = data.generate_user_impl_template();
            let src = match codes {
                anyhow::Result::Ok(codes) => {
                    Some(CargoProjectSrc::new_with_mods(BTreeMap::from([
                        ("lib".to_string(), codes.lib),
                        (
                            "types".to_string(),
                            codes.types.map(|t| t.to_string()).unwrap_or_default(),
                        ),
                    ])))
                }
                anyhow::Result::Err(_) => None,
            };
            create_cargo_project(
                logic_path_str,
                Option::Some(&logic_cargo_toml(
                    &id,
                    !bindings.is_empty(),
                    data.dependencies(),
                )),
                src,
            )
            .map_err(|err| {
                anyhow::anyhow!(r#"[{}] Failed to create logic project by: {}"#, id, err)
            })?;

            let _ = Command::new("cargo")
                .current_dir(logic_path_str)
                .args(["fmt"])
                .output();
        }

        // Generate /accessors/(component)
        if !data.dependencies().is_empty() {
            let accessors_path_str = &paths::accessors_path_str(src_path_str, &id);
            let codes = data.generate_dependency_accessors();
            let src = match codes {
                anyhow::Result::Ok(codes) => {
                    Some(CargoProjectSrc::new_with_mods(BTreeMap::from([
                        ("lib".to_string(), codes.lib),
                        (
                            "types".to_string(),
                            codes.types.map(|t| t.to_string()).unwrap_or_default(),
                        ),
                    ])))
                }
                anyhow::Result::Err(_) => None,
            };
            create_cargo_project(
                accessors_path_str,
                Some(&accessors_cargo_toml(&id, vec![id.to_string()])),
                src,
            )
            .map_err(|err| {
                anyhow::anyhow!(
                    r#"[{}] Failed to create logic dependency accessors by: {}"#,
                    id,
                    err
                )
            })?;
            is_exist_accessors_folder = true;
        }

        // Generate Cargo.toml that configure the entire workspace
        generated_component_ids.push(id.to_string());
        fs::write(
            format!("{}/Cargo.toml", src_path_str),
            root_cargo_toml(
                generated_component_ids.clone(),
                is_exist_bindings_folder,
                is_exist_accessors_folder,
            ),
        )?;
        // Generate /canisters/(component)
        let canister_path_str = &paths::canisters_path_str(src_path_str, &id);
        let GeneratedCodes { lib, types } =
            data.generate_codes(interface_contract).map_err(|err| {
                anyhow::anyhow!(r#"[{}] Failed to generate canister code by: {}"#, id, err)
            })?;
        let src = if let Some(types) = types {
            CargoProjectSrc::new_with_mods(BTreeMap::from([
                ("lib".to_string(), lib),
                ("types".to_string(), types.to_string()),
            ]))
        } else {
            CargoProjectSrc::new(lib)
        };
        create_cargo_project(
            canister_path_str,
            Some(&canister_project_cargo_toml(&id, !bindings.is_empty())),
            Some(src),
        )
        .map_err(|err| {
            anyhow::anyhow!(r#"[{}] Failed to create canister project by: {}"#, id, err)
        })?;

        // generate canister's .did files
        let action = "Generate interfaces (.did files)";
        info!(log, r#"[{}] {} ..."#, id, action);
        let output = Command::new("cargo")
            .current_dir(canister_path_str)
            .args(["test"])
            .output()
            .expect("failed to execute: cargo test");
        if output.status.success() {
            debug!(
                log,
                "{}",
                std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
            );
            info!(log, r#"[{}] Succeeded: {}"#, id, action);
        } else {
            bail!(format!(
                r#"[{}] Failed: {} by: {}"#,
                id,
                action,
                std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stdout")
            ));
        }
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

struct CargoProjectSrc(BTreeMap<String, String>);
impl CargoProjectSrc {
    fn new(src: String) -> Self {
        Self(BTreeMap::from([("lib".to_string(), src)]))
    }

    fn new_with_mods(mods: BTreeMap<String, String>) -> Self {
        Self(mods)
    }
}

fn create_cargo_project(
    path_str: &str,
    manifest: Option<&str>,
    src: Option<CargoProjectSrc>,
) -> anyhow::Result<()> {
    fs::create_dir_all(Path::new(&format!("{}/src", path_str)))?;
    fs::write(
        Path::new(&format!("{}/Cargo.toml", path_str)),
        manifest.unwrap_or_default(),
    )?;
    if let Some(CargoProjectSrc(modules)) = src {
        for (module_name, codes) in modules.iter() {
            fs::write(
                Path::new(&format!("{}/src/{}.rs", path_str, module_name)),
                codes,
            )?;
        }
    } else {
        File::create(Path::new(&format!("{}/src/lib.rs", path_str)))?;
    }

    Ok(())
}
