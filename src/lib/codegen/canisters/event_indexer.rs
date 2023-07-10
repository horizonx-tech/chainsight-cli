use anyhow::ensure;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    lib::{
        codegen::{
            canisters::common::convert_type_from_ethabi_param_type,
            components::event_indexer::EventIndexerComponentManifest,
        },
        utils::{convert_camel_to_snake, ADDRESS_TYPE, U256_TYPE},
    },
    types::ComponentType,
};

fn common_codes() -> TokenStream {
    quote! {
        use candid::CandidType;
        use chainsight_cdk::{
            indexer::{Event, Indexer, IndexingConfig},
            storage::Data,
            web3::Web3CtxParam,
        };
        use chainsight_cdk_macros::{
            define_get_ethereum_address, define_transform_for_web3, define_web3_ctx, did_export, init_in,
            manage_single_state, monitoring_canister_metrics, setup_func, web3_event_indexer,timer_task_func,
            ContractEvent, Persist,
        };
        use ic_solidity_bindgen::{types::EventLog};
        use ic_web3_rs::{
            ethabi::Address,
            futures::{future::BoxFuture, FutureExt},
            transports::ic_http_client::CallOptions,
        };
        use serde::Serialize;
        use std::{collections::HashMap, str::FromStr};
        monitoring_canister_metrics!(60);
        define_web3_ctx!();
        define_transform_for_web3!();
        define_get_ethereum_address!();
        timer_task_func!("set_task", "index", true);
        manage_single_state!("target_addr", String, false);
        setup_func!({
            target_addr: String,
            web3_ctx_param: Web3CtxParam,
            config: IndexingConfig,
        });
        init_in!();

    }
}

fn custom_codes(
    manifest: &EventIndexerComponentManifest,
    interface_contract: ethabi::Contract,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;
    let datasource_event_def = &manifest.datasource.event;

    let event_interface = &manifest
        .datasource
        .event
        .interface
        .clone()
        .ok_or(anyhow::anyhow!(
            "datasource.method.interface is required for contract"
        ))?;
    let contract_struct_ident = format_ident!("{}", event_interface.trim_end_matches(".json")); // temp: specify the extension (only .json?)
    let abi_path = format!("./__interfaces/{}", event_interface);

    let events = interface_contract.events_by_name(&datasource_event_def.identifier)?;
    ensure!(
        events.len() == 1,
        "event is not found or there are multiple events"
    );
    let event = events.first().unwrap();

    let event_struct_name = format_ident!("{}", &event.name);
    let event_struct_field_tokens = event
        .inputs
        .clone()
        .into_iter()
        .map(|event| {
            let field_name_ident = format_ident!("{}", event.name);
            let field_ty = convert_type_from_ethabi_param_type(event.kind).unwrap();
            let field_ty_ident = if field_ty == ADDRESS_TYPE || field_ty == U256_TYPE {
                format_ident!("String")
            } else {
                format_ident!("{}", field_ty)
            }; // todo: refactor
            quote! { pub #field_name_ident: #field_ty_ident }
        })
        .collect::<Vec<_>>();
    let event_struct = quote! {
        #[derive(Clone, Debug,  Default, candid::CandidType, ContractEvent, Serialize, Persist)]
        pub struct #event_struct_name {
            #(#event_struct_field_tokens),*
        }

        impl chainsight_cdk::indexer::Event<EventLog> for #event_struct_name {
            fn from(event: EventLog) -> Self
            where
                EventLog: Into<Self>,
            {
                event.into()
            }

            fn tokenize(&self) -> chainsight_cdk::storage::Data {
                self._tokenize()
            }

            // temp
            fn untokenize(data: chainsight_cdk::storage::Data) -> Self {
                #event_struct_name::_untokenize(data)
            }
        }
    };

    let call_func_ident = format_ident!("event_{}", convert_camel_to_snake(&event.name));

    // temp
    Ok(quote! {
        ic_solidity_bindgen::contract_abi!(#abi_path);
        web3_event_indexer!(#event_struct_name);
        #event_struct

        fn get_logs(
            from: u64,
            to: u64,
            call_options: CallOptions,
        ) -> BoxFuture<'static, Result<HashMap<u64, Vec<EventLog>>, chainsight_cdk::indexer::Error>> {
            async move {
                let res = #contract_struct_ident::new(
                    Address::from_str(get_target_addr().as_str()).unwrap(),
                    &web3_ctx().unwrap()
                ).#call_func_ident(from, to, call_options).await;
                match res {
                    Ok(logs) => Ok(logs),
                    Err(e) => Err(chainsight_cdk::indexer::Error::OtherError(e.to_string())),
                }
            }.boxed()
        }

        did_export!(#label);
    })
}

pub fn generate_codes(
    manifest: &EventIndexerComponentManifest,
    interface_contract: ethabi::Contract,
) -> anyhow::Result<TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::EventIndexer,
        "type is not EventIndexer"
    );

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest, interface_contract)?;

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &EventIndexerComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::EventIndexer,
        "type is not EventIndexer"
    );

    ensure!(
        manifest.datasource.event.interface.is_some(),
        "datasource.event.interface is not set"
    );

    Ok(())
}
