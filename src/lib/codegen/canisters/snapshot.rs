use anyhow::{bail, ensure};
use quote::{format_ident, quote};

use crate::{
    lib::{
        codegen::{
            canisters::common::{
                generate_outside_call_idents, generate_request_arg_idents,
                CanisterMethodIdentifier, CanisterMethodValueType, ContractMethodIdentifier,
                OutsideCallIdentsType,
            },
            components::{common::DatasourceType, snapshot::SnapshotComponentManifest},
        },
        utils::{convert_camel_to_snake, ADDRESS_TYPE, U256_TYPE},
    },
    types::ComponentType,
};

fn common_codes_for_contract() -> proc_macro2::TokenStream {
    let outside_call_idents = generate_outside_call_idents(OutsideCallIdentsType::Eth);

    quote! {
        use std::str::FromStr;
        use chainsight_cdk_macros::{manage_single_state, setup_func, manage_vec_state, timer_task_func, define_transform_for_web3, define_web3_ctx, monitoring_canister_metrics, did_export};
        use ic_web3_rs::types::Address;

        monitoring_canister_metrics!(60);

        #outside_call_idents

        manage_vec_state!("snapshot", Snapshot, true);
        timer_task_func!("set_task", "execute_task", true);
    }
}

fn custom_codes_for_contract(
    manifest: &SnapshotComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;
    let method = &manifest.datasource.method;
    let method_identifier = ContractMethodIdentifier::parse_from_str(&method.identifier)?;
    let method_ident_str = convert_camel_to_snake(&method_identifier.identifier);
    let method_ident = format_ident!("{}", method_ident_str);

    let method_interface = method.interface.clone().ok_or(anyhow::anyhow!(
        "datasource.method.interface is required for contract"
    ))?;
    let contract_struct_ident = format_ident!("{}", method_interface.trim_end_matches(".json"));
    let abi_path = format!("./__interfaces/{}", method_interface);

    // for request values
    ensure!(
        method_identifier.params.len() == method.args.len(),
        "The number of params and args must be the same"
    );
    let method_args = method
        .args
        .iter()
        .enumerate()
        .map(|(idx, arg)| (method_identifier.params[idx].clone(), arg.clone()))
        .collect();
    let (request_val_idents, _) = generate_request_arg_idents(&method_args);

    // for response types & response values
    let mut response_type_idents: Vec<syn::Ident> = vec![];
    let mut response_val_idents: Vec<proc_macro2::TokenStream> = vec![];
    let response_types = method_identifier.return_value;
    match response_types.len() {
        0 => bail!("The number of response types must be greater than 0"),
        1 => {
            // If it's a single type, we process it like we did before
            let ty = syn::parse_str::<syn::Type>(&response_types[0])?;
            let (response_type_ident, response_val_ident) = match_primitive_type(&ty, None)?;
            response_type_idents.push(response_type_ident);
            response_val_idents.push(response_val_ident);
        }
        _ => {
            // If it's not a single type, it must be a tuple
            // In this case, we process it like we did before
            for (idx, elem) in response_types.iter().enumerate() {
                let ty = syn::parse_str::<syn::Type>(elem)?;
                let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
                let (response_type_ident, response_val_ident) =
                    match_primitive_type(&ty, Some(idx_lit))?;
                response_type_idents.push(response_type_ident);
                response_val_idents.push(response_val_ident);
            }
        }
    };

    // consider whether to add timestamp information to the snapshot
    let (
        snapshot_idents,
        expr_to_current_ts_sec,
        expr_to_gen_snapshot,
        expr_to_log_datum,
        queries_expect_timestamp,
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
            quote! { ic_cdk::println!("ts={}, snapshot={:?}", datum.timestamp, datum.value); },
            generate_queries_without_timestamp(format_ident!("SnapshotValue")),
        )
    } else {
        (
            quote! { type Snapshot = (#(#response_type_idents),*); },
            quote! {},
            quote! { let datum: Snapshot = (#(#response_val_idents),*); },
            quote! { ic_cdk::println!("snapshot={:?}", datum); },
            quote! {},
        )
    };

    Ok(quote! {
        #snapshot_idents

        #queries_expect_timestamp

        ic_solidity_bindgen::contract_abi!(#abi_path);

        async fn execute_task() {
            #expr_to_current_ts_sec
            let res = #contract_struct_ident::new(
                Address::from_str(&get_target_addr()).unwrap(),
                &web3_ctx().unwrap()
            ).#method_ident(#(#request_val_idents,)*None).await.unwrap();
            #expr_to_gen_snapshot
            add_snapshot(datum.clone());
            #expr_to_log_datum
        }

        did_export!(#label);
    })
}

fn match_primitive_type(
    ty: &syn::Type,
    idx: Option<proc_macro2::Literal>,
) -> anyhow::Result<(proc_macro2::Ident, proc_macro2::TokenStream)> {
    let res = match ty {
        syn::Type::Path(type_path) => {
            let mut type_string = quote! { #type_path }.to_string();
            type_string.retain(|c| !c.is_whitespace());

            match type_string.as_str() {
                U256_TYPE => (
                    format_ident!("String"),
                    match idx {
                        Some(idx_lit) => quote! { res.#idx_lit.to_string() },
                        None => quote! { res.to_string() },
                    },
                ),
                ADDRESS_TYPE => (
                    format_ident!("String"),
                    match idx {
                        Some(idx_lit) => quote! { hex::encode(res.#idx_lit) },
                        None => quote! { hex::encode(res) },
                    },
                ),
                _ => (
                    format_ident!("{}", type_string),
                    match idx {
                        Some(idx_lit) => quote! { res.#idx_lit },
                        None => quote! { res },
                    },
                ),
            }
        }
        _ => bail!("Unsupported type"),
    };
    Ok(res)
}

fn common_codes_for_canister() -> proc_macro2::TokenStream {
    let outside_call_idents =
        generate_outside_call_idents(OutsideCallIdentsType::CrossCanisterCall);

    quote! {
        use chainsight_cdk_macros::{manage_single_state, setup_func, manage_vec_state, timer_task_func, cross_canister_call_func, monitoring_canister_metrics, did_export};

        monitoring_canister_metrics!(60);

        #outside_call_idents

        manage_vec_state!("snapshot", Snapshot, true);
        timer_task_func!("set_task", "execute_task", true);
    }
}

fn custom_codes_for_canister(
    manifest: &SnapshotComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;
    let method = &manifest.datasource.method;
    let method_identifier = CanisterMethodIdentifier::parse_from_str(&method.identifier)?;

    let method_ident = &method_identifier.identifier;
    let call_method_ident = format_ident!("call_{}", method_ident);

    // for request values
    // todo: validate length of method.args and method_identifier.params
    let method_args = method
        .args
        .iter()
        .enumerate()
        .map(|(idx, arg)| (method_identifier.params[idx].clone(), arg.clone()))
        .collect();
    let (request_val_idents, request_ty_idents) = generate_request_arg_idents(&method_args);

    // for response type
    let response_type = method_identifier.return_value;
    let (response_type_ident, response_type_def_ident) = match response_type {
        CanisterMethodValueType::Scalar(ty) => {
            let type_ident = format_ident!("{}", &ty);
            (quote! { type SnapshotValue = #type_ident; }, quote! {})
        }
        CanisterMethodValueType::Tuple(tys) => {
            let type_idents = tys
                .iter()
                .map(|ty| format_ident!("{}", ty))
                .collect::<Vec<proc_macro2::Ident>>();
            (
                quote! { type SnapshotValue = (#(#type_idents),*); },
                quote! {},
            )
        }
        CanisterMethodValueType::Struct(values) => {
            let response_type_def_ident = format_ident!("{}", "CustomResponseStruct");
            let struct_tokens = values
                .into_iter()
                .map(|(key, ty)| {
                    let key_ident = format_ident!("{}", key);
                    let ty_ident = format_ident!("{}", ty);
                    quote! {
                        pub #key_ident: #ty_ident
                    }
                })
                .collect::<Vec<_>>();
            (
                quote! { type SnapshotValue = #response_type_def_ident; },
                quote! {
                    #[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
                    pub struct #response_type_def_ident {
                        #(#struct_tokens),*
                    }
                },
            )
        }
    };

    // consider whether to add timestamp information to the snapshot
    let (
        snapshot_idents,
        expr_to_current_ts_sec,
        expr_to_gen_snapshot,
        expr_to_log_datum,
        queries_expect_timestamp,
    ) = if manifest.storage.with_timestamp {
        (
            quote! {
                #[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
                pub struct Snapshot {
                    pub value: SnapshotValue,
                    pub timestamp: u64,
                }
                #response_type_ident
            },
            quote! { let current_ts_sec = ic_cdk::api::time() / 1000000; },
            quote! {
                let datum = Snapshot {
                    value: res.unwrap().clone(),
                    timestamp: current_ts_sec,
                };
            },
            quote! { ic_cdk::println!("ts={}, value={:?}", datum.timestamp, datum.value); },
            generate_queries_without_timestamp(format_ident!("SnapshotValue")),
        )
    } else {
        (
            quote! {
                type Snapshot = SnapshotValue;
                #response_type_ident
            },
            quote! {},
            quote! { let datum = res.unwrap().clone(); },
            quote! { ic_cdk::println!("snapshot={:?}", datum); },
            quote! {},
        )
    };

    Ok(quote! {
        #snapshot_idents

        #queries_expect_timestamp

        #response_type_def_ident

        type CallCanisterArgs = (#(#request_ty_idents),*);
        type CallCanisterResponse = SnapshotValue;
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
    })
}

pub fn generate_codes(
    manifest: &SnapshotComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Snapshot,
        "type is not Snapshot"
    );

    let (common_code_token, custom_code_token) = match manifest.datasource.type_ {
        DatasourceType::Canister => (
            common_codes_for_canister(),
            custom_codes_for_canister(manifest)?,
        ),
        DatasourceType::Contract => (
            common_codes_for_contract(),
            custom_codes_for_contract(manifest)?,
        ),
    };

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &SnapshotComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Snapshot,
        "type is not Snapshot"
    );

    let datasource = &manifest.datasource;
    if datasource.type_ == DatasourceType::Contract {
        ensure!(
            datasource.method.interface.is_some(),
            "datasource.method.interface is required for contract"
        );
    }

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length

    Ok(())
}

fn generate_queries_without_timestamp(return_type: proc_macro2::Ident) -> proc_macro2::TokenStream {
    let query_derives = quote! {
        #[ic_cdk::query]
        #[candid::candid_method(query)]
    };

    quote! {
        #query_derives
        pub fn get_last_snapshot_value() -> #return_type {
            get_last_snapshot().value
        }

        #query_derives
        pub fn get_top_snapshot_values(n: usize) -> Vec<#return_type> {
            get_top_snapshots(n).iter().map(|s| s.value.clone()).collect()
        }

        #query_derives
        pub fn get_snapshot_value(idx: usize) -> #return_type {
            get_snapshot(idx).value
        }
    }
}
