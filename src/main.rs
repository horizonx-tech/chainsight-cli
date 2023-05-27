#![allow(special_module_name)]

mod config;
mod commands;
mod lib;

use clap::{Parser, ArgAction};
use commands::{Command, exec};
use config::cli_version_str;
use lib::{logger::create_root_logger, environment::EnvironmentImpl};

#[derive(Debug, Parser)]
#[command(name = "chainsight", version = cli_version_str())]
#[command(about = "ChainSight Command Line Interface")]
struct Cli {
    /// Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance.
    #[arg(long, short, action = ArgAction::Count, global = true)]
    verbose: u8,

    /// Suppresses informational messages. -qq limits to errors only; -qqqq disables them all.
    #[arg(long, short, action = ArgAction::Count, global = true)]
    quiet: u8,

    #[command(subcommand)]
    command: Command
}

fn main() {
    let args = Cli::parse();
    let verbose_level = args.verbose as i64 - args.quiet as i64;
    let env = EnvironmentImpl::new()
        .with_logger(create_root_logger(verbose_level));
    let res = exec(&env, args.command);
    if let Err(msg) = res {
        eprintln!("{}", msg);
    }
}
