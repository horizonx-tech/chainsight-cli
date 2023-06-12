use std::fs::File;

use anyhow::ensure;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use crate::{lib::{codegen::{components::event_indexer::EventIndexerComponentManifest, canisters::common::convert_type_from_ethabi_param_type}, utils::{ADDRESS_TYPE, U256_TYPE}}, types::ComponentType};

fn common_codes() -> TokenStream {
    // todo
    quote! {
        monitoring_canister_metrics!(60);
    }
}

fn custom_codes(manifest: &EventIndexerComponentManifest) -> anyhow::Result<proc_macro2::TokenStream> {
    let datasource_event_def = &manifest.datasource.event;

    let abi_file = File::open(format!("./sample_pj/interfaces/{}", &datasource_event_def.interface.as_ref().unwrap()))?; // todo: pass path as args? field?
    // let abi_file = File::open(format!("./sample_pj/artifacts/__interfaces/{}", &datasource_event_def.interface.as_ref().unwrap()))?;
    let contract = ethabi::Contract::load(abi_file)?;
    let events = contract.events_by_name(&datasource_event_def.identifier)?;
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
        #[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
        pub struct #event_struct_name {
            #(#event_struct_field_tokens),*
        }
    };

    // temp: only Event struct
    Ok(quote! {
        #event_struct
    })
}

pub fn generate_codes(manifest: &EventIndexerComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(manifest.type_ == ComponentType::EventIndexer, "type is not EventIndexer");

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest)?;

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
