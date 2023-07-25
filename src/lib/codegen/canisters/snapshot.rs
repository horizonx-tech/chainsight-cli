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
        use candid::{Decode, Encode};
        use chainsight_cdk_macros::{init_in, manage_single_state, setup_func, prepare_stable_structure, stable_memory_for_vec, StableMemoryStorable, timer_task_func, define_transform_for_web3, define_web3_ctx, chainsight_common, did_export, snapshot_web3_source};

        use ic_web3_rs::types::Address;
        init_in!();


        chainsight_common!(3600);

        #outside_call_idents

        prepare_stable_structure!();
        stable_memory_for_vec!("snapshot", Snapshot, 0, true);
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
                #[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable)]
                #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
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
            quote! {
                #[derive(Debug, Clone, candid :: CandidType, candid :: Deserialize, serde::Serialize, StableMemoryStorable)]
                #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
                pub struct Snapshot(#(pub #response_type_idents),*);
            },
            quote! {},
            quote! { let datum = Snapshot(#(#response_val_idents),*); },
            quote! { ic_cdk::println!("snapshot={:?}", datum); },
            quote! {},
        )
    };

    Ok(quote! {
        #snapshot_idents

        #queries_expect_timestamp

        ic_solidity_bindgen::contract_abi!(#abi_path);
        snapshot_web3_source!(#method_ident_str);
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
        use candid::{Decode, Encode};
        use chainsight_cdk_macros::{init_in,manage_single_state, setup_func, prepare_stable_structure, stable_memory_for_vec, StableMemoryStorable, timer_task_func, chainsight_common, did_export, snapshot_icp_source};
        use chainsight_cdk::rpc::{CallProvider, Caller, Message};
        init_in!();
        chainsight_common!(3600);

        #outside_call_idents

        prepare_stable_structure!();
        stable_memory_for_vec!("snapshot", Snapshot, 0, true);
        timer_task_func!("set_task", "execute_task", true);
    }
}

fn custom_codes_for_canister(
    manifest: &SnapshotComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;
    let method = &manifest.datasource.method;
    let method_identifier = CanisterMethodIdentifier::parse_from_str(&method.identifier)?;

    let method_ident = "proxy_".to_string() + &method_identifier.identifier; // NOTE: to call through proxy

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
                    #[derive(Clone, Debug, candid::CandidType, serde::Serialize, candid::Deserialize)]
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
                #[derive(Clone, Debug, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable)]
                #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
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
                #[derive(Debug, Clone, candid :: CandidType, candid :: Deserialize, serde::Serialize, StableMemoryStorable)]
                #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
                pub struct Snapshot(pub SnapshotValue);

                #response_type_ident
            },
            quote! {},
            quote! { let datum = Snapshot((res.unwrap().clone())); },
            quote! { ic_cdk::println!("snapshot={:?}", datum); },
            quote! {},
        )
    };

    Ok(quote! {
        #snapshot_idents

        #queries_expect_timestamp

        #response_type_def_ident

        snapshot_icp_source!(#method_ident);

        type CallCanisterArgs = (#(#request_ty_idents),*);
        type CallCanisterResponse = SnapshotValue;
        async fn execute_task() {
            #expr_to_current_ts_sec
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let call_result = CallProvider::new(proxy())
                .call(
                    Message::new::<CallCanisterArgs>(
                        (#(#request_val_idents),*),
                        target_canister.clone(),
                        #method_ident
                    ).unwrap()
                ).await;
            if let Err(err) = call_result {
                ic_cdk::println!("error: {:?}", err);
                return;
            }
            let res = call_result.unwrap().reply::<CallCanisterResponse>();
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
    let update_derives = quote! {
        #[ic_cdk::update]
        #[candid::candid_method(update)]
    };

    quote! {
        fn _get_last_snapshot_value() -> #return_type {
            get_last_snapshot().value
        }

        fn _get_top_snapshot_values(n: u64) -> Vec<#return_type> {
            get_top_snapshots(n).iter().map(|s| s.value.clone()).collect()
        }

        fn _get_snapshot_value(idx: u64) -> #return_type {
            get_snapshot(idx).value
        }

        #query_derives
        pub fn get_last_snapshot_value() -> #return_type {
            _get_last_snapshot_value()
        }

        #query_derives
        pub fn get_top_snapshot_values(n: u64) -> Vec<#return_type> {
            _get_top_snapshot_values(n)
        }

        #query_derives
        pub fn get_snapshot_value(idx: u64) -> #return_type {
            _get_snapshot_value(idx)
        }

        #update_derives
        pub fn proxy_get_last_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProviderWithoutArgs::<#return_type>::new(
                proxy(),
                _get_last_snapshot_value,
            )
            .reply(input)
        }

        #update_derives
        pub fn proxy_get_top_snapshot_values(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProvider::<u64, Vec<#return_type>>::new(
                proxy(),
                _get_top_snapshot_values,
            )
            .reply(input)
        }

        #update_derives
        pub fn proxy_get_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProvider::<u64, #return_type>::new(
                proxy(),
                _get_snapshot_value,
            )
            .reply(input)
        }
    }
}
