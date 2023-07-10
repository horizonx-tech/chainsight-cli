use anyhow::ensure;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{
    lib::codegen::components::algorithm_indexer::AlgorithmIndexerComponentManifest,
    types::ComponentType,
};

fn common_codes() -> TokenStream {
    quote! {
        use candid::CandidType;
        use chainsight_cdk::{
            indexer::{Event, IndexingConfig},
            storage::{Data, Token},
        };
        use chainsight_cdk_macros::{
            algorithm_indexer, did_export, init_in, manage_single_state, monitoring_canister_metrics,
            setup_func, timer_task_func, Persist
        };
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        monitoring_canister_metrics!(60);
        init_in!();
        manage_single_state!("target_addr", String, false);
        timer_task_func!("set_task", "index", true);

        setup_func!({
            target_addr: String,
            config: IndexingConfig
        });
        use chainsight_cdk::indexer::Indexer;

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
    let output_struct = quote::format_ident!("{}", &manifest.output.name);
    let output_field_idents: Vec<Ident> = manifest
        .output
        .fields
        .iter()
        .map(|(k, _)| quote::format_ident!("{}", k.clone()))
        .collect();

    let output_field_types: Vec<Ident> = manifest
        .output
        .fields
        .iter()
        .map(|(_, v)| quote::format_ident!("{}", v.clone()))
        .collect();

    let out = quote! {
        algorithm_indexer!(#event_struct, #output_struct);

        #[derive(Clone, Debug,  Default, CandidType, Serialize, Deserialize)]
        pub struct #event_struct {
            #(pub #input_field_idents: #input_field_types),*
        }
        #[derive(Clone, Debug,  Default, candid::CandidType, Deserialize, Serialize, chainsight_cdk_macros::Persist)]
        pub struct #output_struct {
            #(pub #output_field_idents: #output_field_types),*
        }
        impl Event<#event_struct> for #output_struct {
            fn from(event: #event_struct) -> Self {
                Self::default()
            }
            fn tokenize(&self) -> chainsight_cdk::storage::Data {
                self._tokenize()
            }

            fn untokenize(data: Data) -> Self {
                #output_struct::_untokenize(data)
            }
        }

        impl Event<#output_struct> for #output_struct {
            fn from(event: #output_struct) -> Self
            where
                #output_struct: Into<Self>,
            {
                event.into()
            }

            fn tokenize(&self) -> chainsight_cdk::storage::Data {
                self._tokenize()
            }

            fn untokenize(data: chainsight_cdk::storage::Data) -> Self {
                #output_struct::_untokenize(data)
            }
        }

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
