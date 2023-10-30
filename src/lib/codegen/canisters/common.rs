use std::vec;

use anyhow::bail;
use ethabi::{param_type::Reader, ParamType};
use quote::{format_ident, quote};
use regex::Regex;

use crate::lib::utils::{ADDRESS_TYPE, U256_TYPE};

/// Generate method identifiers from function expressions in abi format
#[derive(Debug, Clone, PartialEq)]
pub struct ContractMethodIdentifier {
    pub identifier: String,
    pub params: Vec<String>,
    pub return_value: Vec<String>,
}
impl ContractMethodIdentifier {
    pub fn parse_from_str(s: &str) -> anyhow::Result<Self> {
        let re =
            Regex::new(r"(?P<identifier>\w+)\((?P<params>[^)]*)\)(?::\((?P<return>[^)]*)\))?")?;
        let captures = re.captures(s).unwrap();

        let identifier = captures.name("identifier").unwrap().as_str().to_string();

        let params_str = captures.name("params").unwrap().as_str();
        let params_result: anyhow::Result<Vec<String>> = if params_str.is_empty() {
            Ok(vec![])
        } else {
            params_str
                .split(',')
                .map(|s| convert_type_from_abi_type(s.trim()))
                .collect()
        };
        let params = params_result?;

        let return_value_capture = captures.name("return");
        let return_value = if let Some(val) = return_value_capture {
            val.as_str()
                .split(',')
                .map(|s| convert_type_from_abi_type(s.trim()))
                .collect::<anyhow::Result<Vec<String>>>()
        } else {
            Ok(vec![])
        }?;

        Ok(ContractMethodIdentifier {
            identifier,
            params,
            return_value,
        })
    }
}

pub fn convert_type_from_abi_type(s: &str) -> anyhow::Result<String> {
    let param = Reader::read(s).map_err(|e| anyhow::anyhow!(e))?;
    convert_type_from_ethabi_param_type(param)
}

pub fn convert_type_from_ethabi_param_type(param: ethabi::ParamType) -> anyhow::Result<String> {
    let err_msg = "ic_solidity_bindgen::internal::Unimplemented".to_string(); // temp
                                                                              // ref: https://github.com/horizonx-tech/ic-solidity-bindgen/blob/6c9ffb4354cee4c32b1df17a2210c90f16972c21/ic-solidity-bindgen-macros/src/abi_gen.rs#L124
    let ty_str = match param {
        ParamType::Address => ADDRESS_TYPE,
        ParamType::Bytes => "Vec<u8>",
        ParamType::Int(size) => match size {
            129..=256 => bail!(err_msg),
            65..=128 => "i128",
            33..=64 => "i64",
            17..=32 => "i32",
            9..=16 => "i16",
            1..=8 => "i8",
            _ => bail!(err_msg),
        },
        ParamType::Uint(size) => match size {
            129..=256 => U256_TYPE,
            65..=128 => "u128",
            33..=64 => "u64",
            17..=32 => "u32",
            1..=16 => "u16",
            _ => bail!(err_msg),
        },
        ParamType::Bool => "bool",
        ParamType::String => "String",
        ParamType::Array(_) => bail!(err_msg),         // temp
        ParamType::FixedBytes(_) => bail!(err_msg),    // temp
        ParamType::FixedArray(_, _) => bail!(err_msg), // temp
        ParamType::Tuple(_) => bail!(err_msg),         // temp
    };
    Ok(ty_str.to_string())
}

pub enum OutsideCallType {
    Evm,
    Chainsight,
}
/// Generate common identifiers such as storage, setter, etc. for outside calls
pub fn generate_outside_call_idents(type_: &Vec<OutsideCallType>) -> proc_macro2::TokenStream {
    let mut fields = vec![];
    let mut args = vec![];
    for _type in type_ {
        match _type {
            OutsideCallType::Evm => {
                fields.push(quote! {
                    define_web3_ctx!();
                    define_transform_for_web3!();
                    manage_single_state!("target_addr", String, false);
                });
                args.push(quote! {
                    target_addr: String,
                    web3_ctx_param: chainsight_cdk::web3::Web3CtxParam
                });
            }
            OutsideCallType::Chainsight => {
                fields.push(quote! {
                    manage_single_state!("target_canister", String, false);
                });
                args.push(quote! {
                    target_canister: String
                });
            }
        }
    }
    quote! {
        #(#fields)*
        setup_func!({
            #(#args),*
        });
    }
}

// Generate the part of data of the argument that calls the function of datasource contract/canister
pub fn generate_request_arg_idents(
    method_args: &Vec<(String, serde_yaml::Value)>,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::Ident>) {
    let mut value_idents = vec![];
    let mut type_idents = vec![];
    for method_arg in method_args {
        let (type_, value) = method_arg;
        // temp
        let request_arg_value = match type_.clone().as_str() {
            U256_TYPE => match value {
                serde_yaml::Value::String(val) => {
                    quote! { ic_web3_rs::types::U256::from_dec_str(#val).unwrap() }
                }
                serde_yaml::Value::Number(val) => match val.as_u64() {
                    Some(val) => quote! { #val.into() },
                    None => quote! {},
                },
                _ => quote! {},
            },
            ADDRESS_TYPE => match value {
                serde_yaml::Value::String(val) => {
                    quote! { ic_web3_rs::types::Address::from_str(#val).unwrap() }
                }
                _ => quote! {},
            },
            _ => match value {
                serde_yaml::Value::String(val) => {
                    quote! { #val, }
                }
                serde_yaml::Value::Number(val) => match val.as_u64() {
                    Some(val) => {
                        let type_ident = format_ident!("{}", type_);
                        quote! { #val as #type_ident }
                    }
                    None => {
                        quote! {}
                    }
                },
                _ => {
                    quote! {}
                }
            },
        };
        value_idents.push(request_arg_value);
        if type_ == U256_TYPE || type_ == ADDRESS_TYPE {
            // In the case of contract, other than the primitive type (ic_web3_rs::types::U256 etc.) may be set, in which case type_idents is not used.
            type_idents.push(format_ident!("String")); // temp: thread 'main' panicked at '"ic_web3_rs::types::U256" is not a valid Ident'
        } else {
            type_idents.push(format_ident!("{}", type_));
        }
    }
    (value_idents, type_idents)
}

// Comment out once as it may not be used
// /// Generate CustomStruct Identifiers from manifest's struct data
// pub fn generate_custom_struct_idents(custom_structs: &Vec<DatasourceMethodCustomStruct>) -> Vec<proc_macro2::TokenStream> {
//     let mut custom_struct_ident: Vec<proc_macro2::TokenStream> = vec![];
//     for custom_struct_def in custom_structs {
//         let struct_ident = format_ident!("{}", &custom_struct_def.name);
//         let mut custom_struct_fields = vec![];
//         for field in &custom_struct_def.fields {
//             let field_name_ident = format_ident!("{}", &field.name);
//             let field_type_ident = format_ident!("{}", &field.type_);
//             custom_struct_fields.push(quote! {
//                 pub #field_name_ident: #field_type_ident,
//             });
//         }
//         custom_struct_ident.push(quote! {
//             #[derive(Debug, Clone, candid::CandidType, candid::Deserialize)]
//             pub struct #struct_ident {
//                 #(#custom_struct_fields)*
//             }
//         });
//     };
//     custom_struct_ident
// }
// /// Generate CustomType Identifiers from manifest's struct data
// pub fn generate_custom_type_idents(custom_types: &Vec<DatasourceMethodCustomType>) -> Vec<proc_macro2::TokenStream> {
//     let mut custom_type_ident: Vec<proc_macro2::TokenStream> = vec![];
//     for custom_type_def in custom_types {
//         let type_ident = format_ident!("{}", &custom_type_def.name);
//         let mut custom_type_scalars = vec![];
//         for type_ in &custom_type_def.types {
//             custom_type_scalars.push(format_ident!("{}", &type_));
//         }
//         custom_type_ident.push(quote! {
//             type #type_ident = (#(#custom_type_scalars),*);
//         });
//     }
//     custom_type_ident
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_abi_str() {
        assert_eq!(
            ContractMethodIdentifier::parse_from_str("totalSupply()").unwrap(),
            ContractMethodIdentifier {
                identifier: "totalSupply".to_string(),
                params: vec![],
                return_value: vec![]
            }
        );
        assert_eq!(
            ContractMethodIdentifier::parse_from_str("totalSupply():(uint256)").unwrap(),
            ContractMethodIdentifier {
                identifier: "totalSupply".to_string(),
                params: vec![],
                return_value: vec!["ic_web3_rs::types::U256".to_string()]
            }
        );
        assert_eq!(
            ContractMethodIdentifier::parse_from_str("balanceOf(address):(uint256)").unwrap(),
            ContractMethodIdentifier {
                identifier: "balanceOf".to_string(),
                params: vec!["ic_web3_rs::types::Address".to_string()],
                return_value: vec!["ic_web3_rs::types::U256".to_string()]
            }
        );
        assert_eq!(
            ContractMethodIdentifier::parse_from_str("getPool(address,address,uint24):(address)")
                .unwrap(),
            ContractMethodIdentifier {
                identifier: "getPool".to_string(),
                params: vec![
                    "ic_web3_rs::types::Address".to_string(),
                    "ic_web3_rs::types::Address".to_string(),
                    "u32".to_string()
                ],
                return_value: vec!["ic_web3_rs::types::Address".to_string()]
            }
        );
    }
}
