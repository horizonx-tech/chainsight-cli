use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "upgrade")]
/// Upgrade your Chainsight's project.
pub struct UpgradeOpts {}
