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

pub fn generate_queries_without_timestamp(
    return_type: proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let query_derives = quote! {
        #[ic_cdk::query]
        #[candid::candid_method(query)]
    };
    let update_derives = quote! {
        #[ic_cdk::update]
        #[candid::candid_method(update)]
    };

    quote! {
        fn _get_last_snapshot_value() -> #return_type {
            get_last_snapshot().value
        }

        fn _get_top_snapshot_values(n: u64) -> Vec<#return_type> {
            get_top_snapshots(n).iter().map(|s| s.value.clone()).collect()
        }

        fn _get_snapshot_value(idx: u64) -> #return_type {
            get_snapshot(idx).value
        }

        #query_derives
        pub fn get_last_snapshot_value() -> #return_type {
            _get_last_snapshot_value()
        }

        #query_derives
        pub fn get_top_snapshot_values(n: u64) -> Vec<#return_type> {
            _get_top_snapshot_values(n)
        }

        #query_derives
        pub fn get_snapshot_value(idx: u64) -> #return_type {
            _get_snapshot_value(idx)
        }

        #update_derives
        pub async fn proxy_get_last_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProviderWithoutArgs::<#return_type>::new(
                proxy(),
                _get_last_snapshot_value,
            )
            .reply(input)
            .await
        }

        #update_derives
        pub async fn proxy_get_top_snapshot_values(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProvider::<u64, Vec<#return_type>>::new(
                proxy(),
                _get_top_snapshot_values,
            )
            .reply(input)
            .await
        }

        #update_derives
        pub async fn proxy_get_snapshot_value(input: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
            use chainsight_cdk::rpc::Receiver;
            chainsight_cdk::rpc::ReceiverProvider::<u64, #return_type>::new(
                proxy(),
                _get_snapshot_value,
            )
            .reply(input)
            .await
        }
    }
}
