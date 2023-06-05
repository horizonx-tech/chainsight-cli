use anyhow::ensure;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::{types::ComponentType, lib::codegen::{components::{relayer::RelayerComponentManifest, common::{DestinactionType, DatasourceMethodArg}}, oracle::get_oracle_attributes}};

// temp
fn common_codes() -> TokenStream {
    quote! {
        use std::str::FromStr;
        use chainsight_cdk_macros::{manage_single_state, setup_func, timer_task_func, cross_canister_call_func, define_web3_ctx, define_transform_for_web3, define_get_ethereum_address, monitoring_canister_metrics, did_export};
        use ic_web3::types::{Address, U256};

        monitoring_canister_metrics!(60);
        define_web3_ctx!();
        define_transform_for_web3!();
        define_get_ethereum_address!();

        manage_single_state!("target_canister", String, false);
        manage_single_state!("target_addr", String, false);

        setup_func!({
            target_canister: String,
            target_addr: String,
            web3_ctx_param: Web3CtxParam
        });

        timer_task_func!("set_task", "sync", true);
    }
}

// temp
fn custom_codes(manifest: &RelayerComponentManifest) -> TokenStream {
    let label = &manifest.label;
    let method = &manifest.datasource.method;
    let mut method_ident = method.identifier.clone();
    // method.identifier: remove `()`
    method_ident.pop();
    method_ident.pop();
    let call_method_ident = format_ident!("call_{}", method_ident);

    // from destination: about oracle
    let destination = &manifest.destination;
    let (oracle_name_str, _, _) = get_oracle_attributes(&destination.type_);
    let oracle_ident = format_ident!("{}", oracle_name_str);
    let abi_path = format!("./__interfaces/{}.json", oracle_name_str);

    // for response type
    let response_type_ident = format_ident!("{}", &method.response_types[0]); // temp

    // for request values
    let mut request_val_idents = vec![];
    for method_args in &method.args {
        let DatasourceMethodArg { type_, value } = method_args;
        // temp
        let result = match type_.as_str() {
            "ic_web3::types::U256" => {
                match value {
                    serde_yaml::Value::String(val) => quote! { ic_web3::types::U256::from_dec_str(#val).unwrap(), },
                    serde_yaml::Value::Number(val) => {
                        match val.as_u64() {
                            Some(val) => quote! { #val.into(), },
                            None => quote! {}
                        }
                    },
                    _ => quote! {}
                }
            }
            "ic_web3::types::Address" => {
                match value {
                    serde_yaml::Value::String(val) => quote! { ic_web3::types::Address::from_str(#val).unwrap(), },
                    _ => quote! {}
                }
            },
            _ => {
                match value {
                    serde_yaml::Value::String(val) => {
                        quote! { #val, }
                    },
                    serde_yaml::Value::Number(val) => {
                        match val.as_u64() {
                            Some(val) => {
                                let type_ident = format_ident!("{}", type_);
                                quote! { #val as #type_ident, }
                            },
                            None => {
                                quote! {}
                            }
                        }
                    },
                    _ => {
                        quote! {}
                    }
                }
            }
        };
        request_val_idents.push(result);
    }

    // define custom_struct
    let mut custom_struct_ident: Vec<proc_macro2::TokenStream> = vec![];
    if let Some(custom_structs) = &method.custom_struct {
        for custom_struct_def in custom_structs {
            let struct_ident = format_ident!("{}", &custom_struct_def.name);
            let mut custom_struct_fields = vec![];
            for field in &custom_struct_def.fields {
                let field_name_ident = format_ident!("{}", &field.name);
                let field_type_ident = format_ident!("{}", &field.type_);
                custom_struct_fields.push(quote! {
                    pub #field_name_ident: #field_type_ident,
                });
            }
            custom_struct_ident.push(quote! {
                #[derive(Debug, Clone, candid::CandidType, candid::Deserialize)]
                pub struct #struct_ident {
                    #(#custom_struct_fields)*
                }
            });
        }
    }

    // define custom_type
    let mut custom_type_ident: Vec<proc_macro2::TokenStream> = vec![];
    if let Some(custom_types) = &method.custom_type {
        for custom_type_def in custom_types {
            let type_ident = format_ident!("{}", &custom_type_def.name);
            let mut custom_type_scalars = vec![];
            for type_ in &custom_type_def.types {
                custom_type_scalars.push(format_ident!("{}", &type_));
            }
            custom_type_ident.push(quote! {
                type #type_ident = (#(#custom_type_scalars),*);
            });
        }
    }

    // define data to call update function of oracle
    // temp: args for update_state (support only default manifest)
    let sync_data_ident: proc_macro2::TokenStream = match &destination.type_ {
        DestinactionType::Uint256Oracle => quote! { U256::from_str(&datum.value).unwrap() },
        DestinactionType::StringOracle => quote! { &datum.value.to_string()},
    };

    quote! {
        ic_solidity_bindgen::contract_abi!(#abi_path);

        #(#custom_struct_ident)*
        #(#custom_type_ident)*

        type CallCanisterArgs = ();
        type CallCanisterResponse = (#response_type_ident);
        cross_canister_call_func!(#method_ident, CallCanisterArgs, CallCanisterResponse);

        async fn sync() {
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let res = #call_method_ident(
                target_canister,
                (#(#request_val_idents)*)
            ).await;
            if let Err(err) = res {
                ic_cdk::println!("error: {:?}", err);
                return;
            }
            let datum = res.unwrap();

            #oracle_ident::new(
                Address::from_str(&get_target_addr()).unwrap(),
                &web3_ctx().unwrap()
            ).update_state(#sync_data_ident).await.unwrap();
            ic_cdk::println!("ts={}, value={:?}", datum.timestamp, datum.value);
        }

        did_export!(#label);
    }
}

pub fn generate_codes(manifest: &RelayerComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(manifest.type_ == ComponentType::Relayer, "type is not Relayer");

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest);

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}
