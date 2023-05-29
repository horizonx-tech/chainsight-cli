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
    for component in project_manifest.components {
        let relative_component_path = component.component_path;
        let component_path = format!("{}/{}", project_path, relative_component_path);
        let component_type = get_type_from_manifest(&component_path)?;

        let data: Box<dyn ComponentManifest> = match component_type {
            ComponentType::Snapshot => Box::new(SnapshotComponentManifest::load(&component_path).unwrap()),
            ComponentType::Relayer => Box::new(RelayerComponentManifest::load(&component_path).unwrap()),
        };
        println!("{:?}", data); // TODO: generate codes
    }

    Ok(())
}