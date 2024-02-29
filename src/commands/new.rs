use std::{fs, path::Path};

use anyhow::bail;
use clap::Parser;
use inflector::cases::titlecase::to_title_case;
use slog::info;

use crate::lib::{
    codegen::{
        components::{
            algorithm_indexer::{
                AlgorithmIndexerComponentManifest, AlgorithmIndexerDatasource,
                AlgorithmIndexerOutput,
            },
            algorithm_lens::{AlgorithmLensComponentManifest, AlgorithmLensDataSource},
            common::{ComponentManifest, DatasourceForCanister},
            event_indexer::{EventIndexerComponentManifest, EventIndexerDatasource},
            relayer::{DestinationField, RelayerComponentManifest},
            snapshot_indexer_evm::{
                SnapshotIndexerEVMComponentManifest, SnapshotIndexerEVMDatasource,
            },
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
        CHAINSIGHT_FILENAME, DOTENV_FILENAME, GITIGNORE_FILENAME, PROJECT_MANIFEST_FILENAME,
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

    /// Specifies the path of the project example in chainsight-showcase to use.
    #[arg(long)]
    pub example: Option<String>,

    /// Skip generation of sample component manifests.
    #[arg(long, visible_short_alias = 'n')]
    pub no_samples: bool,
}

pub fn exec(env: &EnvironmentImpl, opts: NewOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = Path::new(&opts.project_name);
    let project_name = project_path.file_stem().unwrap().to_str().unwrap();
    if project_path.exists() {
        bail!(format!(r#"Project '{}' already exists"#, project_name));
    }

    let res = if let Some(example) = opts.example {
        info!(
            log,
            r#"Start creating new project '{}' by example '{}'..."#, project_name, example
        );
        create_project_by_example(&project_path.to_string_lossy(), &example)
    } else {
        info!(log, r#"Start creating new project '{}'..."#, project_name);
        create_project(
            &project_path.to_string_lossy(),
            project_name,
            opts.no_samples,
        )
    };

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

fn create_project(project_path: &str, project_name: &str, no_samples: bool) -> anyhow::Result<()> {
    // Create directories
    fs::create_dir_all(format!("{}/components", project_path))?;
    fs::create_dir_all(format!("{}/interfaces", project_path))?;

    // Create files
    fs::write(
        format!("{}/{}", project_path, GITIGNORE_FILENAME),
        gitignore(),
    )?;
    fs::write(format!("{}/{}", project_path, CHAINSIGHT_FILENAME), "")?;

    if !no_samples {
        return create_sample_components(project_path, project_name, "sample");
    }

    fs::write(
        format!("{}/{}", project_path, PROJECT_MANIFEST_FILENAME),
        ProjectManifestData::new(project_name, PROJECT_MANIFEST_VERSION, &[]).to_str_as_yaml()?,
    )?;

    Ok(())
}

fn create_sample_components(
    project_path: &str,
    project_name: &str,
    component_prefix: &str,
) -> anyhow::Result<()> {
    let event_indexer_id = format!("{}_event_indexer", component_prefix);
    let event_indexer_path = format!("components/{}.yaml", event_indexer_id);
    let algorithm_indexer_id = format!("{}_algorithm_indexer", component_prefix);
    let algorithm_indexer_path = format!("components/{}.yaml", algorithm_indexer_id);
    let snapshot_indexer_evm_id = format!("{}_snapshot_indexer_evm", component_prefix);
    let snapshot_indexer_evm_path = format!("components/{}.yaml", snapshot_indexer_evm_id);
    let snapshot_indexer_icp_id = format!("{}_snapshot_indexer_icp", component_prefix);
    let snapshot_indexer_icp_path = format!("components/{}.yaml", snapshot_indexer_icp_id);
    let relayer_id = format!("{}_relayer", component_prefix);
    let relayer_path = format!("components/{}.yaml", relayer_id);
    let algorithm_lens_id = format!("{}_algorithm_lens", component_prefix);
    let algorithm_lens_path = format!("components/{}.yaml", algorithm_lens_id);
    let snapshot_indexer_https_id = format!("{}_snapshot_indexer_https", component_prefix);
    let snapshot_indexer_https_path = format!("components/{}.yaml", snapshot_indexer_https_id);
    fs::write(
        format!("{}/{}", project_path, PROJECT_MANIFEST_FILENAME),
        ProjectManifestData::new(
            project_name,
            PROJECT_MANIFEST_VERSION,
            &[
                ProjectManifestComponentField::new(&event_indexer_path, None),
                ProjectManifestComponentField::new(&algorithm_indexer_path, None),
                ProjectManifestComponentField::new(&snapshot_indexer_evm_path, None),
                ProjectManifestComponentField::new(&snapshot_indexer_icp_path, None),
                ProjectManifestComponentField::new(&relayer_path, None),
                ProjectManifestComponentField::new(&algorithm_lens_path, None),
                ProjectManifestComponentField::new(&snapshot_indexer_https_path, None),
            ],
        )
        .to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, event_indexer_path),
        template_event_indexer_manifest(&event_indexer_id).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, algorithm_indexer_path),
        template_algorithm_indexer_manifest(&algorithm_indexer_id).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, snapshot_indexer_evm_path),
        template_snapshot_indexer_evm_manifest(&snapshot_indexer_evm_id).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, snapshot_indexer_icp_path),
        template_snapshot_indexer_icp_manifest(&snapshot_indexer_icp_id).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, relayer_path),
        template_relayer_manifest(&relayer_id).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, algorithm_lens_path),
        template_algorithm_lens_manifest(&algorithm_lens_id).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, snapshot_indexer_https_path),
        template_snapshot_indexer_https_manifest(&snapshot_indexer_https_id).to_str_as_yaml()?,
    )?;
    fs::write(
        format!("{}/{}", project_path, DOTENV_FILENAME),
        template_dotenv_file_content(),
    )?;

    Ok(())
}

fn template_event_indexer_manifest(id: &str) -> EventIndexerComponentManifest {
    EventIndexerComponentManifest::new(
        id,
        &to_title_case(id),
        "",
        PROJECT_MANIFEST_VERSION,
        EventIndexerDatasource::default(),
        3600,
    )
}

fn template_algorithm_indexer_manifest(id: &str) -> AlgorithmIndexerComponentManifest {
    AlgorithmIndexerComponentManifest::new(
        id,
        &to_title_case(id),
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmIndexerDatasource::default(),
        vec![AlgorithmIndexerOutput::default()],
        3600,
    )
}

fn template_snapshot_indexer_evm_manifest(id: &str) -> SnapshotIndexerEVMComponentManifest {
    SnapshotIndexerEVMComponentManifest::new(
        id,
        &to_title_case(id),
        "",
        PROJECT_MANIFEST_VERSION,
        SnapshotIndexerEVMDatasource::default(),
        3600,
    )
}

fn template_snapshot_indexer_icp_manifest(id: &str) -> SnapshotIndexerICPComponentManifest {
    SnapshotIndexerICPComponentManifest::new(
        id,
        &to_title_case(id),
        "",
        PROJECT_MANIFEST_VERSION,
        DatasourceForCanister::default(),
        3600,
    )
}

fn template_snapshot_indexer_https_manifest(id: &str) -> SnapshotIndexerHTTPSComponentManifest {
    SnapshotIndexerHTTPSComponentManifest::new(
        id,
        &to_title_case(id),
        "",
        PROJECT_MANIFEST_VERSION,
        SnapshotIndexerHTTPSDataSource::default(),
        3600,
    )
}

fn template_relayer_manifest(id: &str) -> RelayerComponentManifest {
    RelayerComponentManifest::new(
        id,
        &to_title_case(id),
        "",
        PROJECT_MANIFEST_VERSION,
        DatasourceForCanister::default(),
        DestinationField::default(),
        3600,
    )
}
fn template_algorithm_lens_manifest(id: &str) -> AlgorithmLensComponentManifest {
    AlgorithmLensComponentManifest::new(
        id,
        &to_title_case(id),
        "",
        PROJECT_MANIFEST_VERSION,
        AlgorithmLensDataSource::default(),
    )
}
fn template_dotenv_file_content() -> String {
    r#"# This file is automatically generated by Chainsight.
INFURA_MAINNET_RPC_URL_KEY=
INFURA_MUMBAI_RPC_URL_KEY=
"#
    .to_string()
}

fn create_project_by_example(_project_path: &str, _example: &str) -> anyhow::Result<()> {
    Ok(())
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
                let created = create_project(project_name, project_name, false);
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
                assert!(Path::new(&format!("{}/{}", project_name, DOTENV_FILENAME)).exists());
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
                let created = create_project(project_name, project_name, true);
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
                assert!(!Path::new(&format!("{}/{}", project_name, DOTENV_FILENAME)).exists());
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
                    example: None,
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
        let id = format!("{}_event_indexer", COMPONENT_PREFIX);
        assert_display_snapshot!(template_event_indexer_manifest(&id)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_snapshot_indexer_evm() {
        let id = format!("{}_snapshot_indexer_evm", COMPONENT_PREFIX);
        assert_display_snapshot!(template_snapshot_indexer_evm_manifest(&id)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_snapshot_indexer_icp() {
        let id = format!("{}_snapshot_indexer_icp", COMPONENT_PREFIX);
        assert_display_snapshot!(template_snapshot_indexer_icp_manifest(&id)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_snapshot_indexer_https() {
        let id = format!("{}_snapshot_indexer_https", COMPONENT_PREFIX);
        assert_display_snapshot!(template_snapshot_indexer_https_manifest(&id)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_algorithm_indexer() {
        let id = format!("{}_algorithm_indexer", COMPONENT_PREFIX);
        assert_display_snapshot!(template_algorithm_indexer_manifest(&id)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_algorithm_lens() {
        let id = format!("{}_algorithm_lens", COMPONENT_PREFIX);
        assert_display_snapshot!(template_algorithm_lens_manifest(&id)
            .to_str_as_yaml()
            .unwrap());
    }

    #[test]
    fn test_manifest_snapshot_relayer() {
        let id = format!("{}_relayer", COMPONENT_PREFIX);
        assert_display_snapshot!(template_relayer_manifest(&id).to_str_as_yaml().unwrap());
    }
}
