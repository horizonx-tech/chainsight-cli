use crate::lib::utils::paths;

pub fn root_cargo_toml(
    component_ids: Vec<String>,
    with_bindings: bool,
    with_accessors: bool,
) -> String {
    let mut members = component_ids
        .iter()
        .flat_map(|x| vec![format!("canisters/{}", x), format!("logics/{}", x)])
        .collect::<Vec<String>>();
    if with_bindings {
        members.push("bindings/*".to_string());
    }
    if with_accessors {
        members.push("accessors/*".to_string());
    }

    format!(
        r#"[workspace]
members = [{}]

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
candid = "0.9.6"
ic-cdk = "0.11.3"
ic-cdk-macros = "0.8.1"
ic-cdk-timers = "0.5.0"
ic-stable-structures = "0.5.5"
serde = "1.0.163"
serde_bytes = "0.11.12"
serde_json = "1.0.108"
hex = "0.4.3"

ic-web3-rs = "0.1.4"
ic-solidity-bindgen = "0.1.11"
chainsight-cdk-macros = {{ git = "https://github.com/horizonx-tech/chainsight-sdk.git", rev= "011a2b066743870ccc3b205f34fbcb028e11e880" }}
chainsight-cdk = {{ git = "https://github.com/horizonx-tech/chainsight-sdk.git", rev= "011a2b066743870ccc3b205f34fbcb028e11e880" }}
"#,
        members
            .iter()
            .map(|x| format!(r#""{}""#, x))
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub fn logic_cargo_toml(
    project_name: &str,
    with_bindings: bool,
    dependencies: Vec<String>,
) -> String {
    let txt = format!(
        r#"[package]
name = "{}"
version.workspace = true
edition.workspace = true

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
serde_json.workspace = true
hex.workspace = true

ic-web3-rs.workspace = true
ic-solidity-bindgen.workspace = true
chainsight-cdk-macros.workspace = true
chainsight-cdk.workspace = true

{}
{}
"#,
        project_name,
        if with_bindings {
            paths::bindings_dependency(project_name)
        } else {
            "".to_string()
        },
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
version.workspace = true
edition.workspace = true

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

pub fn canister_project_cargo_toml(project_name: &str, with_bindings: bool) -> String {
    format!(
        r#"[package]
name = "{}"
version.workspace = true
edition.workspace = true

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

ciborium = "0.2.1"

{}
{}
"#,
        paths::canister_name(project_name),
        paths::logic_dependency(project_name),
        if with_bindings {
            paths::bindings_dependency(project_name)
        } else {
            "".to_string()
        }
    )
}

pub fn bindings_cargo_toml(component: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version.workspace = true
edition.workspace = true

[lib]
crate-type = ["rlib"]

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-stable-structures.workspace = true
serde.workspace = true
serde_bytes.workspace = true

chainsight-cdk-macros.workspace = true
chainsight-cdk.workspace = true
"#,
        paths::bindings_name(component)
    )
}

pub fn dfx_json(project_ids: Vec<String>) -> String {
    let canisters = project_ids
        .iter()
        .map(|label| {
            format!(
                r#"    "{}": {{
      "type": "custom",
      "candid": "./{}.did",
      "wasm": "./{}.wasm",
      "metadata": [
        {{
          "name": "candid:service",
          "visibility": "public"
        }}
      ]
    }}"#,
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
/artifacts/*
!/artifacts/dfx.json
!/artifacts/canister_ids.json
.env
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;

    use super::*;

    const PROJECT_NAME: &str = "sample";

    #[test]
    fn test_snapshot_root_cargo_toml() {
        let project_ids = vec![
            "sample_snapshot".to_string(),
            "sample_lens".to_string(),
            "sample_relayer".to_string(),
        ];
        assert_display_snapshot!(
            "snapshot_root_cargo_toml",
            root_cargo_toml(project_ids.clone(), false, false)
        );
        assert_display_snapshot!(
            "snapshot_root_cargo_toml_with_bindings",
            root_cargo_toml(project_ids.clone(), true, false)
        );
        assert_display_snapshot!(
            "snapshot_root_cargo_toml_with_accessors",
            root_cargo_toml(project_ids.clone(), true, true)
        );
    }

    #[test]
    fn test_snapshot_logic_cargo_toml() {
        let dependencies = vec!["sample_snapshot".to_string(), "sample_lens".to_string()];
        assert_display_snapshot!(logic_cargo_toml(PROJECT_NAME, false, dependencies))
    }

    #[test]
    fn test_snapshot_accessors_cargo_toml() {
        let dependencies = vec!["sample_snapshot".to_string(), "sample_lens".to_string()];
        assert_display_snapshot!(accessors_cargo_toml(PROJECT_NAME, dependencies))
    }

    #[test]
    fn test_snapshot_canister_cargo_toml() {
        assert_display_snapshot!(canister_project_cargo_toml(PROJECT_NAME, false))
    }

    #[test]
    fn test_snapshot_bindings_cargo_toml() {
        assert_display_snapshot!(bindings_cargo_toml(PROJECT_NAME))
    }

    #[test]
    fn test_snapshot_dfx_json() {
        let project_ids = vec![
            "sample_snapshot".to_string(),
            "sample_lens".to_string(),
            "sample_relayer".to_string(),
        ];
        assert_display_snapshot!(dfx_json(project_ids))
    }

    #[test]
    fn test_snapshot_gitignore() {
        assert_display_snapshot!(gitignore())
    }
}
