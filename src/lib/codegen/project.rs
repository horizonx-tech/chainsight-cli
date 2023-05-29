use std::{fs::OpenOptions, path::Path, io::{Read, Write}};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProjectManifestData {
    version: String,
    label: String,
    components: Vec<ProjectManifestComponentField>
}
#[derive(Deserialize, Serialize)]
pub struct ProjectManifestComponentField {
    canister_id: String
}

pub fn generate_manifest_for_project(project_name: &str) -> Result<std::string::String, serde_yaml::Error> {
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

pub fn add_new_component_to_project_manifest(path: &str, values: &[&str]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&Path::new(path))
        .unwrap(); // TODO
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // TODO

    let mut data: ProjectManifestData = serde_yaml::from_str(&contents).unwrap(); // TODO
    for value in values {
        data.components.push(ProjectManifestComponentField {
            canister_id: value.to_string()
        });
    }
    let updated_yaml = serde_yaml::to_string(&data).unwrap(); // TODO

    let mut file = OpenOptions::new().write(true).truncate(true).open(&path).unwrap();
    file.write_all(updated_yaml.as_bytes()).unwrap(); // TODO
    
    Ok(())
}