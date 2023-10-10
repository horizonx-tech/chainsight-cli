use std::{panic, path::Path};

use inflector::cases::snakecase::to_snake_case;

pub mod clap;
pub mod env;
pub mod interaction;
pub mod paths;
pub mod serializer;

pub const CHAINSIGHT_FILENAME: &str = ".chainsight";
pub const PROJECT_MANIFEST_FILENAME: &str = "project.yaml";
pub const PROJECT_MANIFEST_VERSION: &str = "v1";
pub const DOTENV_FILENAME: &str = ".env";
pub const GITIGNORE_FILENAME: &str = ".gitignore";
pub const ARTIFACTS_DIR: &str = "artifacts";

/// To handle 256bits Unsigned Integer type in ic_web3_rs
pub const U256_TYPE: &str = "ic_web3_rs::types::U256";
/// To handle Address type in ic_web3_rs
pub const ADDRESS_TYPE: &str = "ic_web3_rs::types::Address";

/// Check if .chainsight file exists in project folder
pub fn is_chainsight_project(path: Option<String>) -> Result<(), String> {
    let path_str = path.unwrap_or(".".to_string());
    let path = Path::new(&path_str);
    // Check if .chainsight file exists in the provided or default path
    if !path.join(CHAINSIGHT_FILENAME).exists() {
        return Err(format!(
            "'{}' is not a chainsight project root (no '{}' file exists).",
            path.display(),
            CHAINSIGHT_FILENAME
        ));
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

pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(
    f: F,
) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}
