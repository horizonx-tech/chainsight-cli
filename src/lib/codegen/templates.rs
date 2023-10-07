use crate::lib::utils::paths;

pub fn root_cargo_toml() -> String {
    r#"[workspace]
members = ["canisters/*"]

[workspace.dependencies]
candid = "0.8"
ic-cdk = "0.8"
ic-cdk-macros = "0.6.10"
ic-cdk-timers = "0.1"
ic-stable-structures = "0.5.5"
serde = "1.0.163"
serde_bytes = "0.11.12"
hex = "0.4.3"

ic-web3-rs = { version = "0.1.1" }
ic-solidity-bindgen = { version = "0.1.5" }
chainsight-cdk-macros = { git = "https://github.com/horizonx-tech/chainsight-sdk.git", rev = "5a62683fa207c81959fa023c830977db13258270" }
chainsight-cdk = { git = "https://github.com/horizonx-tech/chainsight-sdk.git", rev = "5a62683fa207c81959fa023c830977db13258270" }
"#.to_string()
}

pub fn logic_cargo_toml(project_name: &str, dependencies: Vec<String>) -> String {
    let txt = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib"]

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-cdk-macros.workspace = true
ic-cdk-timers.workspace = true
ic-stable-structures.workspace = true
serde.workspace = true
serde_bytes.workspace = true
hex.workspace = true

ic-web3-rs.workspace = true
ic-solidity-bindgen.workspace = true
chainsight-cdk-macros.workspace = true
chainsight-cdk.workspace = true

{}
"#,
        project_name,
        if dependencies.is_empty() {
            "".to_string()
        } else {
            paths::accessors_dependency(project_name)
        }
    );

    txt
}

pub fn accessors_cargo_toml(project_name: &str, dependencies: Vec<String>) -> String {
    let txt = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib"]

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-cdk-macros.workspace = true
ic-cdk-timers.workspace = true
ic-stable-structures.workspace = true
serde.workspace = true
serde_bytes.workspace = true
hex.workspace = true

ic-web3-rs.workspace = true
ic-solidity-bindgen.workspace = true
chainsight-cdk-macros.workspace = true
chainsight-cdk.workspace = true

{}
"#,
        paths::accessors_name(project_name),
        dependencies
            .iter()
            .map(|x| paths::bindings_dependency(x))
            .collect::<Vec<String>>()
            .join("\n")
    );

    txt
}

pub fn canister_project_cargo_toml(project_name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-cdk-macros.workspace = true
ic-cdk-timers.workspace = true
ic-stable-structures.workspace = true
serde.workspace = true
serde_bytes.workspace = true
hex.workspace = true

ic-web3-rs.workspace = true
ic-solidity-bindgen.workspace = true
chainsight-cdk-macros.workspace = true
chainsight-cdk.workspace = true

{}
"#,
        paths::canister_name(project_name),
        paths::logic_dependency(project_name),
    )
}

pub fn bindings_cargo_toml(component: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib"]

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
serde.workspace = true
serde_bytes.workspace = true

chainsight-cdk-macros.workspace = true
chainsight-cdk.workspace = true
"#,
        paths::bindings_name(component)
    )
}

pub fn dfx_json(project_labels: Vec<String>) -> String {
    let canisters = project_labels
        .iter()
        .map(|label| {
            format!(
                r#"      "{}": {{
        "type": "custom",
        "candid": "./{}.did",
        "wasm": "./{}.wasm",
        "metadata": [
          {{
            "name": "candid:service",
            "visibility": "public"
          }}
        ]
      }}
"#,
                label, label, label
            )
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let result = format!(
        r#"{{
  "version": 1,
  "canisters": {{
{}
  }},
  "defaults": {{
    "build": {{
      "packtool": "",
      "args": ""
    }}
  }},
  "output_env_file": ".env"
}}
"#,
        canisters
    );

    result
}

pub fn gitignore() -> String {
    r#"src/__interfaces
src/accessors
src/bindings
src/canisters
src/target
artifacts
.env
"#
    .to_string()
}
