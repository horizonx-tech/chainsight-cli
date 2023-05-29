use std::{fs, path::Path};

use anyhow::bail;
use clap::Parser;
use slog::{error, info};

use crate::lib::{environment::EnvironmentImpl, utils::{is_chainsight_project, CHAINSIGHT_FILENAME, PROJECT_MANIFEST_FILENAME}};

#[derive(Debug, Parser)]
#[command(name = "remove")]
/// Delete resources for specified your project
pub struct RemoveOpts {
    #[arg(long)]
    path: Option<String>,
}

const GLOBAL_ERROR_MSG: &str = "Fail remove command";

pub fn exec(env: &EnvironmentImpl, opts: RemoveOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path = opts.path;

    if let Err(msg) = is_chainsight_project(project_path.clone()) {
        error!(
            log,
            r#"{}"#,
            msg
        );
        bail!(GLOBAL_ERROR_MSG.to_string())
    }

    info!(
        log,
        r#"Removing project..."#
    );

    let res = if let Some(project_name) = project_path.clone() {
        fs::remove_dir_all(Path::new(&project_name))
    } else {
        // TODO: check existence of folders/files before removing
        let _ = fs::remove_dir_all(Path::new(&"components")).map_err(|e| return e);
        let _ = fs::remove_file(CHAINSIGHT_FILENAME).map_err(|e| return e);
        fs::remove_file(PROJECT_MANIFEST_FILENAME)
    };
    match res {
        Ok(_) => {
            info!(
                log,
                r#"Project removed successfully"#
            );
            Ok(())
        },
        Err(err) => {
            error!(
                log,
                r#"Fail to remove project: {}"#,
                err
            );
            bail!(GLOBAL_ERROR_MSG.to_string())
        }
    }
}