use anyhow::ensure;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    lib::codegen::components::algorithm_lens::AlgorithmLensComponentManifest, types::ComponentType,
};

fn common_codes() -> TokenStream {
    quote! {
        use chainsight_cdk_macros::{chainsight_common, did_export, init_in, lens_method};
        use ic_web3_rs::futures::{future::BoxFuture, FutureExt};
        mod app;
        chainsight_common!(60);
        init_in!();
    }
}

fn custom_codes(
    manifest: &AlgorithmLensComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;

    let event_interfaces = &manifest.datasource.input.fields;
    let input_struct_ident = format_ident!("{}", &manifest.datasource.input.name);

    let input_fields_idents: Vec<Ident> = event_interfaces
        .iter()
        .map(|(k, _)| format_ident!("{}", k.clone()))
        .collect();

    let input_types_types: Vec<Ident> = event_interfaces
        .iter()
        .map(|(_, v)| format_ident!("{}", v.clone()))
        .collect();

    let output_struct_ident = format_ident!("{}", &manifest.output.name);
    let output_fields_idents: Vec<Ident> = manifest
        .output
        .fields
        .iter()
        .map(|(k, _)| format_ident!("{}", k.clone()))
        .collect();
    let output_types_idents: Vec<Ident> = manifest
        .output
        .fields
        .iter()
        .map(|(_, v)| format_ident!("{}", v.clone()))
        .collect();

    Ok(quote! {
        #[derive(Clone, Debug,  Default, CandidType, serde::Deserialize, serde::Serialize)]
        pub struct #input_struct_ident {
            #(pub #input_fields_idents: #input_types_types),*
        }
        #[derive(Clone, Debug,  Default, CandidType, serde::Deserialize, serde::Serialize)]
        pub struct #output_struct_ident {
            #(pub #output_fields_idents: #output_types_idents),*
        }
        lens_method!(#input_struct_ident, #output_struct_ident)

        did_export!(#label);
    })
}

pub fn generate_codes(manifest: &AlgorithmLensComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmIndexer,
        "type is not EventIndexer"
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
    let input_type_ident = format_ident!("{}", &manifest.datasource.input.name);
    let output_type_ident = format_ident!("{}", &manifest.output.name);

    let code = quote! {
        use crate::{#input_type_ident, #output_type_ident};
        pub async fn calculate(target: candid::Principal, args: #input_type_ident) -> #output_type_ident {
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

    ensure!(
        !manifest.datasource.input.fields.is_empty(),
        "datasource.event.interface is not set"
    );

    Ok(())
}
