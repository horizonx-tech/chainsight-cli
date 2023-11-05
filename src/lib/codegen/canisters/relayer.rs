use anyhow::ensure;
use chainsight_cdk::{
    config::components::RelayerConfig, convert::candid::CanisterMethodIdentifier,
};
use quote::{format_ident, quote};

use crate::{lib::codegen::components::relayer::RelayerComponentManifest, types::ComponentType};

pub fn generate_codes(manifest: &RelayerComponentManifest) -> anyhow::Result<String> {
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
    Ok(code.to_string())
}

pub fn generate_app(manifest: &RelayerComponentManifest) -> anyhow::Result<String> {
    let call_args_idents = if manifest.lens_targets.is_some() {
        quote! {}
    } else {
        let method_identifier =
            CanisterMethodIdentifier::new(&manifest.datasource.method.identifier)?;
        let (request_args_type, _) = method_identifier.get_types();
        if request_args_type.is_some() {
            let request_args_ident =
                format_ident!("{}", CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME);
            quote! {
                pub type CallCanisterArgs = types::#request_args_ident;
                pub fn call_args() -> CallCanisterArgs {
                    todo!()
                }
            }
        } else {
            quote! {
                pub type CallCanisterArgs = ();
                pub fn call_args() -> CallCanisterArgs {
                    ()
                }
            }
        }
    };

    let response_ident = format_ident!("{}", CanisterMethodIdentifier::RESPONSE_TYPE_NAME);
    Ok(quote! {
        mod types;
        pub type CallCanisterResponse = types::#response_ident;
        #call_args_idents
        pub fn filter(_: &CallCanisterResponse) -> bool {
            true
        }
    }
    .to_string())
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
