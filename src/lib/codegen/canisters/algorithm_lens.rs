use crate::{
    lib::{
        codegen::components::{
            algorithm_lens::{AlgorithmLensComponentManifest, AlgorithmLensDataSource},
            common::ComponentManifest,
        },
        utils::{find_duplicates, paths},
    },
    types::ComponentType,
};
use anyhow::ensure;
use chainsight_cdk::{
    config::components::AlgorithmLensConfig, convert::candid::CanisterMethodIdentifier,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn custom_codes(
    manifest: &AlgorithmLensComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let conf: AlgorithmLensConfig = (manifest.clone()).into();
    let conf_json = serde_json::to_string(&conf).unwrap();
    Ok(quote! {
        use chainsight_cdk_macros::def_algorithm_lens_canister;
        def_algorithm_lens_canister!(#conf_json);
    })
}

pub fn generate_codes(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmLens,
        "type is not AlgorithmLens"
    );
    custom_codes(manifest).map(|code| code.to_string())
}

pub fn generate_app(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<String> {
    let id = manifest.id().ok_or(anyhow::anyhow!("id is required"))?;
    let AlgorithmLensComponentManifest {
        datasource: AlgorithmLensDataSource {
            methods, with_args, ..
        },
        ..
    } = manifest;

    let call_func_templates = methods.iter().enumerate().map(|(i, m)| {
        let getter = format_ident!("get_{}", &m.id);
        let method_identifier = CanisterMethodIdentifier::new(&m.identifier).expect("method_identifier parse error");
        let (request_args_type, _) = method_identifier.get_types();
        if request_args_type.is_some() {
            quote! {
                let _result = #getter(targets.get(#i).unwrap().clone(), todo!("Arguments to be used in this call")).await;
            }
        } else {
            quote! {
                let _result = #getter(targets.get(#i).unwrap().clone()).await;
            }
        }
    });

    let output_type_ident = format_ident!("{}", "LensValue");
    let accessors_ident = format_ident!("{}", paths::accessors_name(&id));

    let code = {
        let base_quote = quote! {
            use #accessors_ident::*;

            #[derive(Clone, Debug,  Default, candid::CandidType, serde::Deserialize, serde::Serialize)]
            pub struct #output_type_ident {
                pub dummy: u64
            }
        };

        if with_args.is_some() && with_args.unwrap() {
            // add argument type that is implemented fields by user
            let args_type_ident = format_ident!(
                "{}",
                AlgorithmLensComponentManifest::CALCULATE_ARGS_STRUCT_NAME
            );

            quote! {
                #base_quote

                #[derive(Clone, Debug,  Default, candid::CandidType, serde::Deserialize, serde::Serialize)]
                pub struct #args_type_ident {
                    pub dummy: u64
                }

                pub async fn calculate(targets: Vec<String>, args: #args_type_ident) -> #output_type_ident {
                    #(#call_func_templates)*
                    todo!()
                }
            }
        } else {
            quote! {
                #base_quote

                pub async fn calculate(targets: Vec<String>) -> #output_type_ident {
                    #(#call_func_templates)*
                    todo!()
                }
            }
        }
    };

    Ok(code.to_string())
}

pub fn generate_dependencies_accessor(
    manifest: &AlgorithmLensComponentManifest,
) -> anyhow::Result<String> {
    let methods = manifest.datasource.methods.clone();

    let call_func_dependencies = quote! {
        use chainsight_cdk::lens::LensFinder;
        use chainsight_cdk_macros::algorithm_lens_finder;
        async fn _get_target_proxy(target: candid::Principal) -> candid::Principal {
            let out: ic_cdk::api::call::CallResult<(candid::Principal,)> = ic_cdk::api::call::call(target, "get_proxy", ()).await;
            out.unwrap().0
        }
    };
    let call_funcs = methods
        .iter()
        .map(|m| generate_query_call(&m.id, &m.identifier));

    let bindings_ident = format_ident!("{}", paths::bindings_name(&manifest.id().unwrap()));
    let code = quote! {
        use #bindings_ident as bindings;
        #(#call_funcs)*
        #call_func_dependencies
    };

    Ok(code.to_string())
}
pub fn validate_manifest(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmLens,
        "type is not AlgorithmLens"
    );

    // check duplicated id in dependencies
    let dependencies = manifest.dependencies();
    let duplicateds = find_duplicates(&dependencies);
    ensure!(
        &duplicateds.is_empty(),
        "duplicated id found in datasource.methods: {}",
        duplicateds
            .iter()
            .map(|s| (**s).as_str())
            .collect::<Vec<&str>>()
            .join(", ")
    );

    Ok(())
}

fn generate_query_call(label: &str, method_identifier: &str) -> TokenStream {
    let method_identifier =
        CanisterMethodIdentifier::new(method_identifier).expect("method_identifier parse error");
    let proxy_func_to_call = format!("proxy_{}", &method_identifier.identifier);

    let crate_name_ident = format_ident!("bindings");
    let module_name_ident = format_ident!("{}", label);
    let response_type_idents = format_ident!("{}", CanisterMethodIdentifier::RESPONSE_TYPE_NAME);
    let (request_args_type, _) = method_identifier.get_types();
    if request_args_type.is_some() {
        let request_args_type_idents =
            format_ident!("{}", CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME,);
        quote! {
            algorithm_lens_finder!(
                #label,
                #proxy_func_to_call,
                #crate_name_ident::#module_name_ident::#response_type_idents,
                #crate_name_ident::#module_name_ident::#request_args_type_idents
            );
        }
    } else {
        quote! {
            algorithm_lens_finder!(
                #label,
                #proxy_func_to_call,
                #crate_name_ident::#module_name_ident::#response_type_idents
            );
        }
    }
}
