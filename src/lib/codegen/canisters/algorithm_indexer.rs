use anyhow::{ensure, Context};
use chainsight_cdk::config::components::{
    AlgorithmIndexerConfig, AlgorithmInputType, AlgorithmOutputType,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::{
    lib::codegen::components::algorithm_indexer::AlgorithmIndexerComponentManifest,
    types::ComponentType,
};

// TODO: remove this. see: https://github.com/horizonx-tech/chainsight-sdk/blob/7e57e5ce9e32c2c9c029b13c3ebc47309c3bf7ac/chainsight-cdk-macros/src/canisters/algorithm_indexer.rs#L39
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
    let conf_json = serde_json::to_string(&conf)?;

    Ok(quote! {
        use chainsight_cdk::storage::Data;
        use chainsight_cdk_macros::{def_algorithm_indexer_canister, Persist};
        #[warn(unused_imports)]
        use chainsight_cdk_macros::{KeyValueStore, KeyValuesStore};
        def_algorithm_indexer_canister!(#conf_json);
    })
}

pub fn generate_codes(manifest: &AlgorithmIndexerComponentManifest) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::AlgorithmIndexer,
        "type is not AlgorithmIndexer"
    );
    custom_codes(manifest).map(|code| code.to_string())
}

pub fn generate_app(manifest: &AlgorithmIndexerComponentManifest) -> anyhow::Result<String> {
    let input_type = input_type_ident(manifest);
    let event_struct = format_ident!("{}", &manifest.datasource.input.name);

    let event_interfaces = &manifest.datasource.input.fields;
    let input_field_idents: Vec<Ident> = event_interfaces
        .iter()
        .map(|(k, _)| format_ident!("{}", k.clone()))
        .collect();
    let input_field_types: Vec<proc_macro2::TokenStream> = event_interfaces
        .iter()
        .map(|(_, v)| convert_struct_path_to_token_stream(v))
        .collect();

    let mut output_structs_quotes = Vec::new();
    let (mut key_value_count, mut key_values_count) = (0, 0);
    for i in 0..manifest.output.len() {
        let output_struct = format_ident!("{}", &manifest.output[i].name.clone());

        let mut output_fields_idents: Vec<Ident> = Vec::new();
        let mut output_field_types: Vec<proc_macro2::TokenStream> = Vec::new();

        let mut keys = manifest.output[i].fields.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            let value = manifest.output[i]
                .fields
                .get(key)
                .context(format!("output.{}.fields.{} is not set", i, key))?;
            output_fields_idents.push(format_ident!("{}", key));
            output_field_types.push(convert_struct_path_to_token_stream(value));
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
            #[derive(Clone, Debug, Default, candid::CandidType, serde::Deserialize, serde::Serialize, chainsight_cdk_macros::Persist, chainsight_cdk_macros::#storage_ident)]
            #[memory_id(#idx)]
            pub struct #output_struct {
                #(pub #output_fields_idents: #output_field_types),*
            }
        });
    }

    let code = quote! {
        use std::collections::HashMap;
        use chainsight_cdk::storage::Data;

        #[derive(Clone, Debug, Default, candid::CandidType, serde::Serialize, serde::Deserialize)]
        pub struct #event_struct {
            #(pub #input_field_idents: #input_field_types),*
        }

        #(#output_structs_quotes)*

        pub fn persist(elem: #input_type) {
            todo!()
        }
    };

    Ok(code.to_string())
}

fn convert_struct_path_to_token_stream(val: &str) -> proc_macro2::TokenStream {
    let ident_strs: Vec<_> = val.split("::").collect();
    let idents: Vec<Ident> = ident_strs
        .iter()
        .map(|s| Ident::new(s, Span::call_site()))
        .collect();
    quote! { #(#idents)::* }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_struct_path_to_token_stream() {
        assert_eq!(
            convert_struct_path_to_token_stream(&"ethabi::Address").to_string(),
            "ethabi :: Address"
        );

        assert_eq!(
            convert_struct_path_to_token_stream(&"ic_web3_rs::types::U256").to_string(),
            "ic_web3_rs :: types :: U256"
        );
    }
}
