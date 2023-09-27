use crate::{
    lib::{
        codegen::components::algorithm_lens::{
            AlgorithmLensComponentManifest, AlgorithmLensOutputType,
        },
        utils::paths,
    },
    types::ComponentType,
};
use anyhow::ensure;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

fn common_codes() -> TokenStream {
    quote! {
        use chainsight_cdk::lens::LensFinder;
        use chainsight_cdk_macros::{chainsight_common, did_export, init_in, algorithm_lens_finder, lens_method};
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
    let output_ident = match &manifest.output.type_ {
        AlgorithmLensOutputType::Primitive => {
            let primitive_type = format_ident!(
                "{}",
                &manifest
                    .clone()
                    .output
                    .type_name
                    .expect("field type_name required")
            );
            let input_count = manifest.datasource.methods.len();

            quote! {
                use #logic_ident::*;
                lens_method!(#primitive_type, #input_count);
                did_export!(#label);
            }
        }
        AlgorithmLensOutputType::Struct => {
            let output_struct_ident = format_ident!(
                "{}",
                &manifest.clone().output.name.expect("field name reuired")
            );
            let input_count = manifest.datasource.methods.len();
            quote! {
                use #logic_ident::*;
                lens_method!(#output_struct_ident, #input_count);
                did_export!(#label);
            }
        }
    };

    Ok(output_ident)
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
    let call_func_imports = methods.iter().map(|m| {
        let bindings_ident = format_ident!("{}", paths::bindings_name(&m.label));
        let call_func_ident = format_ident!("get_{}", &m.label);
        quote! {
            use #bindings_ident::#call_func_ident;
        }
    });
    let call_func_templates = methods.iter().map(|m| {
        let getter = format_ident!("get_{}", &m.label);
        quote! {
            let _result = #getter(targets.get(0).unwrap().clone()).await;
        }
    });

    let output_type_ident = match &manifest.output.type_ {
        AlgorithmLensOutputType::Struct => format_ident!(
            "{}",
            &manifest.clone().output.name.expect("filed name reuired")
        ),
        AlgorithmLensOutputType::Primitive => format_ident!(
            "{}",
            &manifest
                .clone()
                .output
                .type_name
                .expect("filed type_name reuired")
        ),
    };
    let imports = match &manifest.output.type_ {
        AlgorithmLensOutputType::Struct => {
            let output_fields_idents: Vec<Ident> = manifest
                .clone()
                .output
                .fields
                .expect("fields required")
                .keys()
                .map(|k| format_ident!("{}", k.clone()))
                .collect();
            let output_types_idents: Vec<Ident> = manifest
                .clone()
                .output
                .fields
                .expect("fields required")
                .values()
                .map(|v| format_ident!("{}", v.clone()))
                .collect();
            quote! {
                #[derive(Clone, Debug,  Default, candid::CandidType, serde::Deserialize, serde::Serialize)]
                pub struct #output_type_ident {
                    #(pub #output_fields_idents: #output_types_idents),*
                }

                #(#call_func_imports)*
            }
        }
        AlgorithmLensOutputType::Primitive => quote! {
            #(#call_func_imports)*
        },
    };

    let code = quote! {
        #imports

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
