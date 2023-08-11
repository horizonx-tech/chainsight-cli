use anyhow::{bail, ensure};
use quote::{format_ident, quote};

use crate::lib::codegen::components::snapshot_json_rpc::SnapshotJsonRPCComponentManifest;

pub fn generate_codes(
    manifest: &SnapshotJsonRPCComponentManifest,
) -> anyhow::Result<proc_macro2::TokenStream> {
    todo!()
}
