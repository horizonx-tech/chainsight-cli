use std::{
    path::Path,
    process::{Command, Output},
};

use crate::types::Network;

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
