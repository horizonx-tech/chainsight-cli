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
    utils::{
        self,
        dfx::{DfxWrapper, DfxWrapperNetwork},
        env::cache_envfile,
    },
};
use slog::{error, info, Logger};

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
    let exit_code = run();
    std::process::exit(exit_code);
}

// ref: https://docs.rs/slog-async/latest/slog_async/#beware-of-stdprocessexit
fn run() -> i32 {
    let args = Cli::parse();
    let _ = cache_envfile(None); // NOTE: Proceed regardless of the absence of an env file or environment variables.
    let verbose_level = args.verbose as i64 - args.quiet as i64;
    let logger = create_root_logger(verbose_level);
    info_on_bin_deps_for_csx(&logger);

    let env = EnvironmentImpl::new().with_logger(logger.clone());
    let res = exec(&env, args.command);
    if let Err(msg) = res {
        error!(&logger, r#"{:#}"#, msg);
        return 1;
    }
    0
}

fn info_on_bin_deps_for_csx(logger: &Logger) {
    let dfx = DfxWrapper::new(DfxWrapperNetwork::IC, None);
    if let Ok((dfx, version)) = dfx {
        info!(logger, "Dfx version: {}", version);
        info!(
            logger,
            "Currently active user identity context: {}",
            dfx.identity_whoami().unwrap_or("Nothing".to_string())
        );
        info!(
            logger,
            "Current identity principal: {}",
            dfx.identity_get_principal()
                .unwrap_or("Nothing".to_string())
        );
        info!(
            logger,
            "Wallet associated with current identity (ic): {}",
            dfx.identity_get_wallet().unwrap_or("Nothing".to_string())
        );
    } else {
        info!(logger, "Dfx version: Not Found");
    };
}
