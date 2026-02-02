use crate::{
   actors::{Actor, arrow::Arrow, update_time::UpdateTime},
   command::Command,
   core::Core,
};

pub struct Executor<'a> {
   core: &'a mut Core,
}

impl<'a> Executor<'a> {
   pub fn new(core: &'a mut Core) -> Self {
      Self { core }
   }

   /// Execute a command by dispatching it to the appropriate actor or handler
   pub fn execute(&mut self, cmd: Command) -> color_eyre::Result<()> {
      macro_rules! try_actor {
         ($actor:ty) => {
            if cmd.name == <$actor>::NAME {
               let opts = <$actor as Actor>::Options::try_from(&cmd)?;
               return <$actor>::act(self.core, opts);
            }
         };
      }

      try_actor!(Arrow);
      try_actor!(UpdateTime);

      // Special commands handled directly
      if cmd.name == "quit" {
         self.core.should_quit = true;
         return Ok(());
      }

      if cmd.name == "goto" {
         return self.goto(&cmd);
      }

      // Layer management
      if cmd.name == "help" {
         return self.help();
      }

      if cmd.name == "search" {
         return self.search();
      }

      if cmd.name == "close" {
         return self.close();
      }

      if cmd.name == "toggle_preview" {
         self.core.toggle_preview();
         return Ok(());
      }

      if cmd.name == "refresh" || cmd.name == "accept" {
         return Ok(());
      }

      // Unknown command - log it but don't fail
      #[cfg(debug_assertions)]
      eprintln!("Unknown command: {}", cmd.name);
      Ok(())
   }

   fn goto(&mut self, cmd: &Command) -> color_eyre::Result<()> {
      let target = cmd.first_arg().unwrap_or("");
      match target {
         "top" => {
            // TODO: Go to top of list
         }
         "bottom" => {
            // TODO: Go to bottom of list
         }
         _ => {
            // TODO: Parse line number
         }
      }
      Ok(())
   }

   fn help(&mut self) -> color_eyre::Result<()> {
      use crate::core::Layer;
      // Push help layer on top of current layer
      self.core.push_layer(Layer::Help);
      Ok(())
   }

   fn search(&mut self) -> color_eyre::Result<()> {
      use crate::core::Layer;
      self.core.search_query.clear();
      // Switch main layer to Search (not a popup)
      self.core.switch_layer(Layer::Search);
      Ok(())
   }

   fn close(&mut self) -> color_eyre::Result<()> {
      // Pop the current layer to return to previous
      self.core.pop_layer();
      Ok(())
   }
}
