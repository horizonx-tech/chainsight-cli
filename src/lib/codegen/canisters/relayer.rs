use anyhow::ensure;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::{types::ComponentType, lib::codegen::{components::{relayer::RelayerComponentManifest, common::{DestinactionType}}, oracle::get_oracle_attributes, canisters::common::{generate_custom_struct_idents, generate_custom_type_idents, generate_request_arg_idents, generate_outside_call_idents, OutsideCallIdentsType}}};

// temp
fn common_codes() -> TokenStream {
    let outside_call_idents = generate_outside_call_idents(OutsideCallIdentsType::All);
    quote! {
        use std::str::FromStr;
        use chainsight_cdk_macros::{manage_single_state, setup_func, timer_task_func, cross_canister_call_func, define_web3_ctx, define_transform_for_web3, define_get_ethereum_address, monitoring_canister_metrics, did_export};
        use ic_web3::types::{Address, U256};

        monitoring_canister_metrics!(60);

        #outside_call_idents

        define_get_ethereum_address!();

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
    let response_type_ident = format_ident!("{}", &method.response.type_);

    // for request values
    let (request_val_idents, request_ty_idents) = generate_request_arg_idents(&method.args);

    // define custom_struct
    let custom_struct_ident = match &method.custom_struct {
        Some(custom_structs) => generate_custom_struct_idents(custom_structs),
        None => vec![]
    };

    // define custom_type
    let custom_type_ident = match &method.custom_type {
        Some(custom_types) => generate_custom_type_idents(custom_types),
        None => vec![]
    };

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

        type CallCanisterArgs = (#(#request_ty_idents),*);
        type CallCanisterResponse = (#response_type_ident);
        cross_canister_call_func!(#method_ident, CallCanisterArgs, CallCanisterResponse);

        async fn sync() {
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let res = #call_method_ident(
                target_canister,
                (#(#request_val_idents),*)
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
