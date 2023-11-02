use anyhow::ensure;
use chainsight_cdk::{
    config::components::SnapshotIndexerICPConfig, convert::candid::CanisterMethodIdentifier,
};
use quote::{format_ident, quote};

use crate::{
    lib::codegen::components::snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
    types::ComponentType,
};

pub fn generate_codes(
    manifest: &SnapshotIndexerICPComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerICP,
        "type is not SnapshotIndexerICP"
    );
    let config: SnapshotIndexerICPConfig = manifest.clone().into();
    let config_json = serde_json::to_string(&config)?;
    let code = quote! {
        use chainsight_cdk_macros::def_snapshot_indexer_icp_canister;
        def_snapshot_indexer_icp_canister!(#config_json);
    };
    Ok(code)
}

pub fn generate_app(
    manifest: &SnapshotIndexerICPComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    if manifest.lens_targets.is_some() {
        return Ok(quote! {});
    }

    let method_identifier = CanisterMethodIdentifier::new(&manifest.datasource.method.identifier)?;
    let (args_ty, _) = method_identifier.get_types();
    let code = if args_ty.is_some() {
        let request_args_ty_name_def_ident =
            format_ident!("{}", CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME);

        quote! {
            mod types;

            pub type CallCanisterArgs = types::#request_args_ty_name_def_ident;
            // todo: set return_value by parsing manifest
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
    };

    Ok(code)
}

pub fn validate_manifest(manifest: &SnapshotIndexerICPComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::SnapshotIndexerICP,
        "type is not SnapshotIndexerICP"
    );

    // TODO
    // - check datasource.method.identifier format
    // - check datasource.method.args length

    Ok(())
}
