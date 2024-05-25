use clap::Subcommand;
use tokio::runtime::Runtime;

use crate::lib::{environment::EnvironmentImpl, utils::interaction::RealUserInteraction};

mod add;
mod auth;
mod build;
mod component_info;
mod config;
mod delete;
mod deploy;
mod exec;
mod generate;
mod new;
mod remove;
mod test;
mod upgrade;
mod utils;

mod identity;

#[cfg(feature = "integration-test")]
mod tests;

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
    Delete(delete::DeleteOpts),
    // Upgrade(upgrade::UpgradeOpts),

    // Experimental
    ComponentInfo(component_info::ComponentInfoOpts),
    Identity(identity::IdentityOpts),
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
        Command::Delete(opts) => {
            let runtime = Runtime::new().expect("Unable to create a runtime");
            runtime.block_on(delete::exec(env, opts))?;
            Ok(())
        }
        // Command::Upgrade(_) => {
        //     println!("Not implemented yet...");
        //     Ok(())
        // }
        Command::ComponentInfo(opts) => {
            let runtime = Runtime::new().expect("Unable to create a runtime");
            runtime.block_on(component_info::exec(env, opts))?;
            Ok(())
        }
        Command::Identity(opts) => identity::exec(env, opts),
    }
}
