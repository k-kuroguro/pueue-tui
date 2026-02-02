use crate::{actors::Actor, command::Command, core::Core, event::NEED_RENDER};

/// UpdateTime actor - Handle time update from background tasks
pub struct UpdateTime;

impl Actor for UpdateTime {
   const NAME: &'static str = "update_time";

   type Options = String;

   fn act(core: &mut Core, time: Self::Options) -> color_eyre::Result<()> {
      *core.current_time.write() = time;
      NEED_RENDER.store(true, std::sync::atomic::Ordering::Relaxed);
      Ok(())
   }
}

impl TryFrom<&Command> for String {
   type Error = color_eyre::Report;

   fn try_from(cmd: &Command) -> Result<Self, Self::Error> {
      cmd.first_arg()
         .ok_or_else(|| color_eyre::eyre::eyre!("Missing time argument"))
         .map(|s| s.to_string())
   }
}
