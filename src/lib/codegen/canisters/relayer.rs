use anyhow::{bail, ensure};
use chainsight_cdk::config::components::RelayerConfig;
use quote::{format_ident, quote};

use crate::{
    lib::codegen::{
        canisters::common::generate_request_arg_idents,
        components::{common::DestinationType, relayer::RelayerComponentManifest},
    },
    types::ComponentType,
};

use super::common::{CanisterMethodIdentifier, CanisterMethodValueType};

pub fn generate_codes(
    manifest: &RelayerComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::Relayer,
        "type is not Relayer"
    );
    let config: RelayerConfig = manifest.clone().into();
    let config_json = serde_json::to_string(&config)?;
    let code = quote! {
        use chainsight_cdk_macros::def_relayer_canister;
        def_relayer_canister!(#config_json);
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
