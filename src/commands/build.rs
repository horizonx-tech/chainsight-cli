use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "build")]
/// Build your ChainSight's project
pub struct BuildOpts {}