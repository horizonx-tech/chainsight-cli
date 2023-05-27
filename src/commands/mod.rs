use clap::{command, Subcommand};

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
    #[command(arg_required_else_help = true)]
    Config(config::ConfigOpts),
    #[command(arg_required_else_help = true)]
    Auth(auth::AuthOpts),
    #[command(arg_required_else_help = true)]
    New(new::NewOpts),
    #[command(arg_required_else_help = true)]
    Create(create::CreateOpts),
    #[command(arg_required_else_help = true)]
    Build(build::BuildOpts),
    #[command(arg_required_else_help = true)]
    Test(test::TestOpts),
    #[command(arg_required_else_help = true)]
    Deploy(deploy::DeployOpts),
    #[command(arg_required_else_help = true)]
    Remove(remove::RemoveOpts),
    #[command(arg_required_else_help = true)]
    Upgrade(upgrade::UpgradeOpts)
}

pub fn exec(env: &EnvironmentImpl, cmd: Command) -> Result<(), String> {
    match cmd {
        Command::Config(_) => {
            println!("Config");
            Ok(())
        },
        Command::Auth(_) => {
            println!("Auth");
            Ok(())
        },
        Command::New(opt) => new::exec(env, opt),
        Command::Create(_) => {
            println!("Create");
            Ok(())
        },
        Command::Build(_) => {
            println!("Build");
            Ok(())
        },
        Command::Test(_) => {
            println!("Test");
            Ok(())
        },
        Command::Deploy(_) => {
            println!("Deploy");
            Ok(())
        },
        Command::Remove(_) => {
            println!("Remove");
            Ok(())
        },
        Command::Upgrade(_) => {
            println!("Upgrade");
            Ok(())
        }
    }
}