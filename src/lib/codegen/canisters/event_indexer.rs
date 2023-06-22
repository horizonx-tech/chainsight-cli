use anyhow::ensure;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use crate::{lib::{codegen::{components::event_indexer::EventIndexerComponentManifest, canisters::common::{convert_type_from_ethabi_param_type, generate_outside_call_idents, OutsideCallIdentsType}}, utils::{ADDRESS_TYPE, U256_TYPE}}, types::ComponentType};

fn common_codes() -> TokenStream {
    let outside_call_idents = generate_outside_call_idents(OutsideCallIdentsType::Eth);

    // todo
    quote! {
        use std::str::FromStr;
        use ic_web3_rs::types::Address;
        use chainsight_cdk_macros::{
            define_transform_for_web3, define_web3_ctx, did_export,
            manage_single_state, monitoring_canister_metrics, setup_func, ContractEvent
        };

        monitoring_canister_metrics!(60);

        #outside_call_idents
    }
}

fn custom_codes(manifest: &EventIndexerComponentManifest, interface_contract: ethabi::Contract) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.label;
    let datasource_event_def = &manifest.datasource.event;

    let event_interface = &manifest.datasource.event.interface.clone()
        .ok_or(anyhow::anyhow!("datasource.method.interface is required for contract"))?;
    let abi_path = format!("./__interfaces/{}", event_interface);

    let events = interface_contract.events_by_name(&datasource_event_def.identifier)?;
    ensure!(events.len() == 1, "event is not found or there are multiple events");
    let event = events.first().unwrap();

    let event_struct_name = format_ident!("{}", &event.name);
    let event_struct_field_tokens = event.inputs.clone().into_iter().map(|event| {
        let field_name_ident = format_ident!("{}", event.name);
        let field_ty = convert_type_from_ethabi_param_type(event.kind).unwrap();
        let field_ty_ident = if field_ty == ADDRESS_TYPE || field_ty == U256_TYPE {
            format_ident!("String")
        } else {
            format_ident!("{}", field_ty)
        }; // todo: refactor
        quote! { pub #field_name_ident: #field_ty_ident }
    }).collect::<Vec<_>>();
    let event_struct = quote! {
        #[derive(Clone, Debug,  Default, candid::CandidType, candid::Deserialize, ContractEvent)]
        pub struct #event_struct_name {
            #(#event_struct_field_tokens),*
        }
    };

    // temp
    Ok(quote! {
        ic_solidity_bindgen::contract_abi!(#abi_path);

        #event_struct

        did_export!(#label);
    })
}

pub fn generate_codes(manifest: &EventIndexerComponentManifest, interface_contract: ethabi::Contract) -> anyhow::Result<TokenStream> {
    ensure!(manifest.type_ == ComponentType::EventIndexer, "type is not EventIndexer");

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest, interface_contract)?;

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &EventIndexerComponentManifest) -> anyhow::Result<()> {
    ensure!(manifest.type_ == ComponentType::EventIndexer, "type is not EventIndexer");

    ensure!(manifest.datasource.event.interface.is_some(), "datasource.event.interface is not set");

    Ok(())
}
