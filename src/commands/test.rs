use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "test")]
/// Test your ChainSight's project
pub struct TestOpts {}