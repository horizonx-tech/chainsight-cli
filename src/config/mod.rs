use lazy_static::lazy_static;
use semver::Version;
use std::process::Command;

lazy_static! {
    // This expect cannot happen, we make sure that CARGO_PKG_VERSION is correct.
    static ref VERSION: Version =
        Version::parse(env!("CARGO_PKG_VERSION")).expect("Cannot parse version.");
    static ref VERSION_STR: String = format!("version = {}, commit-hash = {}", env!("CARGO_PKG_VERSION"), git_commit_hash());
}

#[allow(dead_code)]
pub fn cli_version() -> &'static Version {
    &VERSION
}

pub fn cli_version_str() -> &'static str {
    &VERSION_STR
}

fn git_commit_hash() -> String {
    let commit_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute command \"git rev-parse HEAD\"");

    String::from_utf8(commit_hash.stdout.to_vec())
        .unwrap()
        .replace('\n', "")
}
