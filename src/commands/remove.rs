use std::{fs, path::Path};

use anyhow::bail;
use clap::Parser;
use slog::info;

use crate::lib::{
    environment::EnvironmentImpl,
    utils::{is_chainsight_project, CHAINSIGHT_FILENAME, PROJECT_MANIFEST_FILENAME},
};

#[derive(Debug, Parser)]
#[command(name = "remove")]
/// Delete your Chainsight project. Before this operation, you must delete your canisters.
pub struct RemoveOpts {
    /// Specify the path of the project to be removed.
    /// If not specified, the current directory is targeted.
    #[arg(long)]
    pub path: Option<String>,
}

pub fn exec(env: &EnvironmentImpl, opts: RemoveOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        bail!(format!(r#"{}"#, msg));
    }

    let res = if let Some(project_name) = project_path {
        info!(log, r#"Remove project: {}..."#, project_name);
        fs::remove_dir_all(Path::new(&project_name))
    } else {
        // TODO: check existence of folders/files before removing
        let _ = fs::remove_dir_all(Path::new(&"artifacts"));
        let _ = fs::remove_dir_all(Path::new(&"interfaces"));
        let _ = fs::remove_dir_all(Path::new(&"components"));
        let _ = fs::remove_file(CHAINSIGHT_FILENAME);
        fs::remove_file(PROJECT_MANIFEST_FILENAME)
    };
    match res {
        Ok(_) => {
            info!(log, r#"Project removed successfully"#);
            Ok(())
        }
        Err(err) => {
            bail!(format!(r#"Failed: Remove project: {}"#, err));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::test::tests::{run, test_env};

    use super::*;
    fn setup(project_name: &str) {
        fs::create_dir_all(format!("{}/components", project_name)).unwrap();
        fs::create_dir_all(format!("{}/interfaces", project_name)).unwrap();
        fs::write(format!("{}/{}", project_name, CHAINSIGHT_FILENAME), "").unwrap();
        fs::write(
            format!("{}/{}", project_name, PROJECT_MANIFEST_FILENAME),
            "",
        )
        .unwrap();
    }
    #[test]
    fn test_remove_project() {
        let dummy_teardown = || {};
        let project_name = "remove_test_remove_project";
        run(
            || {
                setup(project_name);
            },
            || {
                let opts = RemoveOpts {
                    path: Some(project_name.to_string()),
                };
                exec(&test_env(), opts).unwrap();
                assert!(Path::new(project_name).exists() == false);
            },
            || dummy_teardown(),
        );
    }
}
