use std::{ffi::OsStr, fmt, path::Path, str::FromStr};

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

    pub fn args(&self) -> Vec<String> {
        let value = match self {
            DfxWrapperNetwork::IC => "ic".to_string(),
            _ => self.value(),
        };
        vec!["--network".to_string(), value]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
}
impl FromStr for Version {
    type Err = &'static str;

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = version.split('-').collect();
        let version_parts: Vec<&str> = parts[0].split('.').collect();

        if version_parts.len() != 3 {
            return Err("Invalid version string");
        }

        let major = version_parts[0]
            .parse::<u32>()
            .map_err(|_| "Invalid major version")?;
        let minor = version_parts[1]
            .parse::<u32>()
            .map_err(|_| "Invalid minor version")?;
        let patch = version_parts[2]
            .parse::<u32>()
            .map_err(|_| "Invalid patch version")?;

        Ok(Version {
            major,
            minor,
            patch,
            pre_release: parts.get(1).map(ToString::to_string),
        })
    }
}
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.pre_release.as_ref() {
            Some(pr) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, pr),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

impl DfxWrapper {
    fn execution_dir(&self) -> &Path {
        let dir = self.execution_dir_str.as_deref().unwrap_or(".");
        Path::new(dir)
    }

    fn args(&self, command: Vec<String>) -> Vec<String> {
        let mut args = vec![command];
        args.push(self.network.args());
        args.concat()
    }

    pub fn new(network: DfxWrapperNetwork, path: Option<String>) -> Self {
        DfxWrapper {
            execution_dir_str: path,
            network,
        }
    }

    pub fn version(&self) -> Result<Version, String> {
        let res = exec_cmd_string_output("dfx", self.execution_dir(), vec!["--version"])
            .map(remove_trailing_newline)?;
        let version_str = res.replace("dfx ", "");
        Version::from_str(&version_str).map_err(|e| e.to_string())
    }

    pub fn identity_whoami(&self) -> Result<String, String> {
        exec_cmd_string_output("dfx", self.execution_dir(), vec!["identity", "whoami"])
            .map(remove_trailing_newline)
    }

    pub fn identity_get_wallet(&self) -> Result<String, String> {
        exec_cmd_string_output(
            "dfx",
            self.execution_dir(),
            self.args(vec!["identity".to_string(), "get-wallet".to_string()]),
        )
        .map(remove_trailing_newline)
    }
}

fn exec_cmd_none_output<I, S>(cmd: &str, execution_dir: &Path, args: I) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    exec_cmd_generic_output(cmd, execution_dir, args, |_stdout| Ok(()))
}

fn exec_cmd_string_output<I, S>(cmd: &str, execution_dir: &Path, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    exec_cmd_generic_output(cmd, execution_dir, args, |stdout| {
        Ok(std::str::from_utf8(&stdout).unwrap().to_string())
    })
}

fn exec_cmd_json_output<T, I, S>(cmd: &str, execution_dir: &Path, args: I) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    exec_cmd_generic_output(cmd, execution_dir, args, |stdout| {
        Ok(serde_json::from_slice(&stdout).unwrap())
    })
}

fn exec_cmd_generic_output<T, F, I, S>(
    cmd: &str,
    execution_dir: &Path,
    args: I,
    process_output: F,
) -> Result<T, String>
where
    F: FnOnce(Vec<u8>) -> Result<T, String>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = exec_cmd(cmd, execution_dir, args)
        .unwrap_or_else(|_| panic!("failed to execute process: {}", cmd));
    if output.status.success() {
        process_output(output.stdout)
    } else {
        Err(std::str::from_utf8(&output.stderr).unwrap().to_string())
    }
}

fn exec_cmd<I, S>(cmd: &str, execution_dir: &Path, args: I) -> std::io::Result<std::process::Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    std::process::Command::new(cmd)
        .current_dir(execution_dir)
        .args(args)
        // .stdout(Stdio::piped())
        .output()
}
