use std::fs::File;
use std::io::Write;
use std::{path::Path, fs};
use std::fmt::Debug;

use anyhow::{Ok, bail};
use clap::Parser;
use slog::{info, error};

use crate::{lib::{environment::EnvironmentImpl, utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME}, codegen::{project::ProjectManifestData, components::{get_type_from_manifest, SnapshotComponentManifest, RelayerComponentManifest, ComponentManifest}}}, types::ComponentType};

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

    let project_path = project_path.unwrap_or(".".to_string());
    let artifacts_path = Path::new(&project_path).join("artifacts");
    if artifacts_path.exists() {
        fs::remove_dir_all(&artifacts_path)?;
    }
    fs::create_dir(&artifacts_path)?;

    let project_manifest = ProjectManifestData::load(&format!("{}/{}", project_path, PROJECT_MANIFEST_FILENAME))?;
    // TODO: need validations
    for component in project_manifest.components {
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", project_path, relative_component_path);
        let component_type = get_type_from_manifest(&component_path)?;

        let (label, data): (String, Box<dyn ComponentManifest>) = match component_type {
            ComponentType::Snapshot => {
                let manifest = SnapshotComponentManifest::load(&component_path).unwrap();
                (
                    manifest.clone().label,
                    Box::new(manifest),
                )
            },
            ComponentType::Relayer => {
                let manifest = RelayerComponentManifest::load(&component_path).unwrap();
                (
                    manifest.clone().label,
                    Box::new(manifest),
                )
            },
        };

        let code_path = format!("{}/artifacts/{}.rs", project_path, label);
        let mut code_file = File::create(&code_path)?;
        code_file.write_all(data.generate_codes()?.to_string().as_bytes())?;
    }

    info!(
        log,
        r#"Project "{}" builded successfully"#,
        project_manifest.label
    );

    Ok(())
}