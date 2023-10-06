use clap::Subcommand;

use crate::lib::{environment::EnvironmentImpl, utils::interaction::RealUserInteraction};

mod add;
mod auth;
mod build;
mod config;
mod deploy;
mod exec;
mod generate;
mod new;
mod remove;
mod test;
mod upgrade;

#[derive(Debug, Subcommand)]
pub enum Command {
    // Config(config::ConfigOpts),
    // Auth(auth::AuthOpts),
    New(new::NewOpts),
    Add(add::AddOpts),
    Generate(generate::GenerateOpts),
    Build(build::BuildOpts),
    // Test(test::TestOpts),
    Deploy(deploy::DeployOpts),
    Exec(exec::ExecOpts),
    Remove(remove::RemoveOpts),
    // Upgrade(upgrade::UpgradeOpts),
}

pub fn exec(env: &EnvironmentImpl, cmd: Command) -> anyhow::Result<()> {
    let interaction = &mut RealUserInteraction {};
    match cmd {
        // Command::Config(_) => {
        //     println!("Not implemented yet...");
        //     Ok(())
        // }
        // Command::Auth(_) => {
        //     println!("Not implemented yet...");
        //     Ok(())
        // }
        Command::New(opts) => new::exec(env, opts),
        Command::Add(opts) => add::exec(env, opts, interaction),
        Command::Generate(opts) => generate::exec(env, opts),
        Command::Build(opts) => build::exec(env, opts),
        // Command::Test(_) => {
        //     println!("Not implemented yet...");
        //     Ok(())
        // }
        Command::Deploy(opts) => deploy::exec(env, opts),
        Command::Exec(opts) => exec::exec(env, opts),
        Command::Remove(opts) => remove::exec(env, opts, interaction),
        // Command::Upgrade(_) => {
        //     println!("Not implemented yet...");
        //     Ok(())
        // }
    }
}
