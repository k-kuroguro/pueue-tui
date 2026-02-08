use crate::{call, command::Command, commands::FetchLog, core::Core, event::Event};

pub struct ToggleLogPreview;

impl Command for ToggleLogPreview {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      core.log_preview = !core.log_preview;
      core.list_state.0 = core.list_state.0.with_offset(0);
      if core.log_preview {
         call!(FetchLog);
      }
      Event::Render.emit();
      Ok(())
   }
}
