use clap::Parser;

use crate::app::App;
use crate::cli::CliArgs;

mod action;
mod app;
mod cli;
mod client;
mod components;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
   color_eyre::install()?;

   let args = CliArgs::parse();
   let mut app = App::new(&args).await?;
   app.run().await?;

   Ok(())
}
