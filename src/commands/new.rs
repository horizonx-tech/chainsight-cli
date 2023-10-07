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
            algorithm_lens::{AlgorithmLensComponentManifest, AlgorithmLensDataSource},
            common::{ComponentManifest, Datasource, SnapshotStorage},
            event_indexer::{EventIndexerComponentManifest, EventIndexerDatasource},
            relayer::{DestinationField, RelayerComponentManifest},
            snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
            snapshot_indexer_https::{
                SnapshotIndexerHTTPSComponentManifest, SnapshotIndexerHTTPSDataSource,
            },
            snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
        },
        project::{ProjectManifestComponentField, ProjectManifestData},
        templates::gitignore,
    },
    environment::EnvironmentImpl,
    utils::{
        CHAINSIGHT_FILENAME, GITIGNORE_FILENAME, PROJECT_MANIFEST_FILENAME,
        PROJECT_MANIFEST_VERSION,
    },
};

#[derive(Debug, Parser)]
#[command(name = "new")]
/// Generates Chainsight project with built-in templates.
pub struct NewOpts {
    /// Specifies the name of the project to create.
    #[arg(required = true)]
    pub project_name: String,

    /// Skip generation of sample component manifests.
    #[arg(long, visible_short_alias = 'n')]
    pub no_samples: bool,
}

pub fn exec(env: &EnvironmentImpl, opts: NewOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_name = opts.project_name;
    let project_name_path = Path::new(&project_name);
    if project_name_path.exists() {
        bail!(format!(r#"Project '{}' already exists"#, project_name));
    }
    info!(log, r#"Start creating new project '{}'..."#, project_name);
    let res = create_project(&project_name, opts.no_samples);
    match res {
        Ok(_) => {
            info!(log, r#"Project '{}' created successfully"#, project_name);

            if opts.no_samples {
                info!(
                    log,
                    "You can add components with:\n\n  cd {} && csx add\n", project_name
                );
            }
            Ok(())
        }
        Err(err) => {
            bail!(format!(
                r#"Failed: Create project '{}' by: {}"#,
                project_name, err
            ));
        }
    }
}

fn create_project(project_name: &str, no_samples: bool) -> anyhow::Result<()> {
    // Create directories
    fs::create_dir_all(format!("{}/components", project_name))?;
    fs::create_dir_all(format!("{}/interfaces", project_name))?;

    // Create files
    fs::write(
        format!("{}/{}", project_name, GITIGNORE_FILENAME),
        gitignore(),
    )?;
    fs::write(format!("{}/{}", project_name, CHAINSIGHT_FILENAME), "")?;

    if !no_samples {
        return create_sample_components(project_name, "sample");
    }

    fs::write(
        format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
        ProjectManifestData::new(project_name, PROJECT_MANIFEST_VERSION, &[]).to_str_as_yaml()?,
    )?;

    Ok(())
}

fn create_sample_components(project_name: &str, component_prefix: &str) -> anyhow::Result<()> {
    let relative_event_indexer_path = format!("components/{}_event_indexer.yaml", component_prefix);
    let relative_algorithm_indexer_path =
        format!("components/{}_algorithm_indexer.yaml", component_prefix);
    let relative_snapshot_indexer_evm_path =
        format!("components/{}_snapshot_indexer_evm.yaml", component_prefix);
    let relative_snapshot_indexer_icp_path =
        format!("components/{}_snapshot_indexer_icp.yaml", component_prefix);
    let relative_relayer_path = format!("components/{}_relayer.yaml", component_prefix);
    let relative_algorithmlens_path =
        format!("components/{}_algorithm_lens.yaml", component_prefix);
    let relative_snapshot_indexer_https_path = format!(
        "components/{}_snapshot_indexer_https.yaml",
        component_prefix
    );
    fs::write(
        format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
        ProjectManifestData::new(
            project_name,
            PROJECT_MANIFEST_VERSION,
            &[
                ProjectManifestComponentField::new(&relative_event_indexer_path, None),
                ProjectManifestComponentField::new(&relative_algorithm_indexer_path, None),
                ProjectManifestComponentField::new(&relative_snapshot_indexer_evm_path, None),
                ProjectManifestComponentField::new(&relative_snapshot_indexer_icp_path, None),
                ProjectManifestComponentField::new(&relative_relayer_path, None),
                ProjectManifestComponentField::new(&relative_algorithmlens_path, None),
                ProjectManifestComponentField::new(&relative_snapshot_indexer_https_path, None),
            ],
        )
        .to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_event_indexer_path),
        template_event_indexer_manifest(component_prefix).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_algorithm_indexer_path),
        template_algorithm_indexer_manifest(component_prefix).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_snapshot_indexer_evm_path),
        template_snapshot_indexer_evm_manifest(component_prefix).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_snapshot_indexer_icp_path),
        template_snapshot_indexer_icp_manifest(component_prefix).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_relayer_path),
        template_relayer_manifest(component_prefix).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_algorithmlens_path),
        template_algorithm_lens_manifest(component_prefix).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_name, relative_snapshot_indexer_https_path),
        template_snapshot_indexer_https_manifest(component_prefix).to_str_as_yaml()?,
    )?;

    Ok(())
}

fn template_event_indexer_manifest(prefix: &str) -> EventIndexerComponentManifest {
    EventIndexerComponentManifest::new(
        &format!("{}_event_indexer", prefix),
        "",
        PROJECT_MANIFEST_VERSION,
        EventIndexerDatasource::default(),
        3600,
    )
}

fn template_algorithm_indexer_manifest(prefix: &str) -> AlgorithmIndexerComponentManifest {
    AlgorithmIndexerComponentManifest::new(
        &format!("{}_algorithm_indexer", prefix),
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmIndexerDatasource::default(),
        vec![AlgorithmIndexerOutput::default()],
        3600,
    )
}

fn template_snapshot_indexer_evm_manifest(prefix: &str) -> SnapshotIndexerEVMComponentManifest {
    SnapshotIndexerEVMComponentManifest::new(
        &format!("{}_snapshot_indexer_evm", prefix),
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::default_contract(),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_snapshot_indexer_icp_manifest(prefix: &str) -> SnapshotIndexerICPComponentManifest {
    SnapshotIndexerICPComponentManifest::new(
        &format!("{}_snapshot_indexer_icp", prefix),
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::default_canister(true),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_snapshot_indexer_https_manifest(prefix: &str) -> SnapshotIndexerHTTPSComponentManifest {
    SnapshotIndexerHTTPSComponentManifest::new(
        &format!("{}_snapshot_indexer_https", prefix),
        "",
        PROJECT_MANIFEST_VERSION,
        SnapshotIndexerHTTPSDataSource::default(),
        SnapshotStorage::default(),
        3600,
    )
}

fn template_relayer_manifest(prefix: &str) -> RelayerComponentManifest {
    RelayerComponentManifest::new(
        &format!("{}_relayer", prefix),
        "",
        PROJECT_MANIFEST_VERSION,
        Datasource::default_canister(false),
        DestinationField::default(),
        3600,
    )
}
fn template_algorithm_lens_manifest(prefix: &str) -> AlgorithmLensComponentManifest {
    AlgorithmLensComponentManifest::new(
        &format!("{}_algorithm_lens", prefix),
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmLensDataSource::default(),
    )
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;

    use crate::commands::test::tests::{run_with_teardown, test_env};

    use super::*;
    fn teardown(project_name: &str) {
        fs::remove_dir_all(project_name).unwrap();
    }

    const COMPONENT_PREFIX: &str = "sample";

    #[test]
    fn test_create_project() {
        let project_name = "new_test_create_project";
        run_with_teardown(
            || {
                let created = create_project(project_name, false);
                assert!(created.is_ok());
                assert!(Path::new(project_name).exists());
                assert!(Path::new(&format!("{}/{}", project_name, CHAINSIGHT_FILENAME)).exists());
                assert!(
                    Path::new(&format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME)).exists()
                );
                assert!(Path::new(&format!("{}/{}", project_name, GITIGNORE_FILENAME)).exists());
                [
                    "event_indexer",
                    "algorithm_indexer",
                    "snapshot_indexer_evm",
                    "snapshot_indexer_icp",
                    "relayer",
                    "algorithm_lens",
                    "snapshot_indexer_https",
                ]
                .iter()
                .for_each(|manifest| {
                    assert!(Path::new(&format!(
                        "{}/components/{}_{}.yaml",
                        project_name, COMPONENT_PREFIX, manifest
                    ))
                    .exists());
                });
            },
            || {
                teardown(project_name);
            },
        )
    }
    #[test]
    fn test_create_project_without_samples() {
        let project_name = "new_test_create_project_without_samples";
        run_with_teardown(
            || {
                let created = create_project(project_name, true);
                assert!(created.is_ok());
                assert!(Path::new(project_name).exists());
                assert!(Path::new(&format!("{}/{}", project_name, CHAINSIGHT_FILENAME)).exists());
                assert!(
                    Path::new(&format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME)).exists()
                );
                assert!(Path::new(&format!("{}/{}", project_name, GITIGNORE_FILENAME)).exists());
                assert!(Path::new(&format!("{}/components", project_name))
                    .read_dir()
                    .unwrap()
                    .next()
                    .is_none());
            },
            || {
                teardown(project_name);
            },
        )
    }
    #[test]
    fn test_exec() {
        let project_name = "new_test_exec";
        run_with_teardown(
            || {
                let env = &test_env();
                let opts = NewOpts {
                    project_name: project_name.to_string(),
                    no_samples: false,
                };
                let res = exec(env, opts);
                assert!(res.is_ok());
            },
            || teardown(project_name),
        )
    }

    #[test]
    fn test_manifest_snapshot_event_indexer() {
        assert_display_snapshot!(template_event_indexer_manifest(COMPONENT_PREFIX)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_snapshot_indexer_chain() {
        assert_display_snapshot!(template_snapshot_indexer_chain_manifest(COMPONENT_PREFIX)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_snapshot_indexer_icp() {
        assert_display_snapshot!(template_snapshot_indexer_icp_manifest(COMPONENT_PREFIX)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_snapshot_indexer_https() {
        assert_display_snapshot!(template_snapshot_indexer_https_manifest(COMPONENT_PREFIX)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_algorithm_indexer() {
        assert_display_snapshot!(template_algorithm_indexer_manifest(COMPONENT_PREFIX)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_algorithm_lens() {
        assert_display_snapshot!(template_algorithm_lens_manifest(COMPONENT_PREFIX)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_relayer() {
        assert_display_snapshot!(template_relayer_manifest(COMPONENT_PREFIX)
            .to_str_as_yaml()
            .unwrap());
    }
}
