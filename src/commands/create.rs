use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use anyhow::bail;
use clap::Parser;
use slog::info;

use crate::{
    lib::{
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
                event_indexer::{
                    EventIndexerComponentManifest, EventIndexerDatasource,
                    EventIndexerEventDefinition, SourceNetwork,
                },
                relayer::{DestinationField, RelayerComponentManifest},
                snapshot_indexer::{SnapshotIndexerComponentManifest, SnapshotStorage},
                snapshot_indexer_https::{
                    SnapshotIndexerHTTPSComponentManifest, SnapshotIndexerHTTPSDataSource,
                },
            },
            project::{ProjectManifestComponentField, ProjectManifestData},
        },
        environment::EnvironmentImpl,
        utils::{
            find_duplicates, is_chainsight_project, PROJECT_MANIFEST_FILENAME,
            PROJECT_MANIFEST_VERSION,
        },
    },
    types::ComponentType,
};

#[derive(Debug, Parser)]
#[command(name = "create")]
/// Generates component manifest of specified type and adds to your project.
pub struct CreateOpts {
    /// Specifies the name of the component to create.
    #[arg(required = true)]
    component_name: String,

    /// Specifies type of the component to create.
    #[arg(long)]
    type_: ComponentType,

    /// Specify the path of the project to which the component is to be added.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    path: Option<String>,
}

pub fn exec(env: &EnvironmentImpl, opts: CreateOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let component_name = opts.component_name;
    let component_type = opts.type_;
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        bail!(format!(r#"{}"#, msg));
    }

    info!(
        log,
        r#"Start creating new component '{}'..."#, component_name
    );

    let codes = match component_type {
        ComponentType::EventIndexer => {
            template_event_indexer_manifest(&component_name).to_str_as_yaml()
        }
        ComponentType::AlgorithmIndexer => {
            template_algorithm_indexer_manifest(&component_name).to_str_as_yaml()
        }
        ComponentType::SnapshotIndexer => {
            template_snapshot_manifest(&component_name).to_str_as_yaml()
        }
        ComponentType::Relayer => template_relayer_manifest(&component_name).to_str_as_yaml(),
        ComponentType::AlgorithmLens => {
            template_algorithm_lens_manifest(&component_name).to_str_as_yaml()
        }
        ComponentType::SnapshotIndexerHTTPS => {
            template_snapshot_web2_manifest(&component_name).to_str_as_yaml()
        }
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

    // update project manifest
    let mut data = ProjectManifestData::load(&project_file_path)?;
    data.add_components(&[ProjectManifestComponentField::new(
        &relative_component_path,
        None,
    )])?;
    //// check whether manifests of the same path exist or not
    {
        let component_paths = data
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
    //// update to .yaml
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&project_file_path)?;
    file.write_all(data.to_str_as_yaml()?.as_bytes())?;

    // write to .yaml
    fs::write(component_file_path, codes)?;

    info!(
        log,
        r#"{:?} component '{}' created successfully"#, component_type, component_name
    );

    Ok(())
}

fn template_event_indexer_manifest(component_name: &str) -> EventIndexerComponentManifest {
    EventIndexerComponentManifest::new(
        component_name,
        "",
        PROJECT_MANIFEST_VERSION,
        EventIndexerDatasource::new(
            "0000000000000000000000000000000000000000".to_string(),
            EventIndexerEventDefinition::new("EventIdentifier".to_string(), None),
            SourceNetwork {
                chain_id: 80001,
                rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/<YOUR_KEY>".to_string(),
            },
            37730337,
            Some("ERC-20".to_string()),
        ),
        3600,
    )
}

fn template_algorithm_indexer_manifest(component_name: &str) -> AlgorithmIndexerComponentManifest {
    AlgorithmIndexerComponentManifest::new(
        component_name,
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmIndexerDatasource::default(),
        vec![AlgorithmIndexerOutput::default()],
        3600,
    )
}

fn template_snapshot_manifest(component_name: &str) -> SnapshotIndexerComponentManifest {
    SnapshotIndexerComponentManifest::new(
        component_name,
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::new_contract("functionIdentifier()".to_string(), None, None),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_relayer_manifest(component_name: &str) -> RelayerComponentManifest {
    RelayerComponentManifest::new(
        component_name,
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::new_canister("function_identifier()".to_string(), None, None),
        DestinationField::default(),
        3600,
    )
}

fn template_algorithm_lens_manifest(component_name: &str) -> AlgorithmLensComponentManifest {
    AlgorithmLensComponentManifest::new(
        component_name,
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmLensDataSource::default(),
        AlgorithmLensOutput::default(),
    )
}
fn template_snapshot_web2_manifest(component_name: &str) -> SnapshotIndexerHTTPSComponentManifest {
    SnapshotIndexerHTTPSComponentManifest::new(
        component_name,
        "",
        PROJECT_MANIFEST_VERSION,
        SnapshotIndexerHTTPSDataSource::default(),
        SnapshotStorage::default(),
        3600,
    )
}

#[cfg(test)]
mod tests {
    use std::{fs::create_dir_all, panic, path::Path};

    use crate::{
        commands::test::runner::run_with,
        lib::{logger::create_root_logger, utils::CHAINSIGHT_FILENAME},
    };
    const PROJECT_DIR: &str = "test-create-project";

    use super::*;

    fn test(test: impl FnOnce() -> () + panic::UnwindSafe) {
        run_with(
            || setup(),
            || {
                test();
            },
            || teardown(),
        );
    }

    #[test]
    fn test_exec() {
        test(|| {
            let log = create_root_logger(1);
            let env = EnvironmentImpl::new().with_logger(log);
            let opts = CreateOpts {
                component_name: "test".to_string(),
                type_: ComponentType::EventIndexer,
                path: Some(PROJECT_DIR.to_string()),
            };
            let result = exec(&env, opts);
            if let Err(e) = result {
                println!("{:?}", e);
                assert!(false);
            }
        });
    }
    fn setup() {
        let project_dir = Path::new(PROJECT_DIR);
        create_dir_all(project_dir.join("components")).unwrap();
        fs::write(project_dir.join(CHAINSIGHT_FILENAME), "").unwrap();
    }
    fn teardown() {
        let project_dir = Path::new(PROJECT_DIR);
        fs::remove_dir_all(project_dir).unwrap();
    }
}
