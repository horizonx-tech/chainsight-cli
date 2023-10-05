use std::{fs, path::Path};

use anyhow::bail;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};
use slog::{info, warn, Logger};

use crate::lib::{
    codegen::{components::common::ComponentTypeInManifest, project::ProjectManifestData},
    environment::EnvironmentImpl,
    utils::{
        find_duplicates, is_chainsight_project, CHAINSIGHT_FILENAME, PROJECT_MANIFEST_FILENAME,
    },
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

    let project_path_str = project_path_opt.clone().unwrap_or(".".to_string());
    if confirm_to_user(
        "Do you want to select components to delete? (If no, delete the entire project.)",
    ) {
        // per component
        let _component_names = get_components_in_project(&project_path_str)?;
        todo!();
    } else {
        remove_project(&log, project_path_opt.clone())?;
    }

    // let res = if let Some(project_name) = project_path {
    //     info!(log, r#"Remove project: {}..."#, project_name);
    //     fs::remove_dir_all(Path::new(&project_name))
    // } else {
    //     // TODO: check existence of folders/files before removing
    //     let _ = fs::remove_dir_all(Path::new(&"artifacts"));
    //     let _ = fs::remove_dir_all(Path::new(&"interfaces"));
    //     let _ = fs::remove_dir_all(Path::new(&"components"));
    //     let _ = fs::remove_file(CHAINSIGHT_FILENAME);
    //     fs::remove_file(PROJECT_MANIFEST_FILENAME)
    // };
    // match res {
    //     Ok(_) => {
    //         info!(log, r#"Project removed successfully"#);
    //         Ok(())
    //     }
    //     Err(err) => {
    //         bail!(format!(r#"Failed: Remove project: {}"#, err));
    //     }
    // }

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

fn confirm_to_user(msg: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .wait_for_newline(true)
        .interact()
        .unwrap()
}

#[derive(Debug)]
struct ProjectComponent {
    label: String,
    manifest_path: String,
}
fn get_components_in_project(project_path: &str) -> anyhow::Result<Vec<ProjectComponent>> {
    let project_file_path = format!("{}/{}", project_path, PROJECT_MANIFEST_FILENAME);
    let project_manifest = ProjectManifestData::load(&project_file_path)?;
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
    use crate::commands::test::tests::{run, test_env};

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
