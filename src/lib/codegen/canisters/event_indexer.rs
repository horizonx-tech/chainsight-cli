use anyhow::ensure;
use chainsight_cdk::config::components::EventIndexerConfig;
use quote::quote;

use crate::{
    lib::{
        codegen::components::event_indexer::EventIndexerComponentManifest,
        utils::url::{is_supporting_ipv6_url, is_valid_rpc_url},
    },
    types::ComponentType,
};

pub fn generate_codes(manifest: &EventIndexerComponentManifest) -> anyhow::Result<String> {
    ensure!(
        manifest.metadata.type_ == ComponentType::EventIndexer,
        "type is not EventIndexer"
    );
    let config: EventIndexerConfig = manifest.clone().into();
    let config_json = serde_json::to_string(&config)?;
    let code = quote! {
        use chainsight_cdk_macros::def_event_indexer_canister;
        def_event_indexer_canister!(#config_json);
    };
    Ok(code.to_string())
}

pub fn validate_manifest(manifest: &EventIndexerComponentManifest) -> anyhow::Result<()> {
    ensure!(
        manifest.metadata.type_ == ComponentType::EventIndexer,
        "type is not EventIndexer"
    );

    ensure!(
        manifest.datasource.event.interface.is_some(),
        "datasource.event.interface is not set"
    );

    let rpc_url = &manifest.datasource.network.rpc_url;
    is_supporting_ipv6_url(rpc_url)?;
    is_valid_rpc_url(rpc_url)?;

    Ok(())
}
