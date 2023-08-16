use crate::{
    lib::codegen::{
        canisters::common::CanisterMethodValueType,
        components::algorithm_lens::{candid_binding_file_name, AlgorithmLensComponentManifest},
    },
    types::ComponentType,
};
use anyhow::ensure;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::common::CanisterMethodIdentifier;

fn common_codes() -> TokenStream {
    quote! {
        use chainsight_cdk::lens::LensFinder;
        use chainsight_cdk_macros::{chainsight_common, did_export, init_in, algorithm_lens_finder, lens_method, manage_single_state, setup_func};
        use ic_web3_rs::futures::{future::BoxFuture, FutureExt};
        use candid::CandidType;
        mod app;
        chainsight_common!(60);
        init_in!();
    }
}

fn custom_codes(
    manifest: &AlgorithmLensComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;

    let method_identifiers = get_method_identifiers(manifest)?;
    let methods = &manifest.datasource.methods;

    let mut algorithm_lens_finders = Vec::<proc_macro2::TokenStream>::new();
    for (i, method_identifier) in method_identifiers.iter().enumerate() {
        let method_label = &methods[i].label;
        let func_to_call = &method_identifier.identifier.to_string();
        let proxy_func_to_call = "proxy_".to_string() + func_to_call;
        let method_args = method_identifier
            .params
            .iter()
            .map(|arg| format_ident!("{}", arg))
            .collect::<Vec<Ident>>();
        let struct_file_name = candid_binding_file_name(&methods[i].candid_file_path);
        let struct_file_ident = format_ident!("{}", struct_file_name);
        let (import_type, method_return_type) = match &method_identifier.return_value {
            CanisterMethodValueType::Scalar(ty, is_scalar) => match is_scalar {
                true => {
                    let ty = format_ident!("{}", ty.to_string());
                    (None, ty)
                }
                false => (
                    Some(format_ident!("{}", ty.to_string())),
                    format_ident!("{}_{}", ty.to_string(), i.to_string()),
                ),
            },
            _ => (None, format_ident!("{}", "TODO".to_string())), // TODO: support tuple & struct
        };
        let import_ident = match import_type {
            None => quote! {},
            Some(import_type) => quote! {
                use crate::#struct_file_ident::#import_type as #method_return_type;
            },
        };

        let ident = if method_args.is_empty() {
            quote! {
                mod #struct_file_ident;
                #import_ident
                algorithm_lens_finder!(
                    #method_label,
                    #proxy_func_to_call,
                    #method_return_type
                );
            }
        } else {
            quote! {
                algorithm_lens_finder!(
                    #method_label,
                    #proxy_func_to_call,
                    #method_return_type,
                    #(#method_args),*
                );
            }
        };

        algorithm_lens_finders.push(ident);
    }

    let output_struct_ident = format_ident!("{}", &manifest.output.name);
    let output_fields_idents: Vec<Ident> = manifest
        .output
        .fields
        .keys()
        .map(|k| format_ident!("{}", k.clone()))
        .collect();
    let output_types_idents: Vec<Ident> = manifest
        .output
        .fields
        .values()
        .map(|v| format_ident!("{}", v.clone()))
        .collect();
    let input_count = manifest.datasource.methods.len();
    Ok(quote! {
        #[derive(Clone, Debug,  Default, CandidType, serde::Deserialize, serde::Serialize)]
        pub struct #output_struct_ident {
            #(pub #output_fields_idents: #output_types_idents),*
        }

        #(#algorithm_lens_finders)*

        lens_method!(#output_struct_ident, #input_count);

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
    let call_func_idents = methods.iter().map(|m| format_ident!("get_{}", &m.label));
    let call_func_templates = call_func_idents.clone().map(|getter| {
        quote! {
            let _result = #getter(targets.get(0).unwrap().clone()).await;
        }
    });

    let output_type_ident = format_ident!("{}", &manifest.output.name);

    let code = quote! {
        use crate::{
            #output_type_ident,
            #(#call_func_idents),*
        };

        pub async fn calculate(targets: Vec<String>, ) -> #output_type_ident {
            #(#call_func_templates)*
            todo!()
        }
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

fn get_method_identifiers(
    manifest: &AlgorithmLensComponentManifest,
) -> anyhow::Result<Vec<CanisterMethodIdentifier>> {
    manifest
        .datasource
        .methods
        .iter()
        .map(|m| CanisterMethodIdentifier::parse_from_str(&m.identifier))
        .collect::<anyhow::Result<Vec<CanisterMethodIdentifier>>>()
}
