use clap::Parser;
use slog::info;

use crate::lib::environment::EnvironmentImpl;

#[derive(Debug, Parser)]
#[command(name = "test")]
/// Test your ChainSight's project
pub struct TestOpts {}

const _GLOBAL_ERROR_MSG: &str = "Fail 'Test' command";

pub fn exec(env: &EnvironmentImpl, _opts: TestOpts) -> anyhow::Result<()> {
    let log = env.get_logger();

    info!(log, r#"Testing successfully"#,);

    Ok(())
}
