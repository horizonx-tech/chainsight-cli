use std::fmt::format;

use anyhow::ensure;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::{types::ComponentType, lib::{codegen::components::{SnapshotComponentManifest, DatasourceType, Datasource, DatasourceMethodArg}, utils::convert_camel_to_snake}};

fn common_codes_for_contract() -> TokenStream {
    quote! {
        use std::str::FromStr;
        use chainsight_cdk_macros::{manage_single_state, setup_func, manage_vec_state, timer_task_func, define_transform_for_web3, define_web3_ctx, monitoring_canister_metrics, did_export};
        use ic_web3::types::Address;

        monitoring_canister_metrics!(60);
        define_web3_ctx!();
        define_transform_for_web3!();
        manage_single_state!("target_addr", String, false);
        setup_func!({
            target_addr: String,
            web3_ctx_param: Web3CtxParam
        });
        timer_task_func!("set_task", "execute_task", true);

        #[derive(Debug, Clone, candid::CandidType, candid::Deserialize)]
        pub struct Snapshot {
            pub value: SnapshotValue,
            pub timestamp: u64,
        }
        manage_vec_state!("snapshot", Snapshot, true);
    }
}

fn custom_codes_for_contract(manifest: &SnapshotComponentManifest) -> TokenStream {
    let label = &manifest.label;
    let method = &manifest.datasource.method;
    let mut camel_method_ident = method.identifier.clone();
    // method.identifier: remove `()`
    camel_method_ident.pop();
    camel_method_ident.pop();
    let method_ident_str = convert_camel_to_snake(&camel_method_ident);
    let method_ident = format_ident!("{}", method_ident_str);

    let method_interface_name = method.interface.trim_end_matches(".json");
    let contract_struct_ident = format_ident!("{}", method_interface_name);
    let abi_path = format!("./src/{}/abi/{}.json", label, method_interface_name);

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

    // for response types & response values
    let mut response_type_idents: Vec<syn::Ident> = vec![];
    let mut response_val_idents: Vec<proc_macro2::TokenStream> = vec![];
    for (idx, response_type) in method.response_types.iter().enumerate() {
        let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
        let result = match response_type.as_str() {
            "ic_web3::types::U256" => {
                (
                    format_ident!("String"),
                    quote! { res.#idx_lit.to_string(), }
                )
            },
            "ic_web3::types::Address" => (
                format_ident!("String"),
                quote! { hex::encode(res.#idx_lit), }
            ),
            _ => (
                format_ident!("{}", response_type),
                quote! { res.#idx_lit, }
            )
        };
        response_type_idents.push(result.0);
        response_val_idents.push(result.1);
    }

    quote! {
        ic_solidity_bindgen::contract_abi!(#abi_path);

        type SnapshotValue = (#(#response_type_idents,)*);
        async fn execute_task() {
            let current_ts_sec = ic_cdk::api::time() / 1000000;
            let res = #contract_struct_ident::new(
                Address::from_str(&get_target_addr()).unwrap(),
                &web3_ctx().unwrap()
            ).#method_ident(#(#request_val_idents)*).await.unwrap();
            let datum = Snapshot {
                value: (
                    #(#response_val_idents)*
                ),
                timestamp: current_ts_sec,
            };
            add_snapshot(datum.clone());
            ic_cdk::println!("ts={}, snapshot={:?}", datum.timestamp, datum.value);
        }

        did_export!(#label);
    }
}

fn common_codes_for_canister() -> TokenStream {
    quote! {
        use chainsight_cdk_macros::{manage_single_state, setup_func, manage_vec_state, timer_task_func, cross_canister_call_func, monitoring_canister_metrics, did_export};

        monitoring_canister_metrics!(60);

        manage_single_state!("target_canister", String, false);
        setup_func!({
            target_canister: String
        });

        #[derive(Clone, candid::CandidType, candid::Deserialize)]
        pub struct Snapshot {
            pub value: SnapshotValue,
            pub timestamp: u64,
        }
        manage_vec_state!("snapshot", Snapshot, true);
    }
}

fn custom_codes_for_canister(manifest: &SnapshotComponentManifest) -> TokenStream {
    let mut method_ident = manifest.datasource.method.identifier.clone();
    // method.identifier: remove `()`
    method_ident.pop();
    method_ident.pop();
    let call_method_ident = format!("call_{}", method_ident);

    quote! {
        type SnapshotValue = VirtualPrice;
        #[derive(Debug, Clone, candid::CandidType, candid::Deserialize)]
        pub struct VirtualPrice {
            pub value: String,
            pub timestamp: u64,
        }

        type CallCanisterArgs = ();
        type CallCanisterResponse = VirtualPrice;
        cross_canister_call_func!(#method_ident, CallCanisterArgs, CallCanisterResponse);

        timer_task_func!("set_task", "save_snapshot", true);
        async fn save_snapshot() {
            let current_ts_sec = ic_cdk::api::time() / 1000000;
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let price = #call_method_ident(
                target_canister,
                ()
            ).await;
            if let Err(err) = price {
                ic_cdk::println!("error: {:?}", err);
                return;
            }
            let datum = Snapshot {
                value: price.unwrap().clone(),
                timestamp: current_ts_sec,
            };
            add_snapshot(datum.clone());
            ic_cdk::println!("ts={}, value={:?}", datum.timestamp, datum.value);
        }

        did_export!("snapshot_icp");
    }
}

pub fn generate_codes(manifest: &SnapshotComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(manifest.type_ == ComponentType::Snapshot, "type is not Snapshot");

    let (common_code_token, custom_code_token) = if manifest.datasource.type_ == DatasourceType::Canister {
        (
            common_codes_for_canister(),
            custom_codes_for_canister(manifest),
        )
    } else {
        (
            common_codes_for_contract(),
            custom_codes_for_contract(manifest),
        )
    };

    // temp: only chain
    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}
