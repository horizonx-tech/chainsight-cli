use std::{panic, path::Path};

pub mod clap;
pub mod env;
pub mod interaction;
pub mod paths;
pub mod serializer;
pub mod url;

pub const CHAINSIGHT_FILENAME: &str = ".chainsight";
pub const PROJECT_MANIFEST_FILENAME: &str = "project.yaml";
pub const PROJECT_MANIFEST_VERSION: &str = "v1";
pub const DOTENV_FILENAME: &str = ".env";
pub const GITIGNORE_FILENAME: &str = ".gitignore";
pub const ARTIFACTS_DIR: &str = "artifacts";

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

#[allow(dead_code)]
pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(
    f: F,
) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}
