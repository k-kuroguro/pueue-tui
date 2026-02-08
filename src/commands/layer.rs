use crate::{
   command::Command,
   core::{Core, Layer},
   event::Event,
};

pub struct Help;

impl Command for Help {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      core.push_layer(Layer::Help);
      Event::Render.emit();
      Ok(())
   }
}

pub struct Close;

impl Command for Close {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      core.pop_layer();
      Event::Render.emit();
      Ok(())
   }
}
