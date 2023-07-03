use std::path::Path;

use inflector::cases::snakecase::to_snake_case;

pub const CHAINSIGHT_FILENAME: &str = ".chainsight";
pub const PROJECT_MANIFEST_FILENAME: &str = "project.yaml";
pub const PROJECT_MANIFEST_VERSION: &str = "v1";

pub const U256_TYPE: &str = "ic_web3_rs::types::U256";
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
    } else {
        if !Path::new(CHAINSIGHT_FILENAME).exists() {
            return Err("No .chainsight file found in the current directory.".to_string());
        }
    }
    Ok(())
}

/// Convert camelCase String to snake_case
pub fn convert_camel_to_snake(val: &str) -> String {
    // use Inflector instead of this
    // let mut result = String::with_capacity(val.len());
    // let mut chars = val.chars().peekable();
    // while let Some(ch) = chars.next() {
    //     if ch.is_uppercase() {
    //         if !result.is_empty() {
    //             // If not the first character, prepend underscore
    //             result.push('_');
    //         }
    //         for lowercase in ch.to_lowercase() {
    //             result.push(lowercase);
    //         }
    //     } else {
    //         result.push(ch);
    //     }
    // }
    // result

    // NOTE: use Inflator in ic-solidity-bindgen
    // https://github.com/horizonx-tech/ic-solidity-bindgen/blob/0972bede5957927bcb8f675decd93878b849dc76/ic-solidity-bindgen-macros/src/abi_gen.rs#L192
    to_snake_case(val)
}
