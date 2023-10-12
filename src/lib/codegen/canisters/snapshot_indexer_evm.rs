use anyhow::{bail, ensure};
use quote::{format_ident, quote};

use crate::{
    lib::{
        codegen::{
            canisters::{
                common::{
                    generate_outside_call_idents, generate_request_arg_idents,
                    ContractMethodIdentifier, OutsideCallType,
                },
                snapshot_indexer_icp::generate_queries_without_timestamp,
            },
            components::{
                common::ComponentManifest,
                snapshot_indexer_evm::SnapshotIndexerEVMComponentManifest,
            },
        },
        utils::{convert_camel_to_snake, ADDRESS_TYPE, U256_TYPE},
    },
    types::ComponentType,
};

fn common_codes() -> proc_macro2::TokenStream {
    let outside_call_idents = generate_outside_call_idents(&vec![OutsideCallType::Evm]);

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

fn custom_codes(
    manifest: &SnapshotIndexerEVMComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let id = &manifest.id().ok_or(anyhow::anyhow!("id is required"))?;
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
        "datatource.method is not valid: The number of params in 'identifier' and 'args' must be the same"
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
        0 => bail!("datatource.method.identifier is not valid: Response required"),
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

        did_export!(#id);
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

pub fn generate_codes(
    manifest: &SnapshotIndexerEVMComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerEVM,
        "type is not SnapshotIndexerEVM"
    );

    // let (common_code_token, custom_code_token) = match manifest.datasource.type_ {
    //     DatasourceType::Canister => (
    //         common_codes_for_canister(),
    //         custom_codes_for_canister(manifest)?,
    //     ),
    //     DatasourceType::Contract => (
    //         common_codes_for_contract(),
    //         custom_codes_for_contract(manifest)?,
    //     ),
    // };
    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest)?;

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn generate_app(
    _manifest: &SnapshotIndexerEVMComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    Ok(quote! {})
}

pub fn validate_manifest(manifest: &SnapshotIndexerEVMComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerEVM,
        "type is not SnapshotIndexerEVM"
    );

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length

    Ok(())
}