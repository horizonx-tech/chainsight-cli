use anyhow::ensure;

use crate::{
    lib::codegen::components::{
        common::CanisterIdType, snapshot_json_rpc::SnapshotJsonRPCComponentManifest,
    },
    types::Network,
};

fn generate_command_to_setup_for_canister(
    label: &str,
    datasrc_id: &str,
    datasrc_id_type: CanisterIdType,
    network: &Network,
) -> String {
    todo!()
}
fn script_contents_for_canister(
    manifest: &SnapshotJsonRPCComponentManifest,
    network: Network,
) -> String {
    todo!()
}

fn generate_command_to_setup_for_contract(
    label: &str,
    datasrc_id: &str,
    datasrc_network_id: u32,
    datasrc_rpc_url: &str,
    network: &Network,
) -> String {
    todo!()
}
fn script_contents_for_contract(
    manifest: &SnapshotJsonRPCComponentManifest,
    network: Network,
) -> String {
    todo!()
}

pub fn generate_scripts(
    manifest: &SnapshotJsonRPCComponentManifest,
    network: Network,
) -> anyhow::Result<String> {
    todo!()
}
