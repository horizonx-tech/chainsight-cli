#![allow(special_module_name)]

mod commands;
mod config;
mod lib;
mod types;

use clap::{ArgAction, Parser};
use commands::{exec, Command};
use config::cli_version_str;
use lib::{
    environment::EnvironmentImpl,
    logger::create_root_logger,
    utils::{self, env::cache_envfile},
};
use slog::error;

/// The Chainsight Executor
#[derive(Debug, Parser)]
#[command(name = "csx", version = cli_version_str(), about = "Chainsight command-line execution envirionment", styles = utils::clap::style())]
struct Cli {
    /// Displays detailed information about operations. -vv will generate a very large number of messages and can affect performance.
    #[arg(long, short, action = ArgAction::Count, global = true)]
    verbose: u8,

    /// Suppresses informational messages. -qq limits to errors only; -qqqq disables them all.
    #[arg(long, short, action = ArgAction::Count, global = true)]
    quiet: u8,

    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Cli::parse();
    let _ = cache_envfile(None); // NOTE: Proceed regardless of the absence of an env file or environment variables.
    let verbose_level = args.verbose as i64 - args.quiet as i64;
    let logger = create_root_logger(verbose_level);
    let env = EnvironmentImpl::new().with_logger(logger.clone());
    let res = exec(&env, args.command);
    if let Err(msg) = res {
        error!(&logger, r#"{}"#, msg);
        std::process::exit(1);
    }
    std::process::exit(0);
}
