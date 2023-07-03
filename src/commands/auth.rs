use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "auth")]
/// Action about Auth, change address to deploy etc
pub struct AuthOpts {}
