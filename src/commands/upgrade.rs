use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "upgrade")]
/// Upgrade your ChainSight's project
pub struct UpgradeOpts {}
