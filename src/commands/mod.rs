use clap::{Subcommand};

use crate::lib::environment::EnvironmentImpl;

mod config;
mod auth;
mod new;
mod create;
mod build;
mod test;
mod deploy;
mod exec;
mod remove;
mod upgrade;

#[derive(Debug, Subcommand)]
pub enum Command {
    Config(config::ConfigOpts),
    Auth(auth::AuthOpts),
    New(new::NewOpts),
    Create(create::CreateOpts),
    Build(build::BuildOpts),
    Test(test::TestOpts),
    Deploy(deploy::DeployOpts),
    Exec(exec::ExecOpts),
    Remove(remove::RemoveOpts),
    Upgrade(upgrade::UpgradeOpts)
}

pub fn exec(env: &EnvironmentImpl, cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Config(_) => {
            println!("Config");
            Ok(())
        },
        Command::Auth(_) => {
            println!("Auth");
            Ok(())
        },
        Command::New(opts) => new::exec(env, opts),
        Command::Create(opts) => create::exec(env, opts),
        Command::Build(opts) => build::exec(env, opts),
        Command::Test(opts) => test::exec(env, opts),
        Command::Deploy(opts) => deploy::exec(env, opts),
        Command::Exec(opts) => exec::exec(env, opts),
        Command::Remove(opts) => remove::exec(env, opts),
        Command::Upgrade(_) => {
            println!("Upgrade");
            Ok(())
        }
    }
}
