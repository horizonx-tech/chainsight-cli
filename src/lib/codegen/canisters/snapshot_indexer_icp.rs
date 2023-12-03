use anyhow::ensure;
use chainsight_cdk::{
    config::components::{SnapshotIndexerICPConfig, LENS_FUNCTION_ARGS_TYPE},
    convert::candid::CanisterMethodIdentifier,
};
use quote::{format_ident, quote};

use crate::{
    lib::{
        codegen::components::{
            algorithm_lens::AlgorithmLensComponentManifest,
            snapshot_indexer_icp::SnapshotIndexerICPComponentManifest,
            utils::{generate_method_identifier, get_did_by_component_id, is_lens_with_args},
        },
        utils::paths::bindings_name,
    },
    types::ComponentType,
};

pub fn generate_codes(manifest: &SnapshotIndexerICPComponentManifest) -> anyhow::Result<String> {
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
    Ok(code.to_string())
}

pub fn generate_app(manifest: &SnapshotIndexerICPComponentManifest) -> anyhow::Result<String> {
    let method = manifest.datasource.method.clone();
    let interface = if method.interface.is_some() {
        method.interface.clone()
    } else {
        get_did_by_component_id(&manifest.datasource.location.id)
    };
    let method_identifier = generate_method_identifier(&method.identifier, &interface)?;

    if manifest.lens_targets.is_some() {
        let codes = if is_lens_with_args(method_identifier) {
            let id = manifest.id.clone().expect("id is not set");
            let bindings = format_ident!("{}", bindings_name(&id));
            let lens_args_ident = format_ident!("{}", LENS_FUNCTION_ARGS_TYPE);
            let calculate_args_ident = format_ident!(
                "{}",
                AlgorithmLensComponentManifest::CALCULATE_ARGS_STRUCT_NAME
            );

            quote! {
                pub type #calculate_args_ident = #bindings::#calculate_args_ident;
                pub type #lens_args_ident = #bindings::#lens_args_ident;
                pub fn call_args() -> #calculate_args_ident {
                    todo!("generate CalculateArgs as args to call")
                }
            }
        } else {
            quote! {}
        };

        return Ok(codes.to_string());
    }

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

    Ok(code.to_string())
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
