use anyhow::ensure;
use chainsight_cdk::{
    config::components::{RelayerConfig, LENS_FUNCTION_ARGS_TYPE},
    convert::candid::CanisterMethodIdentifier,
    web3::ContractFunction,
};
use ethabi::{Param, ParamType};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    lib::{
        codegen::components::{
            algorithm_lens::AlgorithmLensComponentManifest,
            relayer::RelayerComponentManifest,
            utils::{generate_method_identifier, get_did_by_component_id, is_lens_with_args},
        },
        utils::paths::bindings_name,
    },
    types::ComponentType,
};
const CALL_ARGS_STRUCT_NAME: &str = "ContractCallArgs";

#[derive(Clone)]
struct ContractCall {
    contract_function: ContractFunction,
}

impl ContractCall {
    fn new(contract_function: ContractFunction) -> Self {
        Self { contract_function }
    }

    fn call_args(&self) -> Vec<Param> {
        self.contract_function.call_args()
    }

    fn call_args_struct(&self) -> TokenStream {
        let names: Vec<Ident> = self
            .call_args()
            .clone()
            .into_iter()
            .map(|arg| arg.name)
            .map(|name| format_ident!("{}", name))
            .collect();
        let types: Vec<TokenStream> = self
            .call_args()
            .clone()
            .into_iter()
            .map(|arg| Self::kind_to_ty(arg.kind))
            .collect();
        let visibly = format_ident!("{}", "pub");
        let struct_ident = format_ident!("{}", CALL_ARGS_STRUCT_NAME);
        quote! {
            #[derive(Clone, Debug)]
            pub struct #struct_ident {
                #(#visibly #names: #types),*
            }
            impl #struct_ident {
                pub fn new(#(#names: #types),*) -> Self {
                    Self {
                        #(#names),*
                    }
                }
            }
        }
    }

    fn kind_to_ty(p: ParamType) -> TokenStream {
        match p {
            ParamType::Address => quote! { ethabi::Address },
            ParamType::Bytes => quote! { Vec<u8> },
            ParamType::FixedBytes(_) => quote! { Vec<u8> },
            ParamType::Uint(_) => quote! { ic_web3_rs::types::U256 },
            ParamType::Int(_) => quote! { ic_web3_rs::types::U256 },
            ParamType::Bool => quote! { bool },
            ParamType::String => quote! { String },
            ParamType::Array(i) => {
                let inner = Self::kind_to_ty(*i);
                quote! { Vec<#inner> }
            }
            ParamType::FixedArray(i, _) => {
                let inner = Self::kind_to_ty(*i);
                quote! { Vec<#inner> }
            }
            ParamType::Tuple(_) => quote! { Vec<ethabi::Token> },
        }
    }
}

pub fn generate_codes(manifest: &RelayerComponentManifest) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Relayer,
        "type is not Relayer"
    );
    let config: RelayerConfig = manifest.clone().into();
    let config_json = serde_json::to_string(&config)?;
    let code = quote! {
        use chainsight_cdk_macros::def_relayer_canister;
        def_relayer_canister!(#config_json);
    };
    Ok(code.to_string())
}

fn custom_converter(manifest: &RelayerComponentManifest) -> TokenStream {
    let config: RelayerConfig = manifest.clone().into();
    let contract_function = ContractFunction::new(
        "src/".to_owned() + &config.abi_file_path,
        config.method_name,
    );

    match contract_function.call_args().len() {
        0 => quote! {},
        1 => quote! {},
        _ => {
            let call = ContractCall::new(contract_function.clone());
            let args_struct = call.call_args_struct();
            let struct_ident = format_ident!("{}", CALL_ARGS_STRUCT_NAME);
            quote! {
                #args_struct
                pub fn convert(_: &CallCanisterResponse) -> #struct_ident {
                    todo!()
                }
            }
        }
    }
}

pub fn generate_app(manifest: &RelayerComponentManifest) -> anyhow::Result<String> {
    let method = manifest.datasource.method.clone();
    let interface = if method.interface.is_some() {
        method.interface.clone()
    } else {
        get_did_by_component_id(&manifest.datasource.location.id)
    };
    let method_identifier = generate_method_identifier(&method.identifier, &interface)?;
    let converter = custom_converter(manifest);
    let call_args_idents = if manifest.lens_targets.is_some() {
        if is_lens_with_args(method_identifier) {
            let id = manifest.id.clone().expect("id is not set");
            let bindings = format_ident!("{}", bindings_name(&id));
            let lens_args_ident = format_ident!("{}", LENS_FUNCTION_ARGS_TYPE);
            let calculate_args_ident = format_ident!(
                "{}",
                AlgorithmLensComponentManifest::CALCULATE_ARGS_STRUCT_NAME
            );
            quote! {
                pub type #calculate_args_ident = #bindings::#calculate_args_ident;
                pub type #lens_args_ident = #bindings::#lens_args_ident;
                pub fn call_args() -> #calculate_args_ident {
                    todo!("generate CalculateArgs as args to call")
                }
            }
        } else {
            quote! {}
        }
    } else {
        let (request_args_type, _) = method_identifier.get_types();

        if request_args_type.is_some() {
            let request_args_ident =
                format_ident!("{}", CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME);
            quote! {
                pub type CallCanisterArgs = types::#request_args_ident;
                pub fn call_args() -> CallCanisterArgs {
                    todo!()
                }
            }
        } else {
            quote! {
                pub type CallCanisterArgs = ();
                pub fn call_args() -> CallCanisterArgs {
                    ()
                }
            }
        }
    };

    let response_ident = format_ident!("{}", CanisterMethodIdentifier::RESPONSE_TYPE_NAME);
    Ok(quote! {
        mod types;
        use crate::ic_web3_rs::ethabi;
        use ic_web3_rs;
        pub type CallCanisterResponse = types::#response_ident;
        #call_args_idents
        #converter
        pub fn filter(_: &CallCanisterResponse) -> bool {
            true
        }
    }
    .to_string())
}

pub fn validate_manifest(manifest: &RelayerComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Relayer,
        "type is not Relayer"
    );

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length
    // - check destination.type

    Ok(())
}
