use std::path::Path;

pub const CHAINSIGHT_FILENAME: &str = ".chainsight";
pub const PROJECT_MANIFEST_FILENAME: &str = "project.yaml";
pub const PROJECT_MANIFEST_VERSION: &str = "v1";

pub const U256_TYPE: &str = "ic_web3::types::U256";
pub const ADDRESS_TYPE: &str = "ic_web3::types::Address";

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

/// Convert camelCase String to snake_case
pub fn convert_camel_to_snake(val: &str) -> String {
    let mut result = String::with_capacity(val.len());
    let mut chars = val.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if !result.is_empty() {
                // If not the first character, prepend underscore
                result.push('_');
            }
            for lowercase in ch.to_lowercase() {
                result.push(lowercase);
            }
        } else {
            result.push(ch);
        }
    }
    result
}
