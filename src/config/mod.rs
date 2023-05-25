use lazy_static::lazy_static;
use semver::Version;

lazy_static! {
    // This expect cannot happen, we make sure that CARGO_PKG_VERSION is correct.
    static ref VERSION: Version =
        Version::parse(env!("CARGO_PKG_VERSION")).expect("Cannot parse version.");

    static ref VERSION_STR: String = env!("CARGO_PKG_VERSION").to_string();
}

pub fn cli_version() -> &'static Version {
    &VERSION
}

pub fn cli_version_str() -> &'static str {
    &VERSION_STR
}
