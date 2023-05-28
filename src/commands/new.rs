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
    fs::write(format!("{}/project.yaml", project_name), generate_manifest_for_project(project_name).unwrap())?;

    fs::write(
        format!("{}/components/{}-snapshot.yaml", project_name, project_name),
        generate_manifest_for_snapshot(project_name).unwrap().clone()
    )?;
    fs::write(
        format!("{}/components/{}-relayer.yaml", project_name, project_name),
        generate_manifest_for_relayer(project_name).unwrap().clone()
    )?;

    Ok(())
}

#[derive(Serialize)]
struct ProjectManifestData {
    version: String,
    label: String,
    components: Vec<ProjectManifestComponentField>
}
#[derive(Serialize)]
struct ProjectManifestComponentField {
    canister_id: String
}
// temp
fn generate_manifest_for_project(project_name: &str) -> Result<std::string::String, serde_yaml::Error> {
    let data = ProjectManifestData {
        version: "v1".to_owned(),
        label: project_name.to_owned(),
        components: vec![
            ProjectManifestComponentField {
                canister_id: "xxx-xxx-xxx".to_owned()
            },
            ProjectManifestComponentField {
                canister_id: "xxx-xxx-xxx".to_owned()
            },
        ],
    };
    serde_yaml::to_string(&data)
}

#[derive(Serialize)]
enum ComponentType {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "relayer")]
    Relayer,
}
#[derive(Serialize)]
enum DatasourceType {
    #[serde(rename = "canister")]
    Canister,
    #[serde(rename = "contract")]
    Contract,
}
#[derive(Serialize)]
struct SnapshotComponentManifest {
    version: String,
    type_: ComponentType,
    label: String,
    datasource: Datasource,
    interval: u32
}
#[derive(Serialize)]
struct RelayerComponentManifest {
    version: String,
    type_: ComponentType,
    label: String,
    datasource: Datasource,
    destinations: Vec<DestinationField>
}
#[derive(Serialize)]
struct Datasource {
    type_: DatasourceType,
    id: String,
    method: DatasourceMethod
}
#[derive(Serialize)]
struct DatasourceMethod {
    identifier: String,
    args: Vec<String>
}
#[derive(Serialize)]
struct DestinationField {
    network_id: u16,
    oracle: String,
    key: String,
    interval: u32
}
// temp
fn generate_manifest_for_snapshot(project_name: &str) -> Result<std::string::String, serde_yaml::Error> {
    let data = SnapshotComponentManifest {
        version: "v1".to_owned(),
        type_: ComponentType::Snapshot,
        label: format!("{}-snapshot", project_name).to_owned(),
        datasource: Datasource {
            type_: DatasourceType::Contract,
            id: "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_owned(),
            method: DatasourceMethod {
                identifier: "totalSupply()".to_owned(),
                args: vec![]
            },
        },
        interval: 3600,
    };
    serde_yaml::to_string(&data)
}
fn generate_manifest_for_relayer(project_name: &str) -> Result<std::string::String, serde_yaml::Error> {
    let data = RelayerComponentManifest {
        version: "v1".to_owned(),
        type_: ComponentType::Relayer,
        label: format!("{}-relayer", project_name).to_owned(),
        datasource: Datasource {
            type_: DatasourceType::Canister,
            id: "xxx-xxx-xxx".to_owned(),
            method: DatasourceMethod {
                identifier: "total_supply()".to_owned(),
                args: vec![]
            },
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