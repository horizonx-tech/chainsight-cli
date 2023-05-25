use clap::{command, Subcommand};

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

pub fn exec(cmd: Command) {
    match cmd {
        Command::Config(_) => {
            println!("Config");
        },
        Command::Auth(_) => {
            println!("Auth");
        },
        Command::New(_) => {
            println!("New");
        },
        Command::Create(_) => {
            println!("Create");
        },
        Command::Build(_) => {
            println!("Build");
        },
        Command::Test(_) => {
            println!("Test");
        },
        Command::Deploy(_) => {
            println!("Deploy");
        },
        Command::Remove(_) => {
            println!("Remove");
        },
        Command::Upgrade(_) => {
            println!("Upgrade");
        }
    }
}