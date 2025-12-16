use clap::Parser;
use cli::Cli;

use crate::app::App;

mod action;
mod app;
mod cli;
mod components;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
   let args = Cli::parse();
   let mut app = App::new()?;
   app.run().await?;
   Ok(())
}
