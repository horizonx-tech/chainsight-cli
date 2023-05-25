use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "deploy")]
/// Deploy your ChainSight's project
pub struct DeployOpts {}