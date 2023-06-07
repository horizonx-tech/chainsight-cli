use quote::{format_ident, quote};

use crate::lib::codegen::components::common::{DatasourceMethodCustomStruct, DatasourceMethodCustomType, DatasourceMethodArg};

pub enum OutsideCallIdentsType {
    Eth,
    CrossCanisterCall,
    All
}
/// Generate common identifies such as storage, setter, etc. for outside calls
pub fn generate_outside_call_idents(type_: OutsideCallIdentsType) -> proc_macro2::TokenStream {
    let eth_idents = quote! {
        define_web3_ctx!();
        define_transform_for_web3!();

        manage_single_state!("target_addr", String, false);
    };
    let cross_canister_call_idents = quote! {
        manage_single_state!("target_canister", String, false);
    };
    match type_ {
        OutsideCallIdentsType::Eth => {
            quote! {
                #eth_idents

                setup_func!({
                    target_addr: String,
                    web3_ctx_param: Web3CtxParam
                });
            }
        },
        OutsideCallIdentsType::CrossCanisterCall => {
            quote! {
                #cross_canister_call_idents

                setup_func!({
                    target_canister: String
                });
            }
        },
        OutsideCallIdentsType::All => {
            quote! {
                #eth_idents
                #cross_canister_call_idents

                setup_func!({
                    target_canister: String,
                    target_addr: String,
                    web3_ctx_param: Web3CtxParam
                });
            }
        }
    }
}

// Generate the part of data of the argument that calls the function of datasource contract/canister
pub fn generate_request_arg_idents(method_args: &Vec<DatasourceMethodArg>) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::Ident>) {
    let mut value_idents = vec![];
    let mut type_idents = vec![];
    for method_arg in method_args {
        let DatasourceMethodArg { type_, value } = method_arg;
        // temp
        let request_arg_value = match type_.clone().as_str() {
            "ic_web3::types::U256" => {
                match value {
                    serde_yaml::Value::String(val) => quote! { ic_web3::types::U256::from_dec_str(#val).unwrap() },
                    serde_yaml::Value::Number(val) => {
                        match val.as_u64() {
                            Some(val) => quote! { #val.into() },
                            None => quote! {}
                        }
                    },
                    _ => quote! {}
                }
            }
            "ic_web3::types::Address" => {
                match value {
                    serde_yaml::Value::String(val) => quote! { ic_web3::types::Address::from_str(#val).unwrap() },
                    _ => quote! {}
                }
            },
            _ => {
                match value {
                    serde_yaml::Value::String(val) => {
                        quote! { #val, }
                    },
                    serde_yaml::Value::Number(val) => {
                        match val.as_u64() {
                            Some(val) => {
                                let type_ident = format_ident!("{}", type_);
                                quote! { #val as #type_ident }
                            },
                            None => {
                                quote! {}
                            }
                        }
                    },
                    _ => {
                        quote! {}
                    }
                }
            }
        };
        value_idents.push(request_arg_value);
        if type_ == "ic_web3::types::U256" || type_ == "ic_web3::types::Address" {
            // In the case of contract, other than the primitive type (ic_web3::types::U256 etc.) may be set, in which case type_idents is not used.
            type_idents.push(format_ident!("String")); // temp: thread 'main' panicked at '"ic_web3::types::U256" is not a valid Ident'
        } else {
            type_idents.push(format_ident!("{}", type_));
        }
    };
    (value_idents, type_idents)
}

// Generate CustomStruct Identifiers from manifest's struct data
pub fn generate_custom_struct_idents(custom_structs: &Vec<DatasourceMethodCustomStruct>) -> Vec<proc_macro2::TokenStream> {
    let mut custom_struct_ident: Vec<proc_macro2::TokenStream> = vec![];
    for custom_struct_def in custom_structs {
        let struct_ident = format_ident!("{}", &custom_struct_def.name);
        let mut custom_struct_fields = vec![];
        for field in &custom_struct_def.fields {
            let field_name_ident = format_ident!("{}", &field.name);
            let field_type_ident = format_ident!("{}", &field.type_);
            custom_struct_fields.push(quote! {
                pub #field_name_ident: #field_type_ident,
            });
        }
        custom_struct_ident.push(quote! {
            #[derive(Debug, Clone, candid::CandidType, candid::Deserialize)]
            pub struct #struct_ident {
                #(#custom_struct_fields)*
            }
        });
    };
    custom_struct_ident
}

// Generate CustomType Identifiers from manifest's struct data
pub fn generate_custom_type_idents(custom_types: &Vec<DatasourceMethodCustomType>) -> Vec<proc_macro2::TokenStream> {
    let mut custom_type_ident: Vec<proc_macro2::TokenStream> = vec![];
    for custom_type_def in custom_types {
        let type_ident = format_ident!("{}", &custom_type_def.name);
        let mut custom_type_scalars = vec![];
        for type_ in &custom_type_def.types {
            custom_type_scalars.push(format_ident!("{}", &type_));
        }
        custom_type_ident.push(quote! {
            type #type_ident = (#(#custom_type_scalars),*);
        });
    }
    custom_type_ident
}