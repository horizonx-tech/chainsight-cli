use std::{path::Path, fs};

use anyhow::bail;
use clap::Parser;
use slog::{info, error};

use crate::{
    lib::{environment::EnvironmentImpl, codegen::{project::{ProjectManifestData, ProjectManifestComponentField}, components::{snapshot::{SnapshotComponentManifest, SnapshotStorage}, common::{Datasource, ComponentManifest}, relayer::{RelayerComponentManifest, DestinationField}, event_indexer::{EventIndexerComponentManifest, EventIndexerDatasource}}}, utils::{CHAINSIGHT_FILENAME, PROJECT_MANIFEST_FILENAME, PROJECT_MANIFEST_VERSION}}
};

#[derive(Debug, Parser)]
#[command(name = "new")]
/// Generate ChainSight project by prepared template
pub struct NewOpts {
    #[arg(required = true)]
    project_name: String,
}

const GLOBAL_ERROR_MSG: &str = "Fail 'New' command";

pub fn exec(env: &EnvironmentImpl, opts: NewOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_name = opts.project_name;
    let project_name_path = Path::new(&project_name);
    if project_name_path.exists() {
        error!(
            log,
            r#"Project "{}" already exists"#,
            project_name
        );
        bail!(GLOBAL_ERROR_MSG.to_string())
    }
    info!(
        log,
        r#"Creating new project "{}"..."#,
        project_name
    );
    let res = create_project(&project_name);
    match res {
        Ok(_) => {
            info!(
                log,
                r#"Project "{}" created successfully"#,
                project_name
            );
            Ok(())
        },
        Err(err) => {
            error!(
                log,
                r#"Fail to create project "{}": {}"#,
                project_name,
                err
            );
            bail!(GLOBAL_ERROR_MSG.to_string())
        }
    }
}

fn create_project(project_name: &str) -> anyhow::Result<()> {
    // Create directories
    fs::create_dir_all(format!("{}/components", project_name))?;
    fs::create_dir_all(format!("{}/interfaces", project_name))?;

    // Create files
    fs::write(format!("{}/{}", project_name, CHAINSIGHT_FILENAME), "")?;
    let relative_snapshot_chain_path = format!("components/{}_snapshot_chain.yaml", project_name);
    let relative_snapshot_icp_path = format!("components/{}_snapshot_icp.yaml", project_name);
    let relative_relayer_path = format!("components/{}_relayer.yaml", project_name);

    fs::write(
        format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
        ProjectManifestData::new(
            project_name,
            PROJECT_MANIFEST_VERSION,
            &vec![
                ProjectManifestComponentField::new(&relative_snapshot_chain_path, None),
                ProjectManifestComponentField::new(&relative_snapshot_icp_path, None),
                ProjectManifestComponentField::new(&relative_relayer_path, None),
            ]
        ).to_str_as_yaml()?,
    )?;

    fs::write(
        format!("{}/{}", project_name, relative_snapshot_chain_path),
        template_snapshot_chain_manifest(project_name).to_str_as_yaml()?
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_snapshot_icp_path),
        template_snapshot_icp_manifest(project_name).to_str_as_yaml()?
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_relayer_path),
        template_relayer_manifest(project_name).to_str_as_yaml()?
    )?;

    Ok(())
}

fn template_event_indexer_manifest(project_name: &str) -> EventIndexerComponentManifest {
    EventIndexerComponentManifest::new(
        &format!("{}_event_indexer", project_name),
        PROJECT_MANIFEST_VERSION,
        EventIndexerDatasource::default(),
        3600,
    )
}

fn template_snapshot_chain_manifest(project_name: &str) -> SnapshotComponentManifest {
    SnapshotComponentManifest::new(
        &format!("{}_snapshot_chain", project_name),
        PROJECT_MANIFEST_VERSION,
        Datasource::default_contract(),
        SnapshotStorage::default(),
        3600
    )
}

fn template_snapshot_icp_manifest(project_name: &str) -> SnapshotComponentManifest {
    SnapshotComponentManifest::new(
        &format!("{}_snapshot_icp", project_name),
        PROJECT_MANIFEST_VERSION,
        Datasource::default_canister(true),
        SnapshotStorage::default(),
        3600
    )
}

fn template_relayer_manifest(project_name: &str) -> RelayerComponentManifest {
    RelayerComponentManifest::new(
        &format!("{}_relayer", project_name),
        PROJECT_MANIFEST_VERSION,
        Datasource::default_canister(false),
        DestinationField::default(),
    )
}
