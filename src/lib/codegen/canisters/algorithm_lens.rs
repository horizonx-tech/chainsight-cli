use crate::{
    lib::{
        codegen::components::{
            algorithm_lens::AlgorithmLensComponentManifest, common::ComponentManifest,
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
        mod types;
        pub use types::*;
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

fn generate_query_call(label: &str, method_identifier: &str) -> TokenStream {
    let method_identifier =
        CanisterMethodIdentifier::new(method_identifier).expect("method_identifier parse error");
    let proxy_func_to_call = format!("proxy_{}", &method_identifier.identifier);

    let suffix = label;
    let response_type_idents = format_ident!(
        "{}__{}",
        CanisterMethodIdentifier::RESPONSE_TYPE_NAME,
        &suffix
    );
    let (request_args_type, _) = method_identifier.get_types();
    if request_args_type.is_some() {
        let request_args_type_idents = format_ident!(
            "{}__{}",
            CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME,
            &suffix
        );
        quote! {
            algorithm_lens_finder!(
                #label,
                #proxy_func_to_call,
                #response_type_idents,
                #request_args_type_idents
            );
        }
    } else {
        quote! {
            algorithm_lens_finder!(
                #label,
                #proxy_func_to_call,
                #response_type_idents
            );
        }
    }
}
