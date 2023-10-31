use anyhow::ensure;
use chainsight_cdk::config::components::{AlgorithmIndexerConfig, AlgorithmInputType};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    lib::codegen::components::algorithm_indexer::{
        AlgorithmIndexerComponentManifest, AlgorithmOutputType,
    },
    types::ComponentType,
};

// TODO: remove this. see: https://github.com/horizonx-tech/chainsight-sdk/blob/6079cb9ac5f9fb847f383e6f734c8e33c4abc738/chainsight-cdk-macros/src/canisters/algorithm_indexer.rs#L39
fn input_type_ident(manifest: &AlgorithmIndexerComponentManifest) -> TokenStream {
    let event_struct = format_ident!("{}", &manifest.datasource.input.name);
    let source_type = manifest.datasource.source_type.clone();
    match source_type {
        AlgorithmInputType::EventIndexer => {
            // HashMap<u64, Vec<event_struct>>
            let source_ident = format_ident!("{}", &"HashMap".to_string());
            quote! {
                #source_ident<u64, Vec<#event_struct>>
            }
        }
        AlgorithmInputType::KeyValue => {
            // HashMap<String, event_struct>
            let source_ident = format_ident!("{}", &"HashMap".to_string());
            quote! {
                #source_ident<String, #event_struct>
            }
        }
        AlgorithmInputType::KeyValues => {
            // HashMap<String, Vec<event_struct>>
            let source_ident = format_ident!("{}", &"HashMap".to_string());
            quote! {
                #source_ident<String, Vec<#event_struct>>
            }
        }
    }
}
fn custom_codes(
    manifest: &AlgorithmIndexerComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let conf: AlgorithmIndexerConfig = (manifest.clone()).into();
    let conf_json = serde_json::to_string(&conf).unwrap();

    let mut output_structs_quotes = Vec::new();
    let (mut key_value_count, mut key_values_count) = (0, 0);
    for i in 0..manifest.output.len() {
        let output_struct = format_ident!("{}", &manifest.output[i].name.clone());

        let mut output_fields_idents: Vec<Ident> = Vec::new();
        let mut output_field_types: Vec<Ident> = Vec::new();

        let mut keys = manifest.output[i].fields.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            let value = manifest.output[i].fields.get(key).unwrap();
            output_fields_idents.push(format_ident!("{}", key));
            output_field_types.push(format_ident!("{}", value));
        }
        let storage_type = &manifest.output[i].output_type;
        let (storage_ident, idx) = match storage_type {
            AlgorithmOutputType::KeyValue => {
                let storage_ident = format_ident!("{}", &"KeyValueStore".to_string());
                key_value_count += 1;
                (storage_ident, key_value_count)
            }
            _ => {
                let storage_ident = format_ident!("{}", &"KeyValuesStore".to_string());
                key_values_count += 1;
                (storage_ident, key_values_count)
            }
        };

        output_structs_quotes.push(quote! {
            #[derive(Clone, Debug,  Default, CandidType, Deserialize, Serialize, Persist, #storage_ident)]
            #[memory_id(#idx)]
            pub struct #output_struct {
                #(pub #output_fields_idents: #output_field_types),*
            }
        });
    }

    Ok(quote! {
        use chainsight_cdk_macros::def_algorithm_indexer_canister;
        def_algorithm_indexer_canister!(#conf_json);
        #(#output_structs_quotes)*
    })
}

pub fn generate_codes(manifest: &AlgorithmIndexerComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmIndexer,
        "type is not AlgorithmIndexer"
    );
    custom_codes(manifest)
}

pub fn generate_app(manifest: &AlgorithmIndexerComponentManifest) -> anyhow::Result<TokenStream> {
    let input_type = input_type_ident(manifest);
    let event_struct = format_ident!("{}", &manifest.datasource.input.name);

    let event_interfaces = &manifest.datasource.input.fields;
    let mut input_field_idents: Vec<Ident> = event_interfaces
        .iter()
        .map(|(k, _)| format_ident!("{}", k.clone()))
        .collect();
    input_field_idents.sort();
    let mut input_field_types: Vec<Ident> = event_interfaces
        .iter()
        .map(|(_, v)| format_ident!("{}", v.clone()))
        .collect();
    input_field_types.sort();

    let code = quote! {
        use std::collections::HashMap;

        #[derive(Clone, Debug,  Default, candid::CandidType, serde::Serialize, serde::Deserialize)]
        pub struct #event_struct {
            #(pub #input_field_idents: #input_field_types),*
        }

        pub fn persist(elem: #input_type) {
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
