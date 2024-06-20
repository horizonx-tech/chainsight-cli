use anyhow::Context;
use candid::{Encode, Principal};
use chainsight_cdk::indexer::IndexingConfig;

use crate::lib::{
    codegen::components::algorithm_indexer::AlgorithmIndexerComponentManifest,
    utils::component_ids_manager::ComponentIdsManager,
};

pub fn generate_component_setup_args(
    manifest: &AlgorithmIndexerComponentManifest,
    comp_id_mgr: &ComponentIdsManager,
) -> anyhow::Result<Vec<u8>> {
    let datasource_target = &manifest.datasource.principal;
    let principal = if Principal::from_text(datasource_target).is_ok() {
        datasource_target.to_string()
    } else {
        comp_id_mgr
            .get(datasource_target)
            .context(format!(
                "Failed to get canister id for {}",
                datasource_target
            ))
            .unwrap()
    };
    let args = Encode!(
        &principal,
        &IndexingConfig {
            start_from: manifest.datasource.from,
            chunk_size: manifest.datasource.batch_size,
        }
    )?;

    Ok(args)
}
