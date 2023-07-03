use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use anyhow::bail;
use clap::Parser;
use slog::{error, info};

use crate::{
    lib::{
        codegen::{
            components::{
                common::{ComponentManifest, Datasource},
                event_indexer::{
                    EventIndexerComponentManifest, EventIndexerDatasource,
                    EventIndexerEventDefinition,
                },
                relayer::{DestinationField, RelayerComponentManifest},
                snapshot::{SnapshotComponentManifest, SnapshotStorage},
            },
            project::{ProjectManifestComponentField, ProjectManifestData},
        },
        environment::EnvironmentImpl,
        utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME, PROJECT_MANIFEST_VERSION},
    },
    types::ComponentType,
};

#[derive(Debug, Parser)]
#[command(name = "create")]
/// Generates component manifest of specified type and adds to your project.
pub struct CreateOpts {
    #[arg(required = true)]
    component_name: String,
    #[arg(long)]
    type_: ComponentType,
    #[arg(long)]
    path: Option<String>,
}

const GLOBAL_ERROR_MSG: &str = "Fail 'Create' command";

pub fn exec(env: &EnvironmentImpl, opts: CreateOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let component_name = opts.component_name;
    let component_type = opts.type_;
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        error!(log, r#"{}"#, msg);
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    info!(log, r#"Creating new component "{}"..."#, component_name);

    let codes = match component_type {
        ComponentType::EventIndexer => {
            template_event_indexer_manifest(&component_name).to_str_as_yaml()
        }
        ComponentType::Snapshot => template_snapshot_manifest(&component_name).to_str_as_yaml(),
        ComponentType::Relayer => template_relayer_manifest(&component_name).to_str_as_yaml(),
    }?;
    let relative_component_path = format!("components/{}.yaml", component_name);
    let (component_file_path, project_file_path) = if let Some(project_name) = project_path {
        (
            format!("{}/{}", project_name, relative_component_path),
            format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
        )
    } else {
        (
            relative_component_path.clone(),
            PROJECT_MANIFEST_FILENAME.to_string(),
        )
    };

    // write to .yaml
    fs::write(component_file_path, codes)?;
    // update to project.yaml
    let mut data = ProjectManifestData::load(&project_file_path)?;
    data.add_components(&[ProjectManifestComponentField::new(
        &relative_component_path,
        None,
    )])?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&project_file_path)?;
    file.write_all(data.to_str_as_yaml()?.as_bytes())?;

    info!(
        log,
        r#"{:?} component "{}" created successfully"#, component_type, component_name
    );

    Ok(())
}

fn template_event_indexer_manifest(component_name: &str) -> EventIndexerComponentManifest {
    EventIndexerComponentManifest::new(
        component_name,
        PROJECT_MANIFEST_VERSION,
        EventIndexerDatasource::new(
            "0000000000000000000000000000000000000000".to_string(),
            EventIndexerEventDefinition::new("EventIdentifier".to_string(), None),
        ),
        3600,
    )
}

fn template_snapshot_manifest(component_name: &str) -> SnapshotComponentManifest {
    SnapshotComponentManifest::new(
        component_name,
        PROJECT_MANIFEST_VERSION,
        Datasource::new_contract("functionIdentifier()".to_string(), None, None),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_relayer_manifest(component_name: &str) -> RelayerComponentManifest {
    RelayerComponentManifest::new(
        component_name,
        PROJECT_MANIFEST_VERSION,
        Datasource::new_canister("function_identifier()".to_string(), None, None),
        DestinationField::default(),
        3600,
    )
}
