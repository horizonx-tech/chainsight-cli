use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "new")]
/// Generate ChainSight project by prepared template
pub struct NewOpts {}