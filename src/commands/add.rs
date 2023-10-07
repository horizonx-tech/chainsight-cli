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
                algorithm_lens::{AlgorithmLensComponentManifest, AlgorithmLensDataSource},
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
            find_duplicates,
            interaction::{UserInteraction, ValidatorResult},
            is_chainsight_project, PROJECT_MANIFEST_FILENAME, PROJECT_MANIFEST_VERSION,
        },
    },
    types::ComponentType,
};

#[derive(Debug, Parser)]
#[command(name = "add")]
#[clap(alias = "create")]
/// Generates component manifest of specified type and adds to your project.
pub struct AddOpts {
    /// Specifies the name of the component to add.
    component_name: Option<String>,

    /// Specifies type of the component to add.
    #[arg(long)]
    type_: Option<ComponentType>,

    /// Specify the path of the project to which the component is to be added.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    path: Option<String>,
}

pub fn exec<U: UserInteraction>(
    env: &EnvironmentImpl,
    opts: AddOpts,
    interaction: &mut U,
) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        bail!(format!(r#"{}"#, msg));
    }

    let component_name = if let Some(name) = opts.component_name {
        name
    } else {
        input_component_name(interaction)
    };

    let component_type = if let Some(type_) = opts.type_ {
        type_
    } else {
        select_component_type(interaction)
    };

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
        ComponentType::SnapshotIndexerICP => {
            template_snapshot_manifest(&component_name).to_str_as_yaml()
        }
        ComponentType::SnapshotIndexerEVM => {
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
        r#"{:?} component '{}' added successfully"#, component_type, component_name
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

fn input_component_name(interaction: &mut impl UserInteraction) -> String {
    interaction.input("Please input Component Name to add", |input| {
        let chars = input.chars().collect::<Vec<char>>();

        if chars.is_empty() {
            return ValidatorResult::Err("Component Name cannot be empty".to_string());
        }
        for &c in &chars {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return ValidatorResult::Err("Component Name is only single-byte alphanumeric characters or underscores are allowed.".to_string());
            }
        }
        if !chars[0].is_ascii_alphanumeric() || !chars[chars.len() - 1].is_ascii_alphanumeric() {
            return ValidatorResult::Err("Component Name must begin or end with a single-byte alphanumeric character.".to_string());
        }

        ValidatorResult::Ok(())
    })
}

fn select_component_type(interaction: &mut impl UserInteraction) -> ComponentType {
    let all = ComponentType::all();
    let ans = interaction.select(
        "Please select Component Type to add",
        all.iter()
            .map(|comp| format!("{}", comp))
            .collect::<Vec<String>>()
            .as_slice(),
    );
    all[ans]
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use super::*;
    use crate::{
        commands::test::tests::{run, test_env},
        lib::utils::{interaction::MockUserInteraction, CHAINSIGHT_FILENAME},
    };
    fn teardown(project_name: &str) {
        fs::remove_dir_all(project_name).unwrap();
    }
    fn setup(project_name: &str) {
        fs::create_dir(Path::new(project_name)).unwrap();
        fs::create_dir(Path::new(&format!("{}/components", project_name))).unwrap();
        fs::create_dir(Path::new(&format!("{}/interfaces", project_name))).unwrap();
        fs::write(
            format!("{}/{}", project_name, "project.yaml"),
            serde_yaml::to_string(&ProjectManifestData::new(project_name, "1", &[])).unwrap(),
        )
        .unwrap();
        fs::write(format!("{}/{}", project_name, CHAINSIGHT_FILENAME), "").unwrap();
    }

    fn added_projects() -> HashMap<String, ComponentType> {
        let mut projects: HashMap<String, ComponentType> = HashMap::new();
        projects.insert("event_indexer".to_string(), ComponentType::EventIndexer);
        projects.insert(
            "algorithm_indexer".to_string(),
            ComponentType::AlgorithmIndexer,
        );
        projects.insert(
            "snapshot_indexer_icp".to_string(),
            ComponentType::SnapshotIndexerICP,
        );
        projects.insert(
            "snapshot_indexer_evm".to_string(),
            ComponentType::SnapshotIndexerEVM,
        );
        projects.insert("relayer".to_string(), ComponentType::Relayer);
        projects.insert("algorithm_lens".to_string(), ComponentType::AlgorithmLens);
        projects.insert(
            "snapshot_indexer_https".to_string(),
            ComponentType::SnapshotIndexerHTTPS,
        );
        projects
    }

    #[test]
    fn test_add() {
        let projects = added_projects();
        projects.iter().for_each(|(name, component)| {
            let project_name = format!("create_test__add_{}", name);
            run(
                || setup(&project_name),
                || {
                    let opts = AddOpts {
                        component_name: Some(format!("test_{}", name)),
                        type_: Some(*component),
                        path: Some(project_name.to_string()),
                    };
                    let mut interaction = MockUserInteraction::default();
                    exec(&test_env(), opts, &mut interaction).unwrap();
                    assert!(Path::new(&format!(
                        "{}/components/{}.yaml",
                        project_name,
                        format!("test_{}", name)
                    ))
                    .exists());
                },
                || {
                    teardown(&project_name);
                },
            )
        })
    }

    #[test]
    fn test_add_without_args() {
        let projects = added_projects();
        let all_component_types = ComponentType::all();
        projects.iter().for_each(|(name, component)| {
            let project_name = format!("create_test__add_without_args_{}", name);
            run(
                || setup(&project_name),
                || {
                    let opts = AddOpts {
                        component_name: None,
                        type_: None,
                        path: Some(project_name.to_string()),
                    };
                    let component_idx = all_component_types
                        .iter()
                        .position(|c| c == component)
                        .unwrap();
                    let mut interaction = MockUserInteraction {
                        input_answers: vec![format!("test_{}", name)],
                        select_answers: vec![component_idx],
                        ..Default::default()
                    };
                    exec(&test_env(), opts, &mut interaction).unwrap();
                    assert!(Path::new(&format!(
                        "{}/components/{}.yaml",
                        project_name,
                        format!("test_{}", name)
                    ))
                    .exists());
                },
                || {
                    teardown(&project_name);
                },
            )
        })
    }
}
