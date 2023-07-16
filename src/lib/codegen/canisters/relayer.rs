use anyhow::{bail, ensure};
use quote::{format_ident, quote};

use crate::{
    lib::codegen::{
        canisters::common::{
            generate_outside_call_idents, generate_request_arg_idents, OutsideCallIdentsType,
        },
        components::{common::DestinationType, relayer::RelayerComponentManifest},
        oracle::get_oracle_attributes,
    },
    types::ComponentType,
};

use super::common::{CanisterMethodIdentifier, CanisterMethodValueType};

fn common_codes() -> proc_macro2::TokenStream {
    let outside_call_idents = generate_outside_call_idents(OutsideCallIdentsType::All);
    quote! {
        use std::str::FromStr;
        use chainsight_cdk_macros::{manage_single_state, setup_func, init_in, timer_task_func, define_web3_ctx, define_transform_for_web3, define_get_ethereum_address, chainsight_common, did_export};
        use ic_web3_rs::types::{Address, U256};
        use chainsight_cdk::rpc::{CallProvider, Caller, Message};

        chainsight_common!(3600);

        #outside_call_idents

        define_get_ethereum_address!();

        timer_task_func!("set_task", "sync", true);
        init_in!();
    }
}

fn custom_codes(manifest: &RelayerComponentManifest) -> anyhow::Result<proc_macro2::TokenStream> {
    let label = &manifest.metadata.label;
    let method = &manifest.datasource.method;
    let method_identifier = CanisterMethodIdentifier::parse_from_str(&method.identifier)?;

    let method_ident = "proxy_".to_string() + &method_identifier.identifier;

    // from destination: about oracle
    let destination = &manifest.destination;
    let (oracle_name_str, _, _) = get_oracle_attributes(&destination.type_);
    let oracle_ident = format_ident!("{}", oracle_name_str);
    let abi_path = format!("./__interfaces/{}.json", oracle_name_str);

    // for request values
    // todo: validate length of method.args and method_identifier.params
    let method_args = method
        .args
        .iter()
        .enumerate()
        .map(|(idx, arg)| (method_identifier.params[idx].clone(), arg.clone()))
        .collect();
    let (request_val_idents, request_ty_idents) = generate_request_arg_idents(&method_args);

    // for response type
    let response_type: CanisterMethodValueType = method_identifier.return_value;
    let (call_canister_response_type_ident, response_type_def_ident, sync_data_ident) =
        generate_idents_to_call_datasource_and_sync_to_oracle(response_type, destination.type_)?;

    Ok(quote! {
        ic_solidity_bindgen::contract_abi!(#abi_path);

        #response_type_def_ident

        type CallCanisterArgs = (#(#request_ty_idents),*);
        #call_canister_response_type_ident

        async fn sync() {
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let call_result = CallProvider::new(proxy())
            .call(Message::new::<CallCanisterArgs>((#(#request_val_idents),*), target_canister.clone(), #method_ident).unwrap())
            .await;
            if let Err(err) = call_result {
                ic_cdk::println!("error: {:?}", err);
                return;
            }
            let val = call_result.unwrap().reply::<CallCanisterResponse>();
            if let Err(err) = val {
                ic_cdk::println!("error: {:?}", err);
                return;
            }
            let datum = val.unwrap();

            #oracle_ident::new(
                Address::from_str(&get_target_addr()).unwrap(),
                &web3_ctx().unwrap()
            ).update_state(#sync_data_ident, None).await.unwrap();
            ic_cdk::println!("value_to_sync={:?}", datum);
        }

        did_export!(#label);
    })
}

fn generate_idents_to_call_datasource_and_sync_to_oracle(
    canister_response_type: CanisterMethodValueType,
    oracle_type: DestinationType,
) -> anyhow::Result<(
    proc_macro2::TokenStream, // call_canister_response_type_ident
    proc_macro2::TokenStream, // response_type_def_ident
    proc_macro2::TokenStream, // sync_data_ident
)> {
    let res = match canister_response_type {
        CanisterMethodValueType::Scalar(ty) => {
            let ty_ident = format_ident!("{}", ty);
            let call_canister_response_type_ident =
                quote! { type CallCanisterResponse = #ty_ident; };
            let arg_ident = format_ident!("datum");
            match oracle_type {
                DestinationType::Uint256Oracle => {
                    let quote_to_convert_datum_to_u256 =
                        generate_quote_to_convert_datum_to_u256(arg_ident, &ty)?;
                    (
                        call_canister_response_type_ident,
                        quote! {},
                        quote_to_convert_datum_to_u256,
                    )
                }
                DestinationType::Uint128Oracle => {
                    let quote_to_convert_datum =
                        generate_quote_to_convert_datum_to_integer(arg_ident, &ty, "u128")?;
                    (
                        call_canister_response_type_ident,
                        quote! {},
                        quote_to_convert_datum,
                    )
                }
                DestinationType::Uint64Oracle => {
                    let quote_to_convert_datum =
                        generate_quote_to_convert_datum_to_integer(arg_ident, &ty, "u64")?;
                    (
                        call_canister_response_type_ident,
                        quote! {},
                        quote_to_convert_datum,
                    )
                }
                DestinationType::StringOracle => (
                    call_canister_response_type_ident,
                    quote! {},
                    quote! { datum.clone().to_string() },
                ),
            }
        }
        CanisterMethodValueType::Tuple(tys) => {
            match oracle_type {
                DestinationType::StringOracle => {
                    let type_idents = tys
                        .iter()
                        .map(|ty| format_ident!("{}", ty))
                        .collect::<Vec<proc_macro2::Ident>>();
                    (
                        quote! { type CallCanisterResponse = (#(#type_idents),*); },
                        quote! {},
                        quote! { format!("{:?}", &datum) }, // temp
                    )
                }
                _ => bail!("not support tuple type for oracle"),
            }
        }
        CanisterMethodValueType::Struct(values) => {
            match oracle_type {
                DestinationType::StringOracle => {
                    let response_type_def_ident = format_ident!("{}", "CustomResponseStruct");
                    let struct_tokens = values
                        .into_iter()
                        .map(|(key, ty)| {
                            let key_ident = format_ident!("{}", key);
                            let ty_ident = format_ident!("{}", ty);
                            quote! {
                                pub #key_ident: #ty_ident
                            }
                        })
                        .collect::<Vec<_>>();
                    (
                        quote! { type CallCanisterResponse = #response_type_def_ident; },
                        quote! {
                            #[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
                            pub struct #response_type_def_ident {
                                #(#struct_tokens),*
                            }
                        },
                        quote! { format!("{:?}", &datum) }, // temp
                    )
                }
                _ => bail!("not support struct type for oracle"),
            }
        }
    };
    anyhow::Ok(res)
}

fn generate_quote_to_convert_datum_to_u256(
    arg_ident: proc_macro2::Ident,
    datum_scalar_type: &str,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let res = match datum_scalar_type {
        "u8" | "u16" | "u32" | "u64" | "u128" | "U256" | "chainsight_cdk::core::U256" => {
            quote! { U256::from(#arg_ident) }
        }
        "i8" | "i16" | "i32" | "i64" | "i128" => quote! { U256::from(#arg_ident) }, // NOTE: a positive value check needs to be performed on the generated code
        "String" => quote! { U256::from_dec_str(&#arg_ident).unwrap() },
        _ => bail!("This type cannot be converted to U256"),
    };
    Ok(res)
}

fn generate_quote_to_convert_datum_to_integer(
    arg_ident: proc_macro2::Ident,
    datum_scalar_type: &str,
    converted_datum_type: &str,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let converted_datum_type_ident = format_ident!("{}", converted_datum_type);
    let res = match datum_scalar_type {
        "u8" | "u16" | "u32" | "u64" | "u128" => {
            quote! { #arg_ident as #converted_datum_type_ident }
        }
        "i8" | "i16" | "i32" | "i64" | "i128" => {
            quote! { #arg_ident as #converted_datum_type_ident }
        } // NOTE: a positive value check needs to be performed on the generated code
        "String" => quote! { #converted_datum_type_ident::from_str(&#arg_ident).unwrap() },
        _ => bail!(format!(
            "This type cannot be converted to {}",
            converted_datum_type
        )),
    };
    Ok(res)
}

pub fn generate_codes(
    manifest: &RelayerComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Relayer,
        "type is not Relayer"
    );

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest)?;

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &RelayerComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Relayer,
        "type is not Relayer"
    );

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length
    // - check destination.type

    Ok(())
}
