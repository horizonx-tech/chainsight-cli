use std::path::Path;

pub const CHAINSIGHT_FILENAME: &str = ".chainsight";
pub const PROJECT_MANIFEST_FILENAME: &str = "project.yaml";
pub const PROJECT_MANIFEST_VERSION: &str = "v1";

/// Check if .chainsight file exists in project folder
pub fn is_chainsight_project(path: Option<String>) -> Result<(), String> {
    if let Some(path) = path {
        let path = Path::new(&path);
        if !path.exists() {
            return Err(format!("Provided path '{}' does not exist.", path.display()));
        }
        // Check if .chainsight file exists in the provided path
        if !path.join(CHAINSIGHT_FILENAME).exists() {
            return Err(format!("No .chainsight file found in the provided path '{}'.", path.display()));
        }
    } else {
        if !Path::new(CHAINSIGHT_FILENAME).exists() {
            return Err("No .chainsight file found in the current directory.".to_string());
        }
    }
    Ok(())
}
