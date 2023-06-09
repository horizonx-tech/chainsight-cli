use proc_macro2::TokenStream;

use super::components::{snapshot::SnapshotComponentManifest, relayer::RelayerComponentManifest};

mod snapshot;
mod relayer;
mod common;

pub fn generate_snapshot_codes(manifest: &SnapshotComponentManifest) -> anyhow::Result<TokenStream> {
    snapshot::generate_codes(manifest)
}

pub fn generate_relayer_codes(manifest: &RelayerComponentManifest) -> anyhow::Result<TokenStream> {
    relayer::generate_codes(manifest)
}

pub fn validate_snapshot_manifest(manifest: &SnapshotComponentManifest) -> anyhow::Result<()> {
    snapshot::validate_manifest(manifest)
}

pub fn validate_relayer_manifest(manifest: &RelayerComponentManifest) -> anyhow::Result<()> {
    relayer::validate_manifest(manifest)
}
