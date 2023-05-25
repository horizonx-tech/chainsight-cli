mod config;
mod commands;

use clap::{Parser};
use commands::{Command, exec};
use config::cli_version_str;

#[derive(Debug, Parser)]
#[command(name = "chainsight", version = cli_version_str())]
#[command(about = "ChainSight Command Line Interface")]
struct Cli {
    #[command(subcommand)]
    command: Command
}

fn main() {
    let args = Cli::parse();
    exec(args.command)
}
