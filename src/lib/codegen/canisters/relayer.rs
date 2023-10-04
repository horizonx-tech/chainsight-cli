use anyhow::{bail, ensure};
use candid::Principal;
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
        use chainsight_cdk_macros::{manage_single_state, setup_func, init_in, timer_task_func, define_web3_ctx, define_transform_for_web3, define_get_ethereum_address, chainsight_common, did_export,relayer_source};
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

    let label_ident = format_ident!("{}", &manifest.metadata.label);
    let method_ident = "proxy_".to_string() + &method_identifier.identifier;
    let method_ident_origin = &method_identifier.identifier;

    // from destination: about oracle
    let destination = &manifest.destination;
    let (oracle_name_str, _, _) = get_oracle_attributes(&destination.type_);
    let oracle_ident = format_ident!("{}", oracle_name_str);
    let abi_path = format!("./__interfaces/{}.json", oracle_name_str);

    // for response type
    let response_type: CanisterMethodValueType = method_identifier.return_value;
    let sync_data_ident = generate_ident_sync_to_oracle(response_type, destination.type_)?;
    let args_type_ident = match manifest.lens_targets.is_some() {
        true => quote! {
            type CallCanisterArgs = Vec<String>;
        },
        false => quote! {
            type CallCanisterArgs = #label_ident::CallCanisterArgs;
        },
    };
    let lens_targets: Vec<Principal> = manifest
        .clone()
        .lens_targets
        .map(|t| {
            t.identifiers
                .iter()
                .map(|p| Principal::from_text(p).expect("lens target must be principal"))
                .collect()
        })
        .or_else(|| Some(vec![]))
        .unwrap();
    let lens_targets_string_ident: Vec<_> = lens_targets.iter().map(|p| p.to_text()).collect();

    let get_args_ident = match manifest.lens_targets.is_some() {
        true => quote! {
            pub fn call_args() -> Vec<String> {
                vec![
                    #(#lens_targets_string_ident.to_string()),*
                ]
            }
        },
        false => quote! {
            pub fn call_args() -> CallCanisterArgs {
                #label_ident::call_args()
            }
        },
    };

    let relayer_source_ident = match manifest.lens_targets.is_some() {
        true => quote! {
            relayer_source!(#method_ident_origin, true);
        },
        false => quote! {
            relayer_source!(#method_ident_origin, false);
        },
    };

    Ok(quote! {
        ic_solidity_bindgen::contract_abi!(#abi_path);
        use #label_ident::*;
        #relayer_source_ident
        #args_type_ident
        #get_args_ident


        async fn sync() {
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let call_result = CallProvider::new()
            .call(Message::new::<CallCanisterArgs>(call_args(), _get_target_proxy(target_canister.clone()).await, #method_ident).unwrap())
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
            if !filter(&datum) {
                return;
            }

            #oracle_ident::new(
                Address::from_str(&get_target_addr()).unwrap(),
                &web3_ctx().unwrap()
            ).update_state(#sync_data_ident, None).await.unwrap();
            ic_cdk::println!("value_to_sync={:?}", datum);
        }

        did_export!(#label);
    })
}

fn generate_ident_sync_to_oracle(
    canister_response_type: CanisterMethodValueType,
    oracle_type: DestinationType,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let res = match canister_response_type {
        CanisterMethodValueType::Scalar(ty, _) => {
            let arg_ident = format_ident!("datum");
            match oracle_type {
                DestinationType::Uint256Oracle => {
                    generate_quote_to_convert_datum_to_u256(arg_ident, &ty)?
                }
                DestinationType::Uint128Oracle => {
                    generate_quote_to_convert_datum_to_integer(arg_ident, &ty, "u128")?
                }
                DestinationType::Uint64Oracle => {
                    generate_quote_to_convert_datum_to_integer(arg_ident, &ty, "u64")?
                }
                DestinationType::StringOracle => quote! { datum.clone().to_string() },
            }
        }
        CanisterMethodValueType::Tuple(_) => {
            match oracle_type {
                DestinationType::StringOracle => {
                    quote! { format!("{:?}", &datum) } // temp
                }
                _ => bail!("not support tuple type for oracle"),
            }
        }
        CanisterMethodValueType::Struct(_) => {
            match oracle_type {
                DestinationType::StringOracle => {
                    quote! { format!("{:?}", &datum) } // temp
                }
                _ => bail!("not support struct type for oracle"),
            }
        }
        CanisterMethodValueType::Vector(_, _) => {
            match oracle_type {
                DestinationType::StringOracle => {
                    quote! { format!("{:?}", &datum) } // temp
                }
                _ => bail!("not support vec type for oracle"),
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

pub fn generate_app(
    manifest: &RelayerComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    let method = &manifest.datasource.method;
    let method_identifier = CanisterMethodIdentifier::parse_from_str(&method.identifier)?;
    let oracle_type = &manifest.destination.type_;

    let response_type_ident = match method_identifier.return_value {
        CanisterMethodValueType::Scalar(ty, _) => {
            let ty_ident = format_ident!("{}", ty);
            quote! { pub type CallCanisterResponse = #ty_ident }
        }
        CanisterMethodValueType::Tuple(tys) => match oracle_type {
            DestinationType::StringOracle => {
                let type_idents = tys
                    .iter()
                    .map(|(ty, _)| format_ident!("{}", ty))
                    .collect::<Vec<proc_macro2::Ident>>();
                quote! { pub type CallCanisterResponse = (#(#type_idents),*) }
            }
            _ => bail!("not support tuple type for oracle"),
        },
        CanisterMethodValueType::Struct(values) => match oracle_type {
            DestinationType::StringOracle => {
                let response_type_def_ident = format_ident!("{}", "CustomResponseStruct");
                let struct_tokens = values
                    .into_iter()
                    .map(|(key, ty, _)| {
                        let key_ident = format_ident!("{}", key);
                        let ty_ident = format_ident!("{}", ty);
                        quote! {
                            pub #key_ident: #ty_ident
                        }
                    })
                    .collect::<Vec<_>>();
                quote! {
                pub type CallCanisterResponse = #response_type_def_ident;
                   #[derive(Clone, Debug, candid::CandidType, candid::Deserialize)]
                   pub struct #response_type_def_ident {
                       #(#struct_tokens),*
                   }
                }
            }
            _ => bail!("not support struct type for oracle"),
        },
        _ => bail!("not support vector type for oracle"),
    };

    let args_quote = match &manifest.lens_targets.is_some() {
        true => quote! {},
        false => {
            let method_args = method
                .args
                .iter()
                .enumerate()
                .map(|(idx, arg)| (method_identifier.params[idx].clone(), arg.clone()))
                .collect();
            let (request_val_idents, request_type_idents) =
                generate_request_arg_idents(&method_args);

            quote! {
                pub type CallCanisterArgs = (#(#request_type_idents),*);
                pub fn call_args() -> CallCanisterArgs {
                    (#(#request_val_idents),*)
                }
            }
        }
    };

    Ok(quote! {
        #response_type_ident;
        #args_quote
        pub fn filter(_: &CallCanisterResponse) -> bool {
            true
        }
    })
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
