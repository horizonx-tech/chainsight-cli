use anyhow::ensure;
use candid::Principal;
use chainsight_cdk::convert::candid::CanisterMethodIdentifier;
use quote::{format_ident, quote};

use crate::{
    lib::codegen::{
        canisters::common::{generate_outside_call_idents, OutsideCallType},
        components::snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
    },
    types::ComponentType,
};

fn common_codes() -> proc_macro2::TokenStream {
    let outside_call_idents = generate_outside_call_idents(&vec![OutsideCallType::Chainsight]);

    quote! {
        use candid::{Decode, Encode};
        use chainsight_cdk_macros::{init_in,manage_single_state, setup_func, prepare_stable_structure, stable_memory_for_vec, StableMemoryStorable, timer_task_func, chainsight_common, did_export, snapshot_icp_source};
        use chainsight_cdk::rpc::{CallProvider, Caller, Message};

        mod types;

        init_in!();
        chainsight_common!(3600);

        #outside_call_idents

        prepare_stable_structure!();
        stable_memory_for_vec!("snapshot", Snapshot, 0, true);
        timer_task_func!("set_task", "execute_task", true);
    }
}

fn custom_codes(
    manifest: &SnapshotIndexerICPComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let SnapshotIndexerICPComponentManifest {
        id,
        datasource,
        storage,
        lens_targets,
        ..
    } = manifest;
    let id = &id.clone().ok_or(anyhow::anyhow!("id is required"))?;
    let method = &datasource.method;
    let method_identifier = CanisterMethodIdentifier::new(&method.identifier)?;

    let bindings_crate_ident = format_ident!("{}", id);
    let method_ident = "proxy_".to_string() + &method_identifier.identifier; // NOTE: to call through proxy

    let response_ty_def_ident = {
        let types_mod_ident = format_ident!("types");
        let response_ty_name_def_ident =
            format_ident!("{}", CanisterMethodIdentifier::RESPONSE_TYPE_NAME);
        quote! { #types_mod_ident::#response_ty_name_def_ident }
    };

    // consider whether to add timestamp information to the snapshot
    let (
        snapshot_idents,
        expr_to_current_ts_sec,
        expr_to_gen_snapshot,
        expr_to_log_datum,
        queries_expect_timestamp,
    ) = if storage.with_timestamp {
        (
            quote! {

                #[derive(Clone, Debug, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable)]
                #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
                pub struct Snapshot {
                    pub value: SnapshotValue,
                    pub timestamp: u64,
                }
                pub type SnapshotValue = #response_ty_def_ident;
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

                pub type SnapshotValue = #response_ty_def_ident;
            },
            quote! {},
            quote! { let datum = Snapshot((res.unwrap().clone())); },
            quote! { ic_cdk::println!("snapshot={:?}", datum); },
            quote! {},
        )
    };

    let call_canister_args_ident = if lens_targets.is_some() {
        let lens_targets: Vec<Principal> = lens_targets
            .clone()
            .map(|t| {
                t.identifiers
                    .iter()
                    .map(|p| Principal::from_text(p).expect("lens target must be principal"))
                    .collect()
            })
            .or_else(|| Some(vec![]))
            .unwrap();

        let lens_targets_string_ident: Vec<_> = lens_targets.iter().map(|p| p.to_text()).collect();

        quote! {
            type CallCanisterArgs = Vec<String>;
            pub fn call_args() -> CallCanisterArgs {
                vec![
                    #(#lens_targets_string_ident.to_string()),*
                ]
            }
        }
    } else {
        quote! {
            type CallCanisterArgs = #bindings_crate_ident::CallCanisterArgs;
            pub fn call_args() -> CallCanisterArgs {
                #bindings_crate_ident::call_args()
            }
        }
    };

    Ok(quote! {
        #snapshot_idents

        #queries_expect_timestamp

        snapshot_icp_source!(#method_ident);

        #call_canister_args_ident
        type CallCanisterResponse = SnapshotValue;

        async fn execute_task() {
            #expr_to_current_ts_sec
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let px = _get_target_proxy(target_canister).await;
            let call_result = CallProvider::new()
                .call(
                    Message::new::<CallCanisterArgs>(
                        call_args(),
                        px.clone(),
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

        did_export!(#id);
    })
}

pub fn generate_codes(
    manifest: &SnapshotIndexerICPComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerICP,
        "type is not SnapshotIndexerICP"
    );

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest)?;

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn generate_app(
    manifest: &SnapshotIndexerICPComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    if manifest.lens_targets.is_some() {
        return Ok(quote! {});
    }

    let method_identifier = CanisterMethodIdentifier::new(&manifest.datasource.method.identifier)?;
    let (args_ty, _) = method_identifier.get_types();
    let code = if args_ty.is_some() {
        let request_args_ty_name_def_ident =
            format_ident!("{}", CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME);

        quote! {
            mod types;

            pub type CallCanisterArgs = types::#request_args_ty_name_def_ident;
            // todo: set return_value by parsing manifest
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
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &SnapshotIndexerICPComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerICP,
        "type is not SnapshotIndexerICP"
    );

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length

    Ok(())
}

pub fn generate_queries_without_timestamp(
    return_type: proc_macro2::Ident,
) -> proc_macro2::TokenStream {
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
        pub async fn proxy_get_last_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProviderWithoutArgs::<#return_type>::new(
                proxy(),
                _get_last_snapshot_value,
            )
            .reply(input)
            .await
        }

        #update_derives
        pub async fn proxy_get_top_snapshot_values(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProvider::<u64, Vec<#return_type>>::new(
                proxy(),
                _get_top_snapshot_values,
            )
            .reply(input)
            .await
        }

        #update_derives
        pub async fn proxy_get_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProvider::<u64, #return_type>::new(
                proxy(),
                _get_snapshot_value,
            )
            .reply(input)
            .await
        }
    }
}
