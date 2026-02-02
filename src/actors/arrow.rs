use crate::actors::Actor;
use crate::command::Command;
use crate::core::Core;

pub struct Arrow;

pub struct ArrowOptions(pub i32);

impl TryFrom<&Command> for ArrowOptions {
   type Error = color_eyre::Report;

   fn try_from(cmd: &Command) -> color_eyre::Result<Self> {
      Ok(ArrowOptions(cmd.first_i32().unwrap_or(0)))
   }
}

impl Actor for Arrow {
   const NAME: &'static str = "arrow";

   type Options = ArrowOptions;

   fn act(_core: &mut Core, _step: Self::Options) -> color_eyre::Result<()> {
      // TODO: Implement cursor movement logic

      // In a real implementation, this would:
      // 1. Update the cursor position in the current view
      // 2. Handle boundary conditions (top/bottom of list)
      // 3. Trigger related actors (hover, peek, etc.)

      Ok(())
   }
}
