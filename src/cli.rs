use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = version())]
pub struct Cli {}

const VERSION_MESSAGE: &str = env!("CARGO_PKG_VERSION");

fn version() -> String {
   VERSION_MESSAGE.to_string()
}
