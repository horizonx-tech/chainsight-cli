use std::{fs, path::Path};

use anyhow::bail;
use clap::Parser;
use slog::info;

use crate::lib::{
    codegen::{
        components::{
            algorithm_indexer::{
                AlgorithmIndexerComponentManifest, AlgorithmIndexerDatasource,
                AlgorithmIndexerOutput,
            },
            algorithm_lens::{
                AlgorithmLensComponentManifest, AlgorithmLensDataSource, AlgorithmLensOutput,
            },
            common::{ComponentManifest, Datasource},
            event_indexer::{EventIndexerComponentManifest, EventIndexerDatasource},
            relayer::{DestinationField, RelayerComponentManifest},
            snapshot::{SnapshotComponentManifest, SnapshotStorage},
            snapshot_json_rpc::{SnapshotJsonRPCComponentManifest, SnapshotJsonRPCDataSource},
        },
        project::{ProjectManifestComponentField, ProjectManifestData},
    },
    environment::EnvironmentImpl,
    utils::{CHAINSIGHT_FILENAME, PROJECT_MANIFEST_FILENAME, PROJECT_MANIFEST_VERSION},
};

#[derive(Debug, Parser)]
#[command(name = "new")]
/// Generates Chainsight project with built-in templates.
pub struct NewOpts {
    /// Specifies the name of the project to create.
    #[arg(required = true)]
    project_name: String,
}

pub fn exec(env: &EnvironmentImpl, opts: NewOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_name = opts.project_name;
    let project_name_path = Path::new(&project_name);
    if project_name_path.exists() {
        bail!(format!(r#"Project "{}" already exists"#, project_name));
    }
    info!(log, r#"Creating new project "{}"..."#, project_name);
    let res = create_project(&project_name);
    match res {
        Ok(_) => {
            info!(log, r#"Project "{}" created successfully"#, project_name);
            Ok(())
        }
        Err(err) => {
            bail!(format!(
                r#"Fail to create project "{}": {}"#,
                project_name, err
            ));
        }
    }
}

fn create_project(project_name: &str) -> anyhow::Result<()> {
    // Create directories
    fs::create_dir_all(format!("{}/components", project_name))?;
    fs::create_dir_all(format!("{}/interfaces", project_name))?;

    // Create files
    fs::write(format!("{}/{}", project_name, CHAINSIGHT_FILENAME), "")?;
    let relative_event_indexer_path = format!("components/{}_event_indexer.yaml", project_name);
    let relative_algorithm_indexer_path =
        format!("components/{}_algorithm_indexer.yaml", project_name);
    let relative_snapshot_chain_path = format!("components/{}_snapshot_chain.yaml", project_name);
    let relative_snapshot_icp_path = format!("components/{}_snapshot_icp.yaml", project_name);
    let relative_relayer_path = format!("components/{}_relayer.yaml", project_name);
    let relative_algorithmlens_path = format!("components/{}_algorithm_lens.yaml", project_name);
    let relative_snapshot_json_rpc_path =
        format!("components/{}_snapshot_json_rpc.yaml", project_name);
    fs::write(
        format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
        ProjectManifestData::new(
            project_name,
            PROJECT_MANIFEST_VERSION,
            &[
                ProjectManifestComponentField::new(&relative_event_indexer_path, None),
                ProjectManifestComponentField::new(&relative_algorithm_indexer_path, None),
                ProjectManifestComponentField::new(&relative_snapshot_chain_path, None),
                ProjectManifestComponentField::new(&relative_snapshot_icp_path, None),
                ProjectManifestComponentField::new(&relative_relayer_path, None),
                ProjectManifestComponentField::new(&relative_algorithmlens_path, None),
                ProjectManifestComponentField::new(&relative_snapshot_json_rpc_path, None),
            ],
        )
        .to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_event_indexer_path),
        template_event_indexer_manifest(project_name).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_algorithm_indexer_path),
        template_algorithm_indexer_manifest(project_name).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_snapshot_chain_path),
        template_snapshot_chain_manifest(project_name).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_snapshot_icp_path),
        template_snapshot_icp_manifest(project_name).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_relayer_path),
        template_relayer_manifest(project_name).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_algorithmlens_path),
        tempalte_algorithm_lens_manifest(project_name).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_snapshot_json_rpc_path),
        template_snapshot_json_rpc_manifest(project_name).to_str_as_yaml()?,
    )?;

    Ok(())
}

fn template_event_indexer_manifest(project_name: &str) -> EventIndexerComponentManifest {
    EventIndexerComponentManifest::new(
        &format!("{}_event_indexer", project_name),
        "",
        PROJECT_MANIFEST_VERSION,
        EventIndexerDatasource::default(),
        3600,
    )
}

fn template_algorithm_indexer_manifest(project_name: &str) -> AlgorithmIndexerComponentManifest {
    AlgorithmIndexerComponentManifest::new(
        &format!("{}_algorithm_indexer", project_name),
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmIndexerDatasource::default(),
        vec![AlgorithmIndexerOutput::default()],
        3600,
    )
}

fn template_snapshot_chain_manifest(project_name: &str) -> SnapshotComponentManifest {
    SnapshotComponentManifest::new(
        &format!("{}_snapshot_chain", project_name),
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::default_contract(),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_snapshot_icp_manifest(project_name: &str) -> SnapshotComponentManifest {
    SnapshotComponentManifest::new(
        &format!("{}_snapshot_icp", project_name),
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::default_canister(true),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_snapshot_json_rpc_manifest(project_name: &str) -> SnapshotJsonRPCComponentManifest {
    SnapshotJsonRPCComponentManifest::new(
        &format!("{}_snapshot_json_rpc", project_name),
        "",
        PROJECT_MANIFEST_VERSION,
        SnapshotJsonRPCDataSource::default(),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_relayer_manifest(project_name: &str) -> RelayerComponentManifest {
    RelayerComponentManifest::new(
        &format!("{}_relayer", project_name),
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::default_canister(false),
        DestinationField::default(),
        3600,
    )
}
fn tempalte_algorithm_lens_manifest(project_name: &str) -> AlgorithmLensComponentManifest {
    AlgorithmLensComponentManifest::new(
        &format!("{}_algorithm_lens", project_name),
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmLensDataSource::default(),
        AlgorithmLensOutput::default(),
    )
}
