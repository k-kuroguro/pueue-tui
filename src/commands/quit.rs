use crate::{command::Command, core::Core};

pub struct Quit;

impl Command for Quit {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      core.should_quit = true;
      Ok(())
   }
}
