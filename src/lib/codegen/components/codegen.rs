use anyhow::Result;
use ethabi::Contract;

use crate::{
    lib::codegen::canisters::snapshot_indexer_https::JsonTypeGenStrategyImpl,
    types::{ComponentType, Network},
};

use super::{
    algorithm_indexer::{AlgorithmIndexerCodeGenerator, AlgorithmIndexerComponentManifest},
    algorithm_lens::{AlgorithmLensCodeGenerator, AlgorithmLensComponentManifest},
    common::{ComponentManifest, GeneratedCodes},
    event_indexer::{EventIndexerCodeGenerator, EventIndexerComponentManifest},
    relayer::{RelayerCodeGenerator, RelayerComponentManifest},
    snapshot_indexer_evm::{SnapshotIndexerEVMComponentManifest, SnapshotIndexerEvmCodeGenerator},
    snapshot_indexer_https::{
        SnapshotIndesxerHTTPSCodeGenerator, SnapshotIndexerHTTPSComponentManifest,
    },
    snapshot_indexer_icp::{SnapshotIndexerICPCodeGenerator, SnapshotIndexerICPComponentManifest},
};

pub trait CodeGenerator {
    fn generate_code(&self, interface_contract: Option<Contract>) -> Result<GeneratedCodes>;
    fn generate_scripts(&self, network: Network) -> anyhow::Result<String>;
    fn generate_user_impl_template(&self) -> anyhow::Result<GeneratedCodes>;
    fn manifest(&self) -> Box<dyn ComponentManifest>;
    fn generate_component_setup_args(&self, network: &Network) -> anyhow::Result<Option<Vec<u8>>>;
}

pub fn generator(
    component_type: ComponentType,
    component_path: &str,
    id: &str,
) -> Result<Box<dyn CodeGenerator>> {
    match component_type {
        ComponentType::EventIndexer => Ok(Box::new(EventIndexerCodeGenerator::new(
            EventIndexerComponentManifest::load_with_id(component_path, id)?,
        ))),
        ComponentType::AlgorithmIndexer => Ok(Box::new(AlgorithmIndexerCodeGenerator::new(
            AlgorithmIndexerComponentManifest::load_with_id(component_path, id)?,
        ))),
        ComponentType::SnapshotIndexerICP => Ok(Box::new(SnapshotIndexerICPCodeGenerator::new(
            SnapshotIndexerICPComponentManifest::load_with_id(component_path, id)?,
        ))),
        ComponentType::SnapshotIndexerEVM => Ok(Box::new(SnapshotIndexerEvmCodeGenerator::new(
            SnapshotIndexerEVMComponentManifest::load_with_id(component_path, id)?,
        ))),
        ComponentType::Relayer => Ok(Box::new(RelayerCodeGenerator::new(
            RelayerComponentManifest::load_with_id(component_path, id)?,
        ))),
        ComponentType::AlgorithmLens => Ok(Box::new(AlgorithmLensCodeGenerator::new(
            AlgorithmLensComponentManifest::load_with_id(component_path, id)?,
        ))),
        ComponentType::SnapshotIndexerHTTPS => {
            Ok(Box::new(SnapshotIndesxerHTTPSCodeGenerator::new(
                SnapshotIndexerHTTPSComponentManifest::load_with_id(component_path, id)?,
                Box::new(JsonTypeGenStrategyImpl),
            )))
        }
    }
}
