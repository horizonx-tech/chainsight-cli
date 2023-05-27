use std::{path::Path, fs, io};

use clap::Parser;
use serde::Serialize;
use slog::{info, error};

use crate::lib::environment::EnvironmentImpl;

#[derive(Debug, Parser)]
#[command(name = "new")]
/// Generate ChainSight project by prepared template
pub struct NewOpts {
    #[arg(required = true)]
    project_name: String,
}

const GLOBAL_ERROR_MSG: &str = "Fail new command";

pub fn exec(env: &EnvironmentImpl, opts: NewOpts) -> Result<(), String> {
    let log = env.get_logger();
    let project_name = opts.project_name;
    let project_name_path = Path::new(&project_name);
    if project_name_path.exists() {
        error!(
            log,
            r#"Project "{}" already exists"#,
            project_name
        );
        return Err(GLOBAL_ERROR_MSG.to_string());
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
            return Err(GLOBAL_ERROR_MSG.to_string());
        }
    }
}

fn create_project(project_name: &str) -> io::Result<()> {
    // Create directories
    fs::create_dir_all(format!("{}/components", project_name))?;

    // Create files
    fs::write(format!("{}/.chainsight", project_name), "")?;
    fs::write(format!("{}/project.yaml", project_name), generate_codes_for_project_data().unwrap())?;

    let component_file_code = generate_codes_for_component_file().unwrap();
    fs::write(
        format!("{}/components/component_a.yaml", project_name),
        component_file_code.clone()
    )?;
    fs::write(
        format!("{}/components/component_b.yaml", project_name),
        component_file_code
    )?;

    Ok(())
}

#[derive(Serialize)]
struct ProjectData {
    version: String,
    label: String,
    components: Vec<ProjectComponentField>
}
#[derive(Serialize)]
struct ProjectComponentField {
    canister_id: String
}
// temp
fn generate_codes_for_project_data() -> Result<std::string::String, serde_yaml::Error> {
    let data = ProjectData {
        version: "v1".to_owned(),
        label: "rvol-volatility".to_owned(),
        components: vec![
            ProjectComponentField {
                canister_id: "xxx-xxx-xxx".to_owned()
            },
            ProjectComponentField {
                canister_id: "xxx-xxx-xxx".to_owned()
            },
        ],
    };
    serde_yaml::to_string(&data)
}

#[derive(Serialize)]
struct ComponentData {
    version: String,
    type_: String,
    label: String,
    data_source: DataSourceField,
    destinations: Vec<DestinationField> // for relayer?
}
#[derive(Serialize)]
struct DataSourceField {
    canister_id: String,
    method_id: String
}
#[derive(Serialize)]
struct DestinationField {
    network_id: u16,
    oracle: String,
    key: String,
    interval: u32
}
// temp
fn generate_codes_for_component_file() -> Result<std::string::String, serde_yaml::Error> {
    let data = ComponentData {
        version: "v1".to_owned(),
        type_: "relayer".to_owned(),
        label: "rvol-volatility-relayer".to_owned(),
        data_source: DataSourceField {
            canister_id: "xxx-xxx-xxx".to_owned(),
            method_id: "xxxx".to_owned()
        },
        destinations: vec![
            DestinationField {
                network_id: 1,
                oracle: "0xaaaaaaaaaaaaaaaaaaaaa".to_owned(),
                key: "5fd4d8f912a7be9759c2d039168362925359f379c0e92d4bdbc7534806faa5bb".to_owned(),
                interval: 3600,
            },
        ],
    };
    serde_yaml::to_string(&data)
}