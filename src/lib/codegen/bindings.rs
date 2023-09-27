use std::path::Path;

use candid::pretty_check_file;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use regex::Regex;

use crate::lib::utils::paths::canisters_path_str;

use super::{
    canisters::common::{CanisterMethodIdentifier, CanisterMethodValueType},
    components::common::ComponentManifest,
};

pub fn generate_rs_bindings(
    root: &str,
    component: &Box<dyn ComponentManifest>,
) -> anyhow::Result<String> {
    let label = &component.metadata().label;
    let candid_path = &format!("{}/{}.did", &canisters_path_str(root, label), label);
    let bindings = create_candid_rust_binding(Path::new(candid_path))?;

    if let Some(query) = generate_default_query_call(component) {
        return Ok(format!("{}\n\n{}", bindings, query));
    }
    Ok(bindings)
}

fn create_candid_rust_binding(path: &Path) -> anyhow::Result<String> {
    let (env, _) = pretty_check_file(path)?;
    let config = candid::bindings::rust::Config::new();
    let result = candid::bindings::rust::compile(&config, &env, &None)
        .replace("use ic_cdk::api::call::CallResult as Result;", "")
        .replace("pub enum Result", "enum Result");
    let re = Regex::new(r"[^{](\w+): ").unwrap();
    let result = re.replace_all(&result, " pub ${1}: ");
    Ok(result.to_string())
}

fn generate_default_query_call(component: &Box<dyn ComponentManifest>) -> Option<String> {
    if component.default_query_identifier().is_none() {
        return Option::None;
    };
    let method_identifier =
        &CanisterMethodIdentifier::parse_from_str(component.default_query_identifier().unwrap())
            .unwrap();
    let func_to_call = &method_identifier.identifier.to_string();
    let proxy_func_to_call = "proxy_".to_string() + func_to_call;
    let method_label = &component.metadata().label;
    let method_args = method_identifier
        .params
        .iter()
        .map(|arg| format_ident!("{}", arg))
        .collect::<Vec<Ident>>();

    let method_return_type = match &method_identifier.return_value {
        CanisterMethodValueType::Scalar(ty, is_scalar) => match is_scalar {
            true => format_ident!("{}", ty.to_string()),
            false => format_ident!("{}", ty.to_string()),
        },
        _ => format_ident!("{}", "TODO".to_string()), // TODO: support tuple & struct
    };

    let import_macro = quote! {
        use chainsight_cdk::lens::LensFinder;
        use chainsight_cdk_macros::algorithm_lens_finder;

        async fn _get_target_proxy(target: candid::Principal) -> candid::Principal {
            let out: ic_cdk::api::call::CallResult<(candid::Principal,)> = ic_cdk::api::call::call(target, "get_proxy", ()).await;
            out.unwrap().0
        }
    };

    let ident = if method_args.is_empty() {
        quote! {
            #import_macro
            algorithm_lens_finder!(
                #method_label,
                #proxy_func_to_call,
                #method_return_type
            );
        }
    } else {
        quote! {
            #import_macro
            algorithm_lens_finder!(
                #method_label,
                #proxy_func_to_call,
                #method_return_type,
                #(#method_args),*
            );
        }
    };
    Option::Some(ident.to_string())
}
