use crate::{
    lib::codegen::{
        canisters::common::CanisterMethodValueType,
        components::algorithm_lens::AlgorithmLensComponentManifest,
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

    let locations = manifest
        .datasource
        .locations
        .iter()
        .map(|l| l.label.to_string())
        .collect::<Vec<_>>();
    let location_idents = locations
        .iter()
        .map(|l| format_ident!("{}", l))
        .collect::<Vec<Ident>>();

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
        let method_return_type = match &method_identifier.return_value {
            CanisterMethodValueType::Scalar(ty) => format_ident!("{}", ty),
            _ => format_ident!("{}", "TODO".to_string()), // TODO: support tuple & struct
        };

        let ident = if method_args.is_empty() {
            quote! {
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

    Ok(quote! {
        #[derive(Clone, Debug,  Default, CandidType, serde::Deserialize, serde::Serialize)]
        pub struct #output_struct_ident {
            #(pub #output_fields_idents: #output_types_idents),*
        }

        #(manage_single_state!(#locations, String, false);)*
        setup_func!({ #(#location_idents: String),* });

        #(#algorithm_lens_finders)*

        lens_method!(#output_struct_ident);

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
    let locations = manifest
        .datasource
        .locations
        .iter()
        .map(|l| l.label.to_string())
        .collect::<Vec<_>>();
    let location_getter_idents = locations
        .iter()
        .map(|location| format_ident!("get_{}", location));
    let locations_template = location_getter_idents
        .clone()
        .enumerate()
        .map(|(i, getter)| {
            let var_ident = format_ident!("_target_principal_{}", i);
            quote! {
                let #var_ident = #getter();
            }
        });

    let _method_identifiers = get_method_identifiers(manifest)?; // TODO: use this to set args
    let methods = manifest.datasource.methods.clone();
    let call_func_idents = methods.iter().map(|m| format_ident!("get_{}", &m.label));
    let call_func_templates = call_func_idents.clone().map(|getter| {
        quote! {
            let _ = #getter(_target_principal.clone()).await;
        }
    });

    let output_type_ident = format_ident!("{}", &manifest.output.name);

    let code = quote! {
        use crate::{
            #output_type_ident,
            #(#location_getter_idents),*,
            #(#call_func_idents),*
        };

        pub async fn calculate() -> #output_type_ident {
            #(#locations_template)*

            let _target_principal = "".to_string();
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
