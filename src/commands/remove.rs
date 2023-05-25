use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "remove")]
/// Delete resources for specified your project
pub struct RemoveOpts {}