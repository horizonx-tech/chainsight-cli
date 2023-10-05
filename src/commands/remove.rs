use std::{fs, io::Write, path::Path};

use anyhow::{bail, Ok};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use slog::{info, warn, Logger};

use crate::lib::{
    codegen::{
        components::common::ComponentTypeInManifest,
        project::{ProjectManifestComponentField, ProjectManifestData},
    },
    environment::EnvironmentImpl,
    utils::{find_duplicates, is_chainsight_project, PROJECT_MANIFEST_FILENAME},
};

#[derive(Debug, Parser)]
#[command(name = "remove")]
/// Delete your Chainsight project. Before this operation, you must delete your canisters.
pub struct RemoveOpts {
    /// Specify the path of the project to be removed.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    pub path: Option<String>,
}

pub fn exec(env: &EnvironmentImpl, opts: RemoveOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path_opt = opts.path;

    if let Err(msg) = is_chainsight_project(project_path_opt.clone()) {
        bail!(format!(r#"{}"#, msg));
    }

    if confirm_to_user(
        "Do you want to select components to delete? (If no, delete the entire project.)",
    ) {
        remove_components(&log, project_path_opt.clone())?;
    } else {
        remove_project(&log, project_path_opt.clone())?;
    }

    Ok(())
}

fn remove_project(log: &Logger, project_path_opt: Option<String>) -> anyhow::Result<()> {
    let (project_path, with_path_parameter) = if let Some(path) = project_path_opt {
        (path, true)
    } else {
        (".".to_string(), false)
    };
    let entries = fs::read_dir(&project_path)?;
    let target_paths = entries
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;
    println!("> Subjects for deletion include the above files and folders.");
    for path in &target_paths {
        println!("{}", path.to_string_lossy());
    }

    if confirm_to_user("Are you sure you want to delete these? (This operation cannot be undone.)")
    {
        let is_delete_with_root = with_path_parameter
            && confirm_to_user("Do you want to delete the project root folder?");
        for path in &target_paths {
            println!("> Deleting: {}", path.to_string_lossy());
            if path.is_file() {
                fs::remove_file(path)?;
            } else {
                fs::remove_dir_all(path)?;
            }
        }

        if is_delete_with_root {
            println!("> Deleting: {}", &project_path);
            fs::remove_dir(&project_path)?;
        }

        info!(log, r#"Project removed successfully"#);
    } else {
        warn!(log, r#"Remove operation has been stopped."#);
    }

    Ok(())
}

fn remove_components(log: &Logger, project_path_opt: Option<String>) -> anyhow::Result<()> {
    let project_path_str = project_path_opt.clone().unwrap_or(".".to_string());
    let project_file_path = format!("{}/{}", project_path_str, PROJECT_MANIFEST_FILENAME);
    let mut project_manifest = ProjectManifestData::load(&project_file_path)?;

    let components = get_components_in_project(&project_path_str, &project_manifest)?;
    let selected_idxs = multi_select_to_user(
        "Which component is to be removed?",
        &components
            .iter()
            .map(|c| c.label.to_string())
            .collect::<Vec<String>>(),
    );
    let selected_components = selected_idxs
        .iter()
        .map(|idx| components[*idx].clone())
        .collect::<Vec<ProjectComponent>>();
    let target_paths = selected_components
        .iter()
        .map(|c| {
            vec![
                format!("{}/src/bindings/{}_bindings", &project_path_str, c.label),
                format!("{}/src/canisters/{}", &project_path_str, c.label),
                format!("{}/src/logics/{}", &project_path_str, c.label),
                format!("{}/{}", &project_path_str, c.manifest_path),
            ]
        })
        .collect::<Vec<Vec<String>>>();
    println!("> Subjects for deletion include the above files and folders.");
    for (i, paths) in target_paths.iter().enumerate() {
        println!(">> Component: {}", selected_components[i].label);
        for path in paths {
            println!("{}", path);
        }
    }
    println!(
        "> Note: Delete also the manifests' paths in the project.yaml of the selected components."
    );

    if confirm_to_user("Are you sure you want to delete these? (This operation cannot be undone.)")
    {
        for (i, paths) in target_paths.iter().enumerate() {
            println!(">> Component: {}", selected_components[i].label);
            for path in paths {
                println!("> Deleting: {}", path);
                let path_buf = Path::new(path);
                if path_buf.is_file() {
                    fs::remove_file(path)?;
                    continue;
                }
                if path_buf.is_dir() {
                    fs::remove_dir_all(path)?;
                    continue;
                }
                continue;
            }
        }

        println!(">> Overwrite project.yaml for the deleted component.");
        println!("> Updating: {}", &project_file_path);
        project_manifest.components = project_manifest
            .components
            .iter()
            .filter(|c| {
                !selected_components
                    .iter()
                    .any(|sc| sc.manifest_path == c.component_path)
            })
            .map(|c| c.clone())
            .collect::<Vec<ProjectManifestComponentField>>();
        let mut project_yml = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&project_file_path)?;
        let contents = project_manifest.to_str_as_yaml()?;
        project_yml.write_all(&contents.as_bytes())?;
        project_yml.flush()?;

        info!(log, r#"Project removed successfully"#);
    } else {
        warn!(log, r#"Remove operation has been stopped."#);
    }

    Ok(())
}

fn confirm_to_user(msg: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .wait_for_newline(true)
        .interact()
        .unwrap()
}

fn multi_select_to_user(msg: &str, items: &Vec<String>) -> Vec<usize> {
    MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .items(items)
        .interact()
        .unwrap()
}

#[derive(Debug, Clone)]
struct ProjectComponent {
    label: String,
    manifest_path: String,
}
fn get_components_in_project(
    project_path: &str,
    project_manifest: &ProjectManifestData,
) -> anyhow::Result<Vec<ProjectComponent>> {
    let component_paths = project_manifest
        .components
        .iter()
        .map(|c| c.component_path.to_string())
        .collect::<Vec<String>>();

    // check duplicated component paths
    {
        let duplicated_paths = find_duplicates(&component_paths);
        if !duplicated_paths.is_empty() {
            bail!(format!(
                r#"Duplicated component paths found: {:?}"#,
                duplicated_paths
            ));
        }
    }

    let components = component_paths
        .iter()
        .map(|path| {
            let component_path = format!("{}/{}", &project_path, path);
            let component = ComponentTypeInManifest::load(&component_path)?;
            Ok(ProjectComponent {
                label: component.metadata.label,
                manifest_path: path.to_string(),
            })
        })
        .collect::<anyhow::Result<Vec<ProjectComponent>>>()?;

    Ok(components)
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::test::tests::{run, test_env},
        lib::utils::CHAINSIGHT_FILENAME,
    };

    use super::*;
    fn setup(project_name: &str) {
        fs::create_dir_all(format!("{}/components", project_name)).unwrap();
        fs::create_dir_all(format!("{}/interfaces", project_name)).unwrap();
        fs::write(format!("{}/{}", project_name, CHAINSIGHT_FILENAME), "").unwrap();
        fs::write(
            format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
            "",
        )
        .unwrap();
    }
    #[test]
    fn test_remove_project() {
        let dummy_teardown = || {};
        let project_name = "remove_test_remove_project";
        run(
            || {
                setup(project_name);
            },
            || {
                let opts = RemoveOpts {
                    path: Some(project_name.to_string()),
                };
                exec(&test_env(), opts).unwrap();
                assert!(Path::new(project_name).exists() == false);
            },
            || dummy_teardown(),
        );
    }
}
