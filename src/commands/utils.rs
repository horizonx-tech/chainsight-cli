use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::bail;
use ic_agent::{Agent, Identity};

use crate::{
    lib::utils::{is_chainsight_project, ARTIFACTS_DIR},
    types::Network,
};

// Return artifacts path as working directory
pub fn working_dir(project_path: Option<String>) -> anyhow::Result<String> {
    let path = if let Some(project_path) = &project_path {
        // Assuming the specified path is the project path, calculate the artifacts path.
        if let Err(msg) = is_chainsight_project(Some(project_path.clone())) {
            bail!(format!(r#"{}"#, msg));
        };
        format!("{}/{}", project_path, ARTIFACTS_DIR)
    } else {
        let current_path_str = ".";
        let current_path = Path::new(current_path_str);
        // If dfx.json is available, current path is used as artifacts path
        if current_path.join("dfx.json").exists() {
            current_path_str.to_string()
        } else {
            // Determine if it is a project path.
            if let Err(msg) = is_chainsight_project(None) {
                bail!(format!(r#"{}"#, msg));
            };
            format!("./{}", ARTIFACTS_DIR)
        }
    };

    Ok(path)
}

pub async fn get_agent(
    network: &Network,
    port: Option<u16>,
    identity: Option<Box<dyn Identity>>,
) -> anyhow::Result<Agent> {
    let mut builder = Agent::builder().with_url(network.to_url(port));
    if let Some(identity) = identity {
        builder = builder.with_identity(identity);
    }

    let agent = builder.build()?;
    if network == &Network::Local {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}

pub fn output_by_exec_cmd(
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> std::io::Result<Output> {
    Command::new(cmd)
        .current_dir(execution_dir)
        .args(args)
        .output()
}

pub struct DfxArgsBuilder {
    network: Network,
    with_component_flag: bool,
    component: Option<String>,
}
impl DfxArgsBuilder {
    pub fn new(network: Network, component: Option<String>) -> Self {
        Self {
            network,
            with_component_flag: true,
            component,
        }
    }

    pub fn new_only_network(network: Network) -> Self {
        Self {
            network,
            with_component_flag: false,
            component: None,
        }
    }

    pub fn generate<'a>(&'a self, args: Vec<&'a str>) -> Vec<&'a str> {
        let mut args = args.clone();

        // network
        args = match self.network {
            Network::Local => args,
            Network::IC => Self::with_ic_network(args),
        };

        // component
        args = if self.with_component_flag {
            if let Some(c) = &self.component {
                Self::with_component(args, c)
            } else {
                Self::with_all(args)
            }
        } else {
            args
        };

        args
    }

    pub fn with_ic_network(mut args: Vec<&str>) -> Vec<&str> {
        args.push("--network");
        args.push("ic");
        args
    }

    pub fn with_all(mut args: Vec<&str>) -> Vec<&str> {
        args.push("--all");
        args
    }

    pub fn with_component<'a>(mut args: Vec<&'a str>, component: &'a str) -> Vec<&'a str> {
        args.push(component);
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfx_args_builder_only_network() {
        struct Input<'a> {
            pub cmd: Vec<&'a str>,
            pub network: Network,
        }
        struct InOut<'a> {
            pub in_: Input<'a>,
            pub out: String,
        }

        let input_output: Vec<InOut> = vec![
            InOut {
                in_: Input {
                    cmd: vec!["identity", "whoami"],
                    network: Network::Local,
                },
                out: "identity whoami".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["identity", "whoami"],
                    network: Network::IC,
                },
                out: "identity whoami --network ic".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["identity", "get-principal"],
                    network: Network::Local,
                },
                out: "identity get-principal".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["identity", "get-principal"],
                    network: Network::IC,
                },
                out: "identity get-principal --network ic".to_string(),
            },
        ];

        for InOut { in_, out } in input_output {
            let args_builder = DfxArgsBuilder::new_only_network(in_.network);
            let actual = args_builder.generate(in_.cmd);
            assert_eq!(actual.join(" "), out);
        }
    }

    #[test]
    fn test_dfx_args_builder_with_components() {
        struct Input<'a> {
            pub cmd: Vec<&'a str>,
            pub network: Network,
            pub component: Option<String>,
        }
        struct InOut<'a> {
            pub in_: Input<'a>,
            pub out: String,
        }

        let input_output: Vec<InOut> = vec![
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::Local,
                    component: None,
                },
                out: "canister create --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::IC,
                    component: None,
                },
                out: "canister create --network ic --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::Local,
                    component: Some("icrc1_component".to_string()),
                },
                out: "canister create icrc1_component".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["canister", "create"],
                    network: Network::IC,
                    component: Some("icrc1_component".to_string()),
                },
                out: "canister create --network ic icrc1_component".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::Local,
                    component: None,
                },
                out: "build --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::IC,
                    component: None,
                },
                out: "build --network ic --all".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::Local,
                    component: Some("icrc1_component".to_string()),
                },
                out: "build icrc1_component".to_string(),
            },
            InOut {
                in_: Input {
                    cmd: vec!["build"],
                    network: Network::IC,
                    component: Some("icrc1_component".to_string()),
                },
                out: "build --network ic icrc1_component".to_string(),
            },
        ];

        for InOut { in_, out } in input_output {
            let args_builder = DfxArgsBuilder::new(in_.network, in_.component);
            let actual = args_builder.generate(in_.cmd);
            assert_eq!(actual.join(" "), out);
        }
    }
}
