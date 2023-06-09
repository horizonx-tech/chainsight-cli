use anyhow::ensure;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::{types::ComponentType, lib::{utils::convert_camel_to_snake, codegen::{components::{snapshot::SnapshotComponentManifest, common::DatasourceType}, canisters::common::{generate_request_arg_idents, generate_outside_call_idents, OutsideCallIdentsType, MethodIdentifier}}}};

fn common_codes_for_contract() -> TokenStream {
    let outside_call_idents = generate_outside_call_idents(OutsideCallIdentsType::Eth);

    quote! {
        use std::str::FromStr;
        use chainsight_cdk_macros::{manage_single_state, setup_func, manage_vec_state, timer_task_func, define_transform_for_web3, define_web3_ctx, monitoring_canister_metrics, did_export};
        use ic_web3::types::Address;

        monitoring_canister_metrics!(60);

        #outside_call_idents

        manage_vec_state!("snapshot", Snapshot, true);
        timer_task_func!("set_task", "execute_task", true);
    }
}

fn custom_codes_for_contract(manifest: &SnapshotComponentManifest) -> TokenStream {
    let label = &manifest.label;
    let method = &manifest.datasource.method;
    let method_identifier = MethodIdentifier::parse_from_abi_str(&method.identifier).expect("Failed to parse method.identifier");
    let method_ident_str = convert_camel_to_snake(&method_identifier.identifier);
    let method_ident = format_ident!("{}", method_ident_str);

    let method_interface = method.interface.clone().expect("method.interface is not defined");
    let contract_struct_ident = format_ident!("{}", method_interface.trim_end_matches(".json"));
    let abi_path = format!("./__interfaces/{}", method_interface);

    // for request values
    // todo: validate length of method.args and method_identifier.params
    let method_args = method.args.iter().enumerate()
        .map(|(idx, arg)| (method_identifier.params[idx].clone(), arg.clone())).collect();
    let (request_val_idents, _) = generate_request_arg_idents(&method_args);

    // for response types & response values
    let mut response_type_idents: Vec<syn::Ident> = vec![];
    let mut response_val_idents: Vec<proc_macro2::TokenStream> = vec![];
    let response_type = syn::parse_str::<syn::Type>(&method.response.type_).unwrap();
    match &response_type {
        syn::Type::Tuple(type_tuple) => {
            // If it's a tuple, we process it like we did before
            for (idx, elem) in type_tuple.elems.iter().enumerate() {
                let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
                let result = match_primitive_type(elem, Some(idx_lit));
                response_type_idents.push(result.0);
                response_val_idents.push(result.1);
            }
        }
        _ => {
            // If it's not a tuple, it must be a primitive type
            let result = match_primitive_type(&response_type, None);
            response_type_idents.push(result.0);
            response_val_idents.push(result.1);
        }
    }

    // consider whether to add timestamp information to the snapshot
    let (
        snapshot_idents,
        expr_to_current_ts_sec,
        expr_to_gen_snapshot,
        expr_to_log_datum
    ) = if manifest.storage.with_timestamp {
        (
            quote! {
                #[derive(Debug, Clone, candid::CandidType, candid::Deserialize)]
                pub struct Snapshot {
                    pub value: SnapshotValue,
                    pub timestamp: u64,
                }
                type SnapshotValue = (#(#response_type_idents),*);
            },
            quote! { let current_ts_sec = ic_cdk::api::time() / 1000000; },
            quote! {
                let datum = Snapshot {
                    value: (
                        #(#response_val_idents),*
                    ),
                    timestamp: current_ts_sec,
                };
            },
            quote! { ic_cdk::println!("ts={}, snapshot={:?}", datum.timestamp, datum.value); }
        )
    } else {
        (
            quote! { type Snapshot = (#(#response_type_idents),*); },
            quote! {},
            quote! { let datum: Snapshot = (#(#response_val_idents),*); },
            quote! { ic_cdk::println!("snapshot={:?}", datum); }
        )
    };

    quote! {
        #snapshot_idents

        ic_solidity_bindgen::contract_abi!(#abi_path);

        async fn execute_task() {
            #expr_to_current_ts_sec
            let res = #contract_struct_ident::new(
                Address::from_str(&get_target_addr()).unwrap(),
                &web3_ctx().unwrap()
            ).#method_ident(#(#request_val_idents),*).await.unwrap();
            #expr_to_gen_snapshot
            add_snapshot(datum.clone());
            #expr_to_log_datum
        }

        did_export!(#label);
    }
}

fn match_primitive_type(ty: &syn::Type, idx: Option<proc_macro2::Literal>) -> (proc_macro2::Ident, proc_macro2::TokenStream) {
    match ty {
        syn::Type::Path(type_path) => {
            let mut type_string = quote! { #type_path }.to_string();
            type_string.retain(|c| !c.is_whitespace());

            match type_string.as_str() {
                "ic_web3::types::U256" => {
                    (
                        format_ident!("String"),
                        match idx {
                            Some(idx_lit) => quote! { res.#idx_lit.to_string() },
                            None => quote! { res.to_string() }
                        }
                    )
                },
                "ic_web3::types::Address" => (
                    format_ident!("String"),
                    match idx {
                        Some(idx_lit) => quote! { hex::encode(res.#idx_lit) },
                        None => quote! { hex::encode(res) }
                    }
                ),
                _ => (
                    format_ident!("{}", type_string),
                    match idx {
                        Some(idx_lit) => quote! { res.#idx_lit },
                        None => quote! { res }
                    }
                )
            }
        },
        _ => panic!("Unsupported type"),
    }
}

fn common_codes_for_canister() -> TokenStream {
    let outside_call_idents = generate_outside_call_idents(OutsideCallIdentsType::CrossCanisterCall);

    quote! {
        use chainsight_cdk_macros::{manage_single_state, setup_func, manage_vec_state, timer_task_func, cross_canister_call_func, monitoring_canister_metrics, did_export};

        monitoring_canister_metrics!(60);

        #outside_call_idents

        manage_vec_state!("snapshot", Snapshot, true);
        timer_task_func!("set_task", "execute_task", true);
    }
}

fn custom_codes_for_canister(manifest: &SnapshotComponentManifest) -> TokenStream {
    let label = &manifest.label;
    let method = &manifest.datasource.method;
    let method_identifier = MethodIdentifier::parse_from_candid_str(&method.identifier).expect("Failed to parse method.identifier");

    let method_ident = &method_identifier.identifier;
    let call_method_ident = format_ident!("call_{}", method_ident);

    // for request values
    // todo: validate length of method.args and method_identifier.params
    let method_args = method.args.iter().enumerate()
        .map(|(idx, arg)| (method_identifier.params[idx].clone(), arg.clone())).collect();
    let (request_val_idents, request_ty_idents) = generate_request_arg_idents(&method_args);

    // for response type
    let response_with_timestamp = manifest.datasource.method.response.with_timestamp;
    let (response_type_ident, response_type_def_ident) = if response_with_timestamp.is_some() && response_with_timestamp.unwrap() {
        let ty_ident = format_ident!("{}", &method.response.type_);
        let ty_with_ts_ident = format_ident!("{}ValueWithTimestamp", &method.response.type_);
        (
            ty_with_ts_ident.clone(),
            quote! {
                #[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
                pub struct #ty_with_ts_ident {
                    pub value: #ty_ident,
                    pub timestamp: u64,
                }
            }
        )
    } else {
        (
            format_ident!("{}", &method.response.type_),
            quote! {}
        )
    };

    // consider whether to add timestamp information to the snapshot
    let (
        snapshot_idents,
        expr_to_current_ts_sec,
        expr_to_gen_snapshot,
        expr_to_log_datum
    ) = if manifest.storage.with_timestamp {
        (
            quote! {
                #[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
                pub struct Snapshot {
                    pub value: SnapshotValue,
                    pub timestamp: u64,
                }
                type SnapshotValue = #response_type_ident;
            },
            quote! { let current_ts_sec = ic_cdk::api::time() / 1000000; },
            quote! {
                let datum = Snapshot {
                    value: res.unwrap().clone(),
                    timestamp: current_ts_sec,
                };
            },
            quote! { ic_cdk::println!("ts={}, value={:?}", datum.timestamp, datum.value); },
        )
    } else {
        (
            quote! { type Snapshot = #response_type_ident; },
            quote! {},
            quote! { let datum = res.unwrap().clone(); },
            quote! { ic_cdk::println!("snapshot={:?}", datum); },
        )
    };

    quote! {
        #snapshot_idents

        #response_type_def_ident

        type CallCanisterArgs = (#(#request_ty_idents),*);
        type CallCanisterResponse = (#response_type_ident);
        cross_canister_call_func!(#method_ident, CallCanisterArgs, CallCanisterResponse);
        async fn execute_task() {
            #expr_to_current_ts_sec
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let res = #call_method_ident(
                target_canister,
                (#(#request_val_idents),*)
            ).await;
            if let Err(err) = res {
                ic_cdk::println!("error: {:?}", err);
                return;
            }
            #expr_to_gen_snapshot
            add_snapshot(datum.clone());
            #expr_to_log_datum
        }

        did_export!(#label);
    }
}

pub fn generate_codes(manifest: &SnapshotComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(manifest.type_ == ComponentType::Snapshot, "type is not Snapshot");

    let (common_code_token, custom_code_token) = match manifest.datasource.type_ {
        DatasourceType::Canister => (
            common_codes_for_canister(),
            custom_codes_for_canister(manifest),
        ),
        DatasourceType::Contract => (
            common_codes_for_contract(),
            custom_codes_for_contract(manifest),
        )
    };

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &SnapshotComponentManifest) -> anyhow::Result<()> {
    ensure!(manifest.type_ == ComponentType::Snapshot, "type is not Snapshot");

    let datasource = &manifest.datasource;
    if datasource.type_ == DatasourceType::Contract {
        ensure!(datasource.method.interface.is_some(), "datasource.method.interface is required for contract");
        ensure!(datasource.method.response.with_timestamp.is_none(), "datasource.method.response.with_timestamp is not supported for contract");
    } else {
        ensure!(datasource.method.response.with_timestamp.is_some(), "datasource.method.response.with_timestamp is required for canister");
    }

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length

    Ok(())
}