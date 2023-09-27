use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "config")]
/// Configure about parameters, filepath etc
pub struct ConfigOpts {}

#[cfg(test)]
mod tests {}
