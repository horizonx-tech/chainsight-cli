use crate::{
    lib::{
        codegen::components::algorithm_lens::AlgorithmLensComponentManifest,
        utils::paths::bindings_name,
    },
    types::ComponentType,
};
use anyhow::ensure;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::common::{CanisterMethodIdentifier, CanisterMethodValueType};

fn common_codes() -> TokenStream {
    quote! {
        use chainsight_cdk_macros::{chainsight_common, did_export, init_in, lens_method};
        use candid::CandidType;
        use ic_web3_rs::futures::{future::BoxFuture, FutureExt};
        chainsight_common!(60);
        init_in!();
    }
}

fn custom_codes(
    manifest: &AlgorithmLensComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;

    let logic_ident: Ident = format_ident!("{}", label);
    let input_count = manifest.datasource.methods.len();

    Ok(quote! {
        use #logic_ident::*;
        lens_method!(#input_count);
        did_export!(#label);
    })
}

pub fn generate_codes(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmLens,
        "type is not AlgorithmLens"
    );

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest)?;

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn generate_app(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<TokenStream> {
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
        .map(|m| generate_query_call(&m.label, &m.identifier));
    let call_func_templates = methods.iter().map(|m| {
        let getter = format_ident!("get_{}", &m.label);
        quote! {
            let _result = #getter(targets.get(0).unwrap().clone()).await;
        }
    });

    let output_type_ident = format_ident!("{}", "LensValue");
    let imports = quote! {
        #[derive(Clone, Debug,  Default, candid::CandidType, serde::Deserialize, serde::Serialize)]
        pub struct #output_type_ident {
            pub dummy: u64
        }
    };

    let code = quote! {
        #imports

        pub async fn calculate(targets: Vec<String>, ) -> #output_type_ident {
            #(#call_func_templates)*
            todo!()
        }


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

    Ok(())
}

fn create_return_ident(ty: &String, is_scalar: &bool, label: &str) -> (Ident, TokenStream) {
    match is_scalar {
        true => (format_ident!("{}", ty), quote! {}),
        false => {
            let crate_ident = format_ident!("{}", bindings_name(ty));
            let ty_ident = format_ident!("{}_{}", ty, label);
            (
                ty_ident.clone(),
                quote! {
                    use #crate_ident::SnapshotValue as #ty_ident;
                },
            )
        }
    }
}

fn generate_query_call(label: &str, method_identifier: &str) -> TokenStream {
    let method_identifier = &CanisterMethodIdentifier::parse_from_str(method_identifier).unwrap();
    let func_to_call = &method_identifier.identifier.to_string();
    let proxy_func_to_call = "proxy_".to_string() + func_to_call;

    let method_args_idents = method_identifier
        .params
        .iter()
        .map(|arg| format_ident!("{}", arg))
        .collect::<Vec<Ident>>();

    let (method_return_type, method_return_type_import) = match &method_identifier.return_value {
        CanisterMethodValueType::Scalar(ty, is_scalar) => create_return_ident(ty, is_scalar, label),
        CanisterMethodValueType::Vector(ty, is_scalar) => create_return_ident(ty, is_scalar, label),
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
                    #method_return_type,
                    Vec<#method_return_type>,
                    #(#method_args_idents),*
                );
            },
        },
        _ => quote! {}, // TODO: support tuple & struct,
    }
}
