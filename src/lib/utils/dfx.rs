use std::path::Path;

use super::remove_trailing_newline;

#[derive(Clone, Debug)]
pub enum DfxWrapperNetwork {
    IC,
    Local(Option<u64>),
    Custom(String),
}
impl DfxWrapperNetwork {
    pub fn value(&self) -> String {
        match self {
            DfxWrapperNetwork::IC => "https://ic0.app/".to_string(),
            DfxWrapperNetwork::Local(port) => format!("http://localhost:{}", port.unwrap_or(4943)),
            DfxWrapperNetwork::Custom(custom) => custom.clone(),
        }
    }

    pub fn args(&self) -> String {
        format!("--network {}", self.value())
    }
}
impl Default for DfxWrapperNetwork {
    fn default() -> Self {
        DfxWrapperNetwork::IC
    }
}

#[derive(Clone, Debug, Default)]
pub struct DfxWrapper {
    execution_dir_str: Option<String>,
    network: DfxWrapperNetwork,
}

impl DfxWrapper {
    fn execution_dir(&self) -> &Path {
        let dir = self.execution_dir_str.as_deref().unwrap_or(".");
        Path::new(dir)
    }

    pub fn new(network: DfxWrapperNetwork, path: Option<String>) -> Self {
        DfxWrapper {
            execution_dir_str: path,
            network,
        }
    }

    pub fn version(&self) -> Result<String, String> {
        exec_cmd_string_output("dfx", self.execution_dir(), vec!["--version"])
            .map(remove_trailing_newline)
    }
}

fn exec_cmd_none_output(cmd: &str, execution_dir: &Path, args: Vec<&str>) -> Result<(), String> {
    exec_cmd_generic_output(cmd, execution_dir, args, |_stdout| Ok(()))
}

fn exec_cmd_string_output(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> Result<String, String> {
    exec_cmd_generic_output(cmd, execution_dir, args, |stdout| {
        Ok(std::str::from_utf8(&stdout).unwrap().to_string())
    })
}

fn exec_cmd_json_output<T>(cmd: &str, execution_dir: &Path, args: Vec<&str>) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    exec_cmd_generic_output(cmd, execution_dir, args, |stdout| {
        Ok(serde_json::from_slice(&stdout).unwrap())
    })
}

fn exec_cmd_generic_output<T, F>(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
    process_output: F,
) -> Result<T, String>
where
    F: FnOnce(Vec<u8>) -> Result<T, String>,
{
    let output = exec_cmd(cmd, execution_dir, args)
        .unwrap_or_else(|_| panic!("failed to execute process: {}", cmd));
    if output.status.success() {
        process_output(output.stdout)
    } else {
        Err(std::str::from_utf8(&output.stderr).unwrap().to_string())
    }
}

fn exec_cmd(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> std::io::Result<std::process::Output> {
    std::process::Command::new(cmd)
        .current_dir(execution_dir)
        .args(args)
        // .stdout(Stdio::piped())
        .output()
}
