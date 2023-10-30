use crate::{
    lib::{
        codegen::components::{
            algorithm_lens::AlgorithmLensComponentManifest, common::ComponentManifest,
        },
        utils::{
            catch_unwind_silent, find_duplicates,
            paths::{self, bindings_name},
        },
    },
    types::ComponentType,
};
use anyhow::{ensure, Result};
use chainsight_cdk::config::components::AlgorithmLensConfig;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::common::{CanisterMethodIdentifier, CanisterMethodValueType};

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

pub fn generate_codes(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmLens,
        "type is not AlgorithmLens"
    );
    custom_codes(manifest)
}

pub fn generate_app(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<TokenStream> {
    let id = manifest.id().ok_or(anyhow::anyhow!("id is required"))?;
    let methods = manifest.datasource.methods.clone();

    let call_func_templates = methods.iter().enumerate().map(|(i, m)| {
        let getter = format_ident!("get_{}", &m.id);
        let method_identifier = &CanisterMethodIdentifier::parse_from_str(&m.identifier).unwrap();
        let result= parse_method_args_idents(method_identifier);
        match result {
            Ok(method_args_idents) => {
                if method_args_idents.is_empty() {
                    quote! {
                        let _result = #getter(targets.get(#i).unwrap().clone()).await;
                    }
                } else {
                    quote! {
                        let _result = #getter(targets.get(#i).unwrap().clone(), #(#method_args_idents::default()),*).await;
                    }
                }
            },
            Err(error) => {
                let message = error.to_string();
                quote! { todo!(#message); }
            }
        }
    });
    let output_type_ident = format_ident!("{}", "LensValue");
    let accessors_ident = format_ident!("{}", paths::accessors_name(&id));

    let code = quote! {
        use #accessors_ident::*;

        #[derive(Clone, Debug,  Default, candid::CandidType, serde::Deserialize, serde::Serialize)]
        pub struct #output_type_ident {
            pub dummy: u64
        }

        pub async fn calculate(targets: Vec<String>, ) -> #output_type_ident {
            #(#call_func_templates)*
            todo!()
        }
    };

    Ok(code)
}

pub fn generate_dependencies_accessor(
    manifest: &AlgorithmLensComponentManifest,
) -> anyhow::Result<TokenStream> {
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

    let code = quote! {
        #(#call_funcs)*
        #call_func_dependencies
    };

    Ok(code)
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

fn create_return_ident(ty: &String, is_scalar: &bool, label: &str) -> (Ident, TokenStream) {
    match is_scalar {
        true => (format_ident!("{}", ty), quote! {}),
        false => {
            let crate_ident = format_ident!("{}", bindings_name(label));
            let ty_ident = format_ident!("{}", ty);
            let return_ty_ident = format_ident!("{}_{}", ty, label);

            (
                return_ty_ident.clone(),
                quote! {
                    use #crate_ident::#ty_ident as #return_ty_ident;
                },
            )
        }
    }
}

fn generate_query_call(label: &str, method_identifier: &str) -> TokenStream {
    let method_identifier = &CanisterMethodIdentifier::parse_from_str(method_identifier).unwrap();
    let func_to_call = &method_identifier.identifier.to_string();
    let proxy_func_to_call = "proxy_".to_string() + func_to_call;

    let result = parse_method_args_idents(method_identifier);
    match result {
        Ok(method_args_idents) => {
            let (method_return_type, method_return_type_import) =
                match &method_identifier.return_value {
                    CanisterMethodValueType::Scalar(ty, is_scalar) => {
                        create_return_ident(ty, is_scalar, label)
                    }
                    CanisterMethodValueType::Vector(ty, is_scalar) => {
                        create_return_ident(ty, is_scalar, label)
                    }
                    _ => (format_ident!("{}", "TODO".to_string()), quote!()), // TODO: support tuple & struct
                };
            match method_identifier.return_value {
                CanisterMethodValueType::Scalar(_, _) => match method_args_idents.is_empty() {
                    true => quote! {
                        #method_return_type_import
                        algorithm_lens_finder!(
                            #label,
                            #proxy_func_to_call,
                            #method_return_type
                        );
                    },
                    false => quote! {
                        #method_return_type_import
                        algorithm_lens_finder!(
                            #label,
                            #proxy_func_to_call,
                            #method_return_type,
                            #(#method_args_idents),*
                        );
                    },
                },
                CanisterMethodValueType::Vector(_, _) => match method_args_idents.is_empty() {
                    true => quote! {
                        #method_return_type_import
                        algorithm_lens_finder!(
                            #label,
                            #proxy_func_to_call,
                            Vec<#method_return_type>
                        );
                    },
                    false => quote! {
                        #method_return_type_import
                        algorithm_lens_finder!(
                            #label,
                            #proxy_func_to_call,
                            Vec<#method_return_type>,
                            #(#method_args_idents),*
                        );
                    },
                },
                _ => quote! {}, // TODO: support tuple & struct,
            }
        }
        _ => quote! {},
    }
}

fn parse_method_args_idents(
    method_identifier: &CanisterMethodIdentifier,
) -> anyhow::Result<Vec<Ident>> {
    let (method_args_idents_results, errors): (Vec<_>, Vec<_>) = {
        method_identifier
            .params
            .iter()
            .map(|arg| catch_unwind_silent(|| format_ident!("{}", arg)))
            .partition(Result::is_ok)
    };

    if !errors.is_empty() {
        return Err(anyhow::anyhow!(
            "Unsupported type found in arguments. Please implement manually: {}",
            method_identifier.identifier
        ));
    }

    Ok(method_args_idents_results
        .into_iter()
        .map(Result::unwrap)
        .collect::<Vec<Ident>>())
}
