use std::fmt::Debug;
use std::fs;
use std::process::Command;

use anyhow::{bail, Ok};
use clap::Parser;
use slog::{debug, info, Logger};

use crate::commands::generate;
use crate::lib::codegen::components::algorithm_indexer::AlgorithmIndexerComponentManifest;
use crate::lib::codegen::components::algorithm_lens::AlgorithmLensComponentManifest;
use crate::lib::codegen::components::common::{ComponentManifest, ComponentTypeInManifest};
use crate::lib::codegen::components::event_indexer::EventIndexerComponentManifest;
use crate::lib::codegen::components::relayer::RelayerComponentManifest;
use crate::lib::codegen::components::snapshot_indexer::SnapshotIndexerComponentManifest;
use crate::lib::codegen::components::snapshot_indexer_https::SnapshotIndexerHTTPSComponentManifest;
use crate::lib::codegen::templates::dfx_json;
use crate::lib::utils::{find_duplicates, paths, ARTIFACTS_DIR};
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
    /// Specify the path of the project to build.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    path: Option<String>,

    /// Only perform build.
    /// Perform this steps with code already generated.
    #[arg(long)]
    only_build: bool,
}

pub fn exec(env: &EnvironmentImpl, opts: BuildOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path.clone();

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
        r#"Start building project '{}'"#, project_manifest.label
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

    if opts.only_build {
        info!(log, r#"Skip codegen"#);
    } else {
        generate::exec(env, generate::GenerateOpts::new(opts.path.clone()))?;
        info!(log, r#"Start building..."#);
    }

    // build codes generated
    execute_codebuild(log, &project_path_str, &component_data)?;

    info!(
        log,
        r#"Project '{}' built successfully"#, project_manifest.label
    );

    Ok(())
}

fn execute_codebuild(
    log: &Logger,
    project_path_str: &str,
    component_data: &Vec<Box<dyn ComponentManifest>>,
) -> anyhow::Result<()> {
    let src_path_str = &paths::src_path_str(project_path_str);
    // Regenerate output dir
    let output_path_str = &format!("{}/{}", project_path_str, ARTIFACTS_DIR);
    let _ = fs::remove_dir_all(output_path_str);
    fs::create_dir_all(output_path_str)?;

    let projects = component_data
        .iter()
        .map(|data| data.metadata().label.to_string())
        .collect::<Vec<String>>();
    fs::write(format!("{}/dfx.json", output_path_str), dfx_json(projects))?;

    // Copy .did to output dir
    for component_datum in component_data {
        let label = &component_datum.metadata().label.clone();
        let did_src_path = format!(
            "{}/{}.did",
            paths::canisters_path_str(src_path_str, label),
            label
        );
        let did_dst_path = format!("{}/{}.did", output_path_str, label);
        fs::copy(did_src_path, did_dst_path)?;
    }

    let action = "Compile canisters";
    info!(log, "{}...", action);
    let output = Command::new("cargo")
        .current_dir(src_path_str)
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
            std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
        );
        info!(log, "Succeeded: {}", action);
    } else {
        bail!(format!(
            "Failed: {} by: {}",
            action,
            std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stdout")
        ));
    }

    let action = "Shrink/Optimize module";
    info!(log, "{}s...", action);
    for component_datum in component_data {
        let label = &component_datum.metadata().label.clone();
        let wasm_path = format!(
            "{}/target/wasm32-unknown-unknown/release/{}.wasm",
            src_path_str,
            paths::canister_name(label)
        );
        let output_path = format!("{}/{}.wasm", output_path_str, label);
        let output = Command::new("ic-wasm")
            .current_dir(project_path_str)
            .args([&wasm_path, "-o", &output_path, "shrink"])
            .output()
            .expect("failed to execute process: ic_wasm shrink");
        if output.status.success() {
            debug!(
                log,
                "[{}] {}",
                label,
                std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
            );
            debug!(log, "[{}] Succeeded: {}", label, action);
        } else {
            bail!(format!(
                "[{}] Failed: '{}' by: {}",
                label,
                action,
                std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stdout")
            ));
        }
    }
    info!(log, "Succeeded: {}s", action);

    let action = "Add metadata to modules";
    info!(log, "{}...", action);
    for component_datum in component_data {
        add_metadata_to_wasm(log, project_path_str, component_datum.as_ref())?;
    }
    info!(log, "Succeeded: {}", action);

    anyhow::Ok(())
}

fn add_meta(
    label: &str,
    key: &str,
    value: &str,
    wasm_path: &str,
    project_path_str: &str,
    log: &Logger,
) -> anyhow::Result<()> {
    let action = format!("Add metadata {}", key);
    let output = Command::new("ic-wasm")
        .current_dir(project_path_str)
        .args([
            &wasm_path, "-o", &wasm_path, "metadata", key, "-d", value, "-v", "public",
        ])
        .output()
        .unwrap_or_else(|_| panic!("failed to execute process: ic-wasm metadata {}", key));
    if output.status.success() {
        debug!(
            log,
            "[{}] {}",
            label,
            std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout")
        );
        debug!(log, "[{}] Succeeded: '{}'", label, action);
    } else {
        bail!(format!(
            "[{}] Failed: '{}' by: {}",
            label,
            action,
            std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stdout")
        ));
    };
    anyhow::Ok(())
}

fn add_metadata_to_wasm(
    log: &Logger,
    project_path_str: &str,
    component_datum: &dyn ComponentManifest,
) -> anyhow::Result<()> {
    let label = &component_datum.metadata().label.clone();
    let wasm_path = &format!("{}/{}.wasm", ARTIFACTS_DIR, label);
    let put_meta = |key: &str, value: &str| -> anyhow::Result<()> {
        add_meta(label, key, value, wasm_path, &project_path_str, log)
    };

    put_meta("chainsight:label", label)?;
    put_meta(
        "chainsight:component_type",
        &component_datum.component_type().to_string(),
    )?;
    put_meta(
        "chainsight:description",
        &component_datum.metadata().description,
    )?;
    let tags = component_datum.metadata().tags.clone().unwrap_or(vec![]);
    let tags_str = serde_json::to_string(&tags)?;

    put_meta("chainsight:tags", tags_str.as_str())?;
    let meta_json = serde_json::to_string(&vec![component_datum.get_sources()])?;
    put_meta("chainsight:sources", meta_json.as_str())?;
    for (key, value) in component_datum.custom_tags().iter() {
        put_meta(key, value)?;
    }
    anyhow::Ok(())
}
