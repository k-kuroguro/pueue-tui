use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(version = version())]
pub struct CliArgs {
   /// If provided, pueue-tui only uses this config file.
   ///
   /// This path can also be set via the "PUEUE_CONFIG_PATH" environment variable.
   /// The commandline option overwrites the environment variable!
   #[arg(short, long, value_hint = ValueHint::FilePath)]
   pub config: Option<PathBuf>,

   /// The name of the profile that should be loaded from your config file.
   #[arg(short, long)]
   pub profile: Option<String>,
}

const VERSION_MESSAGE: &str = env!("CARGO_PKG_VERSION");

fn version() -> String {
   VERSION_MESSAGE.to_string()
}
