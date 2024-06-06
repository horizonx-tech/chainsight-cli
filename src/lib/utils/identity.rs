use std::{collections::BTreeMap, env, fs, path::PathBuf};

use anyhow::Context;
use candid::Principal;
use ic_agent::identity::Secp256k1Identity;
use serde::{Deserialize, Serialize};

const DFX_CONFIG_ROOT_PATH: &str = ".config/dfx";

// ref: dfinity/sdk/src/dfx-core/src/identity/mod.rs
const IDENTITY_JSON: &str = "identity.json";
const WALLET_CONFIG_FILENAME: &str = "wallets.json";

// keyring
// ref: dfinity/sdk/src/dfx-core/src/identity/keyring_mock.rs
const KEYRING_SERVICE_NAME: &str = "internet_computer_identities";
const KEYRING_IDENTITY_PREFIX: &str = "internet_computer_identity_";

// ref: dfinity/sdk/src/dfx-core/src/identity/identity_manager.rs
// (config root)/identity.json
#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    pub default: String,
}
// (config root)/identity/(identity-name)/identity.json
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IdentityConfiguration {
    pub hsm: Option<String>,        // temp
    pub encryption: Option<String>, // temp
    pub keyring_identity_suffix: Option<String>,
}

// ref: dfinity/sdk/src/dfx-core/src/identity/mod.rs
// (config root)/identity/(identity-name)/wallets.json
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletNetworkMap {
    #[serde(flatten)]
    pub networks: BTreeMap<String, Principal>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGlobalConfig {
    pub identities: BTreeMap<String, WalletNetworkMap>,
}

pub fn identity_from_keyring(context_name: Option<String>) -> anyhow::Result<Secp256k1Identity> {
    let context_name = context_name.unwrap_or(default_identity_context()?);
    let entry = keyring::Entry::new(
        KEYRING_SERVICE_NAME,
        &format!("{}{}", KEYRING_IDENTITY_PREFIX, context_name),
    )?;
    let password = entry.get_password()?;

    let pem = hex::decode(password.clone())?;

    let identity = Secp256k1Identity::from_pem(pem.as_slice())?;
    Ok(identity)
}

fn default_identity_context() -> anyhow::Result<String> {
    let path_str = format!("~/{}/{}", DFX_CONFIG_ROOT_PATH, IDENTITY_JSON);
    let path = get_path_to_home(&path_str).context(format!("Not found: {}", &path_str))?;
    let identity_json: Configuration = serde_json::from_str(&fs::read_to_string(path)?)?;
    Ok(identity_json.default)
}

fn get_home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
fn get_path_to_home(path: &str) -> Option<PathBuf> {
    if path.starts_with('~') {
        get_home_dir().map(|home| home.join(path.trim_start_matches("~/")))
    } else {
        Some(PathBuf::from(path))
    }
}
