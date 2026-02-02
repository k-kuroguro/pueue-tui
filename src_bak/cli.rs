use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(version)]
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

impl CliArgs {
   pub fn parse() -> Self {
      <Self as Parser>::parse()
   }
}
