use std::path::Path;

use anyhow::{bail, Context};
use candid::{Decode, Encode, Principal};
use clap::Parser;
use slog::info;

use crate::{
    commands::utils::{output_by_exec_cmd, DfxArgsBuilder},
    lib::{
        environment::EnvironmentImpl,
        utils::{is_chainsight_project, ARTIFACTS_DIR},
    },
    types::Network,
};

#[derive(Debug, Parser)]
#[command(name = "component-info")]
/// Display component information. IDs and other information, including sidecars, can be checked.
pub struct ComponentInfoOpts {
    /// Specify the path of the project to be deleted.
    /// If not specified, the current directory is targeted.
    #[arg(long, short = 'p')]
    pub path: Option<String>,

    /// Specify the component name or canister id to delete.
    #[arg(long, short = 'c')]
    component: String,

    /// Specify the network to execute on.
    #[arg(long)]
    #[clap(default_value = "local")]
    network: Network,

    /// Specifies the port to call.
    /// This option is used only if the target is localhost.
    #[arg(long)]
    port: Option<u16>,
}

pub async fn exec(env: &EnvironmentImpl, opts: ComponentInfoOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    info!(log, r#"Start info component '{}'..."#, opts.component);

    let working_dir_str = working_dir(opts.path.clone())?;
    let working_dir = Path::new(&working_dir_str);

    let component_id = Principal::from_text(&opts.component).unwrap_or_else(|_| {
        canister_id_from_canister_name(working_dir, &opts.network, &opts.component)
            .expect("failed to get canister id")
    });
    let url = match &opts.network {
        Network::Local => format!("http://localhost:{}", opts.port.unwrap_or(4943)),
        Network::IC => "https://ic0.app/".to_string(),
    };
    let agent = agent(&url);
    if opts.network == Network::Local {
        agent.fetch_root_key().await.unwrap();
    }

    info!(log, "Confirm sidecars to delete");
    let res = exec_internal(&agent, &component_id).await?;
    info!(log, "  proxy: {}", res.proxy.to_text());
    info!(log, "  vault: {}", res.vault.to_text());
    info!(log, "  db: {}", res.db.to_text());

    Ok(())
}

pub struct ComponentInfo {
    pub proxy: Principal,
    pub vault: Principal,
    pub db: Principal,
}
pub async fn exec_internal(
    agent: &ic_agent::Agent,
    component_id: &Principal,
) -> anyhow::Result<ComponentInfo> {
    let proxy = get_proxy_from_component(&agent, &component_id).await?;
    let vault = vault_from_proxy(&agent, &proxy).await?;
    let db = db_from_proxy(&agent, &proxy).await?;
    Ok(ComponentInfo { proxy, vault, db })
}

// Return artifacts path as working directory
fn working_dir(project_path: Option<String>) -> anyhow::Result<String> {
    let path = if let Some(project_path) = &project_path {
        // Assuming the specified path is the project path, calculate the artifacts path.
        if let Err(msg) = is_chainsight_project(Some(project_path.clone())) {
            bail!(format!(r#"{}"#, msg));
        };
        format!("{}/{}", project_path, ARTIFACTS_DIR)
    } else {
        let current_path_str = ".";
        let current_path = Path::new(current_path_str);
        // If dfx.json is available, current path is used as artifacts path
        if current_path.join("dfx.json").exists() {
            current_path_str.to_string()
        } else {
            // Determine if it is a project path.
            if let Err(msg) = is_chainsight_project(None) {
                bail!(format!(r#"{}"#, msg));
            };
            format!("./{}", ARTIFACTS_DIR)
        }
    };

    Ok(path)
}

fn agent(url: &str) -> ic_agent::Agent {
    ic_agent::Agent::builder()
        .with_url(url)
        .with_verify_query_signatures(false)
        .build()
        .unwrap()
}

async fn get_proxy_from_component(
    agent: &ic_agent::Agent,
    principal: &Principal,
) -> anyhow::Result<Principal> {
    let res = agent
        .update(principal, "get_proxy")
        .with_arg(Encode!().unwrap())
        .call_and_wait()
        .await?;
    Ok(Decode!(res.as_slice(), Principal).unwrap())
}

async fn vault_from_proxy(
    agent: &ic_agent::Agent,
    principal: &Principal,
) -> anyhow::Result<Principal> {
    let res = agent
        .query(principal, "vault")
        .with_arg(Encode!().unwrap())
        .call()
        .await?;
    Ok(Decode!(res.as_slice(), Principal).unwrap())
}

async fn db_from_proxy(
    agent: &ic_agent::Agent,
    principal: &Principal,
) -> anyhow::Result<Principal> {
    let res = agent
        .query(principal, "db")
        .with_arg(Encode!().unwrap())
        .call()
        .await?;
    Ok(Decode!(res.as_slice(), Principal).unwrap())
}

fn canister_id_from_canister_name(
    execution_dir: &Path,
    network: &Network,
    canister_name: &str,
) -> Result<Principal, String> {
    let args_builder = DfxArgsBuilder::new_only_network(network.clone());
    let args = args_builder.generate(vec!["canister", "id", canister_name]);

    let output = output_by_exec_cmd("dfx", execution_dir, args).expect("failed to execute process");
    if output.status.success() {
        let msg = std::str::from_utf8(&output.stdout).unwrap_or("failed to parse stdout");
        Ok(Principal::from_text(msg.replace('\n', "")).unwrap())
    } else {
        let msg = std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr");
        Err(msg.to_string())
    }
}
