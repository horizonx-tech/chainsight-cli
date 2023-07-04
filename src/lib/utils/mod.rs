use std::path::Path;

use inflector::cases::snakecase::to_snake_case;

pub mod clap;

pub const CHAINSIGHT_FILENAME: &str = ".chainsight";
pub const PROJECT_MANIFEST_FILENAME: &str = "project.yaml";
pub const PROJECT_MANIFEST_VERSION: &str = "v1";

/// To handle 256bits Unsigned Integer type in ic_web3_rs
pub const U256_TYPE: &str = "ic_web3_rs::types::U256";
/// To handle Address type in ic_web3_rs
pub const ADDRESS_TYPE: &str = "ic_web3_rs::types::Address";

/// Check if .chainsight file exists in project folder
pub fn is_chainsight_project(path: Option<String>) -> Result<(), String> {
    if let Some(path) = path {
        let path = Path::new(&path);
        if !path.exists() {
            return Err(format!(
                "Provided path '{}' does not exist.",
                path.display()
            ));
        }
        // Check if .chainsight file exists in the provided path
        if !path.join(CHAINSIGHT_FILENAME).exists() {
            return Err(format!(
                "No .chainsight file found in the provided path '{}'.",
                path.display()
            ));
        }
    } else if !Path::new(CHAINSIGHT_FILENAME).exists() {
        return Err("No .chainsight file found in the current directory.".to_string());
    }
    Ok(())
}

/// Convert camelCase String to snake_case
pub fn convert_camel_to_snake(val: &str) -> String {
    // NOTE: use Inflator in ic-solidity-bindgen
    // https://github.com/horizonx-tech/ic-solidity-bindgen/blob/0972bede5957927bcb8f675decd93878b849dc76/ic-solidity-bindgen-macros/src/abi_gen.rs#L192
    to_snake_case(val)
}

/// Outputs duplicate values for a given set of elements.
pub fn find_duplicates<T: Eq + std::hash::Hash>(values: &[T]) -> Vec<&T> {
    let mut duplicates = Vec::new();
    let mut set = std::collections::HashSet::new();
    for value in values {
        if !set.insert(value) {
            duplicates.push(value);
        }
    }
    duplicates
}