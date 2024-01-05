use std::{collections::BTreeMap, fmt, fs::File, io, path::Path};

use anyhow::{bail, Ok};
use chainsight_cdk::core::Env;
use clap::Parser;
use slog::{debug, info, Logger};

use crate::{
    commands::utils::{output_by_exec_cmd, DfxArgsBuilder},
    lib::{
        codegen::{components::codegen::CodeGenerator, project::ProjectManifestData},
        environment::EnvironmentImpl,
        utils::{ARTIFACTS_DIR, PROJECT_MANIFEST_FILENAME},
    },
    types::Network,
};

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

    /// Specify the network to execute on.
    #[arg(long)]
    #[clap(default_value = "local")]
    network: Network,

    /// Specifies the port to call.
    /// This option is used only if the target is localhost.
    #[arg(long)]
    port: Option<u16>,

    /// Specify the initial number of cycles for canister.
    /// Used as a parameter for `dfx canister create`.
    #[arg(long)]
    with_cycles: Option<u64>,
}

pub fn exec(env: &EnvironmentImpl, opts: DeployOpts) -> anyhow::Result<()> {
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
    execute_deployment(
        log,
        &artifacts_path_str,
        project_manifest.load_code_generator(&project_path_str)?,
        &opts.component,
        opts.with_cycles,
        network,
    )?;
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
    let local_subnet = format!("http://127.0.0.1:{}", port.unwrap_or(4943));

    exec_command(
        log,
        "dfx",
        artifacts_path,
        match network {
            Network::Local => vec!["ping", &local_subnet],
            Network::IC => vec!["ping", "ic"],
        },
    )?;

    let exec = |args: Vec<&str>| -> anyhow::Result<String> {
        exec_command(log, "dfx", artifacts_path, args)
    };
    let args_builder = DfxArgsBuilder::new_only_network(network);
    exec(args_builder.generate(vec!["identity", "whoami"]))?;
    exec(args_builder.generate(vec!["identity", "get-principal"]))?;
    exec(args_builder.generate(vec!["identity", "get-wallet"]))?;

    Ok(())
}

fn execute_deployment(
    log: &Logger,
    artifacts_path_str: &str,
    generators: Vec<Box<dyn CodeGenerator>>,
    component: &Option<String>,
    with_cycles: Option<u64>,
    network: Network,
) -> anyhow::Result<()> {
    let artifacts_path = Path::new(&artifacts_path_str);

    let exec = |args: Vec<&str>| -> anyhow::Result<String> {
        exec_command(log, "dfx", artifacts_path, args)
    };
    let args_builder = DfxArgsBuilder::new(network.clone(), component.clone());

    // Check before deployments
    {
        let canister_info = get_canister_info(log, artifacts_path_str, network.clone());
        if let anyhow::Result::Ok(canister_info) = canister_info {
            let target_component_names = if let Some(component) = component {
                vec![component.to_string()]
            } else {
                canister_names_in_dfx_json(artifacts_path_str)?
            };
            let mut installed = Vec::<String>::new();
            let mut msg = String::new();
            for name in target_component_names {
                let (is_created, is_installed) = canister_info.status(&name);
                msg.push_str(&format!("Canister Name: {}\n", name));
                msg.push_str(&format!("  Created: {}\n", is_created));
                msg.push_str(&format!("  Installed: {}\n", is_installed));
                if is_installed {
                    installed.push(name);
                }
            }
            debug!(log, "Current deployed status:\n{}", msg);
            if !installed.is_empty() {
                bail!("Already installed: {:?}", installed);
            }
        }
    }

    // Execute
    if let Some(cycles) = with_cycles {
        exec(args_builder.generate(vec![
            "canister",
            "create",
            "--with-cycles",
            &cycles.to_string(),
        ]))?;
    } else {
        exec(args_builder.generate(vec!["canister", "create"]))?;
    }
    exec(args_builder.generate(vec!["build"]))?;
    exec(args_builder.generate(vec!["canister", "install"]))?;

    let env = match network {
        Network::Local => Env::LocalDevelopment,
        Network::IC => Env::Production,
    };
    for generator in generators.iter() {
        let id = generator.manifest().id().unwrap();
        if let Some(target) = component {
            if target != &id {
                continue;
            }
        }
        let builder = DfxArgsBuilder::new(network.clone(), Some(id.clone()));

        exec(builder.generate(vec![
            "canister",
            "update-settings",
            "--add-controller",
            &env.initializer().to_string(),
        ]))?;
    }

    // Check after deployments
    let canister_info = get_canister_info(log, artifacts_path_str, network.clone())?;
    info!(log, "Current deployed status:\n{}", canister_info);

    Ok(())
}

fn exec_command(
    log: &Logger,
    cmd: &str,
    execution_dir: &Path,
    args: Vec<&str>,
) -> anyhow::Result<String> {
    let cmd_string = format!("{} {}", cmd, args.join(" "));
    info!(log, "Running command: '{}'", cmd_string);

    let output = output_by_exec_cmd(cmd, execution_dir, args)
        .unwrap_or_else(|_| panic!("failed to execute process: {}", cmd_string));
    if output.status.success() {
        let stdout = std::str::from_utf8(&output.stdout);
        debug!(log, "{}", stdout.unwrap_or("failed to parse stdout"));
        info!(log, "Suceeded: {}", cmd_string);
        anyhow::Ok(stdout.unwrap_or_default().to_string())
    } else {
        bail!(format!(
            "Failed: {} by: {} ",
            cmd_string,
            std::str::from_utf8(&output.stderr).unwrap_or("failed to parse stderr")
        ));
    }
}

type CanisterInfoControllers = String;
type CanisterInfoModuleHash = String;
#[derive(Clone)]
struct CanisterInfo(CanisterInfoControllers, CanisterInfoModuleHash);
impl CanisterInfo {
    fn controllers(&self) -> &CanisterInfoControllers {
        &self.0
    }
    fn module_hash(&self) -> &CanisterInfoModuleHash {
        &self.1
    }
}
impl fmt::Display for CanisterInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Controllers: {}", self.controllers())?;
        writeln!(f, "  Module Hash: {}", self.module_hash())
    }
}
#[derive(Clone, Default)]
struct CanistersInfo(BTreeMap<CanisterName, (CanisterIdString, Option<CanisterInfo>)>);
impl CanistersInfo {
    fn get(&self, name: &str) -> Option<&(CanisterIdString, Option<CanisterInfo>)> {
        self.0.get(name)
    }
    fn info(&self, name: &str) -> Option<CanisterInfo> {
        self.get(name).unwrap().1.clone()
    }
    fn status(&self, name: &str) -> (bool, bool) {
        let id = self.get(name);
        if id.is_none() {
            (false, false)
        } else {
            let info = self.info(name);
            let installed = if let Some(info) = info {
                !info.module_hash().eq("None") // HACK: Do not hardcode 'None'
            } else {
                false
            };
            (true, installed)
        }
    }
}
impl fmt::Display for CanistersInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, (id, info_opt)) in &self.0 {
            writeln!(f, "Canister Name: {}", name)?;
            writeln!(f, "  Canister Id: {}", id)?;
            if let Some(info) = info_opt {
                writeln!(f, "{}", info)?;
            }
        }
        fmt::Result::Ok(())
    }
}
fn get_canister_info(
    log: &Logger,
    artifacts_path_str: &str,
    network: Network,
) -> anyhow::Result<CanistersInfo> {
    let artifacts_path = Path::new(&artifacts_path_str);
    let ids = read_canister_ids_json(artifacts_path_str, network.clone())?;
    let mut result = CanistersInfo::default();
    for (name, ids) in ids {
        // HACK: check to use 'network' name as key
        if !ids.is_empty() {
            let info = call_canister_info(log, artifacts_path, &name, network.clone());
            result.0.insert(name, (format!("{:?}", ids), info));
        }
    }
    Ok(result)
}
fn call_canister_info(
    log: &Logger,
    artifacts_path: &Path,
    canister_id_or_name: &str,
    network: Network,
) -> Option<CanisterInfo> {
    let args_builder = DfxArgsBuilder::new(network, Some(canister_id_or_name.to_string()));
    let output = output_by_exec_cmd(
        "dfx",
        artifacts_path,
        args_builder.generate(vec!["canister", "info"]),
    );
    if let io::Result::Ok(output) = output {
        let msg = std::str::from_utf8(&output.stdout).expect("failed to parse stdout");
        let lines = msg.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        // NOTE: occur immediately after resetting node in local
        if lines.is_empty() {
            return None;
        }
        let controllers = lines[0].split(':').last().unwrap().trim().to_string();
        let module_hash = lines[1].split(':').last().unwrap().trim().to_string();
        Some(CanisterInfo(controllers, module_hash))
    } else {
        debug!(log, "Failed to call canister info: {}", canister_id_or_name);
        None
    }
}

type CanisterName = String;
type NetworkName = String;
type CanisterIdString = String;
type CanisterIds = BTreeMap<CanisterName, BTreeMap<NetworkName, CanisterIdString>>;

fn read_canister_ids_json(
    artifacts_path_str: &str,
    network: Network,
) -> anyhow::Result<CanisterIds> {
    let json_path = canister_ids_json_path(artifacts_path_str, network);
    let json = File::open(json_path)?;
    let canister_ids: CanisterIds = serde_json::from_reader(json)?;
    Ok(canister_ids)
}
fn canister_ids_json_path(artifacts_path_str: &str, network: Network) -> String {
    let json_filename = "canister_ids.json";
    match network {
        Network::Local => format!("{}/.dfx/local/{}", artifacts_path_str, json_filename),
        Network::IC => format!("{}/{}", artifacts_path_str, json_filename),
    }
}

fn canister_names_in_dfx_json(artifacts_path_str: &str) -> anyhow::Result<Vec<CanisterName>> {
    let dfx_json_path = format!("{}/dfx.json", artifacts_path_str);
    let dfx_json_file = File::open(dfx_json_path)?;
    let dfx_json: serde_json::Value = serde_json::from_reader(dfx_json_file)?;
    let canisters = dfx_json.get("canisters");
    if let Some(canisters) = canisters {
        let canisters = canisters.as_object().expect("failed to parse canisters");
        let names = canisters.keys().map(|s| s.to_string()).collect::<Vec<_>>();
        Ok(names)
    } else {
        bail!("Failed to parse dfx.json")
    }
}
