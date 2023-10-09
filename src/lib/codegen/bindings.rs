use std::path::Path;

use candid::pretty_check_file;
use regex::Regex;

use crate::lib::utils::paths::canisters_path_str;

use super::components::common::ComponentManifest;

pub fn generate_rs_bindings(
    root: &str,
    component: &dyn ComponentManifest,
) -> anyhow::Result<String> {
    let id = &component.id().unwrap();
    let candid_path = &format!("{}/{}.did", &canisters_path_str(root, id), id);
    let bindings = create_candid_rust_binding(Path::new(candid_path))?;
    Ok(bindings)
}

fn create_candid_rust_binding(path: &Path) -> anyhow::Result<String> {
    let (env, _) = pretty_check_file(path)?;
    let config = candid::bindings::rust::Config::new();
    let result = candid::bindings::rust::compile(&config, &env, &None)
        .replace("use ic_cdk::api::call::CallResult as Result;", "")
        .replace("pub enum Result", "enum Result");
    let re = Regex::new(r"[^{](?:pub )*(\w+): ").unwrap();
    let result = re.replace_all(&result, " pub ${1}: ");
    Ok(result.to_string())
}
