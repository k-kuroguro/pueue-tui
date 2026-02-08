use crate::app::App;
use crate::cli::CliArgs;

mod app;
mod cli;
mod client;
mod command;
mod commands;
mod core;
mod dispatcher;
mod event;
mod keymap;
mod tasks;
mod terminal;
mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
   color_eyre::install()?;

   let args = CliArgs::parse();
   let mut app = App::new(&args).await?;
   app.run().await?;

   Ok(())
}
