use std::fs;

use clap::Parser;
use slog::{info, error};

use crate::{lib::{environment::EnvironmentImpl, utils::{is_chainsight_project, PROJECT_MANIFEST_FILENAME}, codegen::{components::{generate_manifest_for_relayer, generate_manifest_for_snapshot}, project::add_new_component_to_project_manifest}}, types::ComponentType};

#[derive(Debug, Parser)]
#[command(name = "create")]
/// Create new component & add to your ChainSight's project
pub struct CreateOpts {
    #[arg(required = true)]
    component_name: String,
    #[arg(long)]
    type_: ComponentType,
    #[arg(long)]
    path: Option<String>,
}

const GLOBAL_ERROR_MSG: &str = "Fail create command";

pub fn exec(env: &EnvironmentImpl, opts: CreateOpts) -> Result<(), String> {
    let log = env.get_logger();
    let component_name = opts.component_name;
    let component_type = opts.type_;
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        error!(
            log,
            r#"{}"#,
            msg
        );
        return Err(GLOBAL_ERROR_MSG.to_string());
    }

    info!(
        log,
        r#"Creating new component "{}"..."#,
        component_name
    );

    let res = match component_type {
        ComponentType::Snapshot => generate_manifest_for_snapshot(&component_name),
        ComponentType::Relayer => generate_manifest_for_relayer(&component_name)
    };
    match res {
        Ok(codes) => {
            let (component_file_path, project_file_path) = if let Some(project_name) = project_path.clone() {
                (
                    format!("{}/components/{}.yaml", project_name, component_name),
                    format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
                )
            } else {
                (
                    format!("components/{}.yaml", component_name),
                    PROJECT_MANIFEST_FILENAME.to_string(),
                )
            };
            // write to .yaml
            fs::write(
                component_file_path,
                codes
            ).unwrap(); // TODO
            // update to project.yaml
            add_new_component_to_project_manifest(&project_file_path, &vec!["xxx-xxx-xxx"]).unwrap(); // TODO

            info!(
                log,
                r#"{:?} component "{}" created successfully"#,
                component_type,
                component_name
            );
        },
        Err(err) => {
            error!(
                log,
                r#"Fail to create {:?} component "{}": {}"#,
                component_type,
                component_name,
                err
            );
            return Err(GLOBAL_ERROR_MSG.to_string());
        }
    }

    Ok(())
}