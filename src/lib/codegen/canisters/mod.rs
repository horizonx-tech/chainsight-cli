use proc_macro2::TokenStream;

use super::components::{SnapshotComponentManifest, RelayerComponentManifest};

mod snapshot;
mod relayer;

pub fn generate_snapshot_codes(manifest: &SnapshotComponentManifest) -> anyhow::Result<TokenStream> {
    snapshot::generate_codes(manifest)
}

pub fn generate_relayer_codes(manifest: &RelayerComponentManifest) -> anyhow::Result<TokenStream> {
    relayer::generate_codes(manifest)
}
