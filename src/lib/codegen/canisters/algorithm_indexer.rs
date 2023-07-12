use anyhow::ensure;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{
    lib::codegen::components::algorithm_indexer::{
        AlgorithmIndexerComponentManifest, AlgorithmOutputType,
    },
    types::ComponentType,
};

fn common_codes() -> TokenStream {
    quote! {
        use candid::CandidType;
        use chainsight_cdk::{indexer::IndexingConfig, storage::Data};
        use chainsight_cdk_macros::{
            algorithm_indexer, did_export, init_in, manage_single_state, monitoring_canister_metrics,
            setup_func, timer_task_func, KeyValueStore, KeyValuesStore, Persist,
        };
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        monitoring_canister_metrics!(60);
        init_in!();
        manage_single_state!("target_addr", String, false);

        setup_func!({
            target_addr: String,
            config: IndexingConfig
        });
        timer_task_func!("set_task", "index", true);


    }
}

fn custom_codes(
    manifest: &AlgorithmIndexerComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;

    let event_interfaces = &manifest.datasource.input.fields;

    let event_struct = quote::format_ident!("{}", &manifest.datasource.input.name);
    let input_field_idents: Vec<Ident> = event_interfaces
        .iter()
        .map(|(k, _)| quote::format_ident!("{}", k.clone()))
        .collect();

    let input_field_types: Vec<Ident> = event_interfaces
        .iter()
        .map(|(_, v)| quote::format_ident!("{}", v.clone()))
        .collect();

    let mut output_structs_quotes = Vec::new();
    let mut key_value_count = 1;
    let mut key_values_count = 1;
    for i in 0..manifest.output.len() {
        let output_struct = quote::format_ident!("{}", &manifest.output[i].name.clone());

        let output_fields_idents: Vec<Ident> = manifest.output[i]
            .fields
            .iter()
            .map(|(k, _)| quote::format_ident!("{}", k.clone()))
            .collect();
        let output_field_types: Vec<Ident> = manifest.output[i]
            .fields
            .iter()
            .map(|(_, v)| quote::format_ident!("{}", v.clone()))
            .collect();
        let storage_type = &manifest.output[i].output_type;
        let (storage_ident, idx) = match storage_type {
            AlgorithmOutputType::KeyValue => {
                let ident = quote::format_ident!("chainsight_cdk_macros::KeyValueStore");
                key_value_count += 1;
                (ident, key_value_count)
            }
            _ => {
                let ident = quote::format_ident!("chainsight_cdk_macros::KeyValuesStore");
                key_values_count += 1;
                (ident, key_values_count)
            }
        };

        output_structs_quotes.push(quote! {
            #[derive(Clone, Debug,  Default, CandidType, Deserialize, Serialize, chainsight_cdk_macros::Persist,#storage_ident)]
            #[memory_id(#idx)]
            pub struct #output_struct {
                #(pub #output_fields_idents: #output_field_types),*
            }
        });
    }
    let out = quote! {
        algorithm_indexer!(#event_struct);

        #[derive(Clone, Debug,  Default, CandidType, Serialize, Deserialize)]
        pub struct #event_struct {
            #(pub #input_field_idents: #input_field_types),*
        }
        #(#output_structs_quotes)*

    };

    // temp
    Ok(quote! {
        #out
        did_export!(#label);
    })
}

pub fn generate_codes(manifest: &AlgorithmIndexerComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmIndexer,
        "type is not EventIndexer"
    );

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest)?;

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn generate_app(manifest: &AlgorithmIndexerComponentManifest) -> anyhow::Result<TokenStream> {
    let event_struct = quote::format_ident!("{}", &manifest.datasource.input.name);

    let code = quote! {
        use std::collections::HashMap;
        use crate::#event_struct;
        pub fn persist(elem: HashMap<u64, Vec<#event_struct>>) {
            todo!()
        }
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &AlgorithmIndexerComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmIndexer,
        "type is not AlgorithmIndexer"
    );

    ensure!(
        !manifest.datasource.input.fields.is_empty(),
        "datasource.event.interface is not set"
    );

    Ok(())
}
