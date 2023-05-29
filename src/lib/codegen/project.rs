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
    component_path: String,
    canister_id: Option<String>
}

pub fn generate_manifest_for_project(project_name: &str, version: &str, components_values: &[(String, Option<String>)]) -> Result<std::string::String, serde_yaml::Error> {
    let components = components_values.iter().map(|(component_path, canister_id)| {
        ProjectManifestComponentField {
            component_path: component_path.to_owned(),
            canister_id: canister_id.clone()
        }
    }).collect::<Vec<_>>();
    let data = ProjectManifestData {
        version: version.to_owned(),
        label: project_name.to_owned(),
        components,
    };
    serde_yaml::to_string(&data)
}

pub fn add_new_component_to_project_manifest(path: &str, components_values: &[(String, Option<String>)]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&Path::new(path))
        .unwrap(); // TODO
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // TODO

    let mut data: ProjectManifestData = serde_yaml::from_str(&contents).unwrap(); // TODO
    for value in components_values {
        data.components.push(ProjectManifestComponentField {
            component_path: value.0.to_owned(),
            canister_id: value.1.clone()
        });
    }
    let updated_yaml = serde_yaml::to_string(&data).unwrap(); // TODO

    let mut file = OpenOptions::new().write(true).truncate(true).open(&path).unwrap();
    file.write_all(updated_yaml.as_bytes()).unwrap(); // TODO
    
    Ok(())
}