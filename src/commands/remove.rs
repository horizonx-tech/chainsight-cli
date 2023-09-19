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
    path: Option<String>,
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
