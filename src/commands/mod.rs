use clap::{Subcommand};

use crate::lib::environment::EnvironmentImpl;

mod config;
mod auth;
mod new;
mod create;
mod build;
mod test;
mod deploy;
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
        Command::Test(_) => {
            println!("Test");
            Ok(())
        },
        Command::Deploy(_) => {
            println!("Deploy");
            Ok(())
        },
        Command::Remove(opts) => remove::exec(env, opts),
        Command::Upgrade(_) => {
            println!("Upgrade");
            Ok(())
        }
    }
}