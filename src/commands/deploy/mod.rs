use std::path::Path;

use anyhow::{anyhow, Ok};
use candid::Principal;
use chainsight_cdk::core::Env;
use clap::Parser;
use ic_agent::Identity;
use slog::{info, Logger};
use types::ComponentsToDeploy;

use crate::{
    lib::{
        codegen::project::ProjectManifestData,
        environment::EnvironmentImpl,
        utils::{
            component_ids_manager::ComponentIdsManager,
            dfx::{DfxWrapper, DfxWrapperNetwork},
            identity::{get_wallet_principal_from_local_context, identity_from_context},
            ARTIFACTS_DIR, PROJECT_MANIFEST_FILENAME,
        },
    },
    types::Network,
};

pub mod functions;
mod types;

#[derive(Debug, Parser)]
#[command(name = "deploy")]
/// Deploy the components of your project.
/// If you want to operate on a local network, you need to build a local dfx network in advance.
pub struct DeployOpts {
    /// Specify the path of the project to deploy.
    /// If not specified, the current directory is targeted.
    #[arg(long, short = 'p')]
    path: Option<String>,

    /// Specify the component to deploy.
    /// If this option is not specified, the command will be given to all components managed by the project.
    #[arg(long, short = 'c')]
    component: Option<String>,

    /// Specify the context of identity to execute on.
    /// If this option is specfied, the default context is used.
    #[arg(long)]
    context: Option<String>,

    /// Specify the network to execute on.
    #[arg(long)]
    #[clap(default_value = "local")]
    network: Network,

    /// Specifies the port to call.
    /// This option is used only if the target is localhost.
    #[arg(long)]
    port: Option<u16>,

    /// Specify the wallet to use.
    /// If this option is not specified & no string, the default wallet is used.
    #[arg(long, short = 'w')]
    wallet: Option<Option<String>>,

    /// Specify the initial number of cycles for canister.
    /// Used as a parameter for `dfx canister create`.
    #[arg(long)]
    with_cycles: Option<u128>,
}

pub async fn exec(env: &EnvironmentImpl, opts: DeployOpts) -> anyhow::Result<()> {
    let log = env.get_logger();
    let project_path_str = opts.path.unwrap_or(".".to_string());
    let artifacts_path_str = format!("{}/{}", &project_path_str, ARTIFACTS_DIR);
    let artifacts_path = Path::new(&artifacts_path_str);
    let project_manifest = ProjectManifestData::load(&format!(
        "{}/{}",
        &project_path_str, PROJECT_MANIFEST_FILENAME
    ))?;
    let network = opts.network;

    info!(log, "Checking environments...");
    check_before_deployment(log, artifacts_path, opts.port, network.clone())?;
    info!(log, "Checking environments finished successfully");

    info!(
        log,
        r#"Start deploying project '{}'..."#, project_manifest.label
    );
    let components_to_deploy = if let Some(component) = opts.component {
        ComponentsToDeploy::Single(component)
    } else {
        // todo: clean to collect component ids, better to use only manifest.yaml?
        let components = project_manifest
            .load_code_generator(&project_path_str)?
            .iter()
            .map(|cg| cg.manifest().id().unwrap())
            .collect::<Vec<_>>();
        ComponentsToDeploy::Multiple(components)
    };

    execute_deployment(
        log,
        &artifacts_path_str,
        components_to_deploy,
        opts.context,
        opts.wallet,
        opts.with_cycles,
        network,
        opts.port,
    )
    .await?;
    info!(
        log,
        r#"Project '{}' deployed successfully"#, project_manifest.label
    );

    Ok(())
}

fn check_before_deployment(
    log: &Logger,
    artifacts_path: &Path,
    port: Option<u16>,
    network: Network,
) -> anyhow::Result<()> {
    let dfx = DfxWrapper::new(
        match network {
            Network::Local => DfxWrapperNetwork::Local(port),
            Network::IC => DfxWrapperNetwork::IC,
        },
        Some(artifacts_path.to_str().unwrap().to_string()),
    );

    if let core::result::Result::Ok((dfx, version)) = dfx {
        info!(log, "Dfx version: {}", version);

        info!(log, "Running command: dfx ping");
        let ping_response = dfx.ping().map_err(|e| anyhow!(e))?;
        info!(log, "> {}", ping_response);
        info!(log, "Suceeded: dfx ping");

        info!(log, "Running command: dfx identity whoami");
        let whoami = dfx.identity_whoami().map_err(|e| anyhow!(e))?;
        info!(log, "> {}", whoami);
        info!(log, "Suceeded: dfx identity whoami");

        info!(log, "Running command: dfx identity get-principal");
        let principal = dfx.identity_get_principal().map_err(|e| anyhow!(e))?;
        info!(log, "> {}", principal);
        info!(log, "Suceeded: dfx identity get-principal");

        info!(log, "Running command: dfx identity get-wallet");
        let wallet = dfx.identity_get_wallet().map_err(|e| anyhow!(e))?;
        info!(log, "> {}", wallet);
        info!(log, "Suceeded: dfx identity get-wallet");
    } else {
        info!(log, "Dfx version: Not Found");
    };

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn execute_deployment(
    log: &Logger,
    artifacts_path_str: &str,
    components_to_deploy: ComponentsToDeploy,
    identity_context: Option<String>,
    wallet: Option<Option<String>>,
    with_cycles: Option<u128>,
    network: Network,
    port: Option<u16>,
) -> anyhow::Result<()> {
    // Execute
    let caller_identity = identity_from_context(identity_context.clone())?;
    let caller_principal = caller_identity.sender().map_err(|e| anyhow!(e))?;
    let wallet_principal = match wallet {
        Some(canister_id) => Some(if let Some(canister_id) = canister_id {
            Principal::from_text(canister_id).map_err(|e| anyhow!(e))?
        } else {
            get_wallet_principal_from_local_context(&network, port, identity_context).await?
        }),
        _ => None,
    };

    //// for saving component ids
    let dfx_bin_network = match network {
        Network::Local => DfxWrapperNetwork::Local(port),
        Network::IC => DfxWrapperNetwork::IC,
    };
    let (components, mut comp_id_mgr) = match components_to_deploy {
        ComponentsToDeploy::Single(val) => {
            let comp_id_mgr = ComponentIdsManager::load(&dfx_bin_network, artifacts_path_str)
                .unwrap_or_else(|_| ComponentIdsManager::new(&dfx_bin_network));
            (vec![val], comp_id_mgr)
        }
        ComponentsToDeploy::Multiple(val) => (val, ComponentIdsManager::new(&dfx_bin_network)),
    };

    // Check if the component has already been deployed
    for name in &components {
        if comp_id_mgr.contains_key(name) {
            anyhow::bail!(
                "The component '{}' has already been deployed. If you want to redeploy, delete the component id of the same name from `component_ids_(network).json`.",
                name
            );
        }
    }

    let mut name_and_ids = vec![];
    for name in components {
        let deploy_dest_id = functions::canister_create(
            Box::new(caller_identity.clone()),
            &wallet_principal,
            &network,
            port,
            with_cycles,
        )
        .await?;
        info!(log, "Created Canister ID: {} > {}", &name, &deploy_dest_id);
        name_and_ids.push((name.clone(), deploy_dest_id));
        comp_id_mgr.add(name, deploy_dest_id.to_text());
        comp_id_mgr.save(artifacts_path_str)?; // note: save every time to ensure that no results are lost along the way due to execution failures.
    }

    for (name, deploy_dest_id) in &name_and_ids {
        let wasm_path = format!("{}/{}.wasm", artifacts_path_str, name);
        functions::canister_install(
            &wasm_path,
            *deploy_dest_id,
            Box::new(caller_identity.clone()),
            &wallet_principal,
            &network,
            port,
        )
        .await?;
        info!(log, "Installed Module: {}", &wasm_path);
    }

    let env = match network {
        Network::Local => Env::LocalDevelopment,
        Network::IC => Env::Production,
    };
    for (name, deploy_dest_id) in &name_and_ids {
        functions::canister_update_settings(
            *deploy_dest_id,
            vec![caller_principal, env.initializer()],
            Box::new(caller_identity.clone()),
            &wallet_principal,
            &network,
            port,
        )
        .await?;
        info!(
            log,
            "Added management-canister to component's controllers: {}", &name
        );
    }

    Ok(())
}
