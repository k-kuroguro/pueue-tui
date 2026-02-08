use std::sync::Arc;

use crate::{command::Command, core::Core, event::Event};

pub struct UpdateTasks {
   tasks: Arc<[pueue_lib::Task]>,
}

impl UpdateTasks {
   pub fn new(tasks: Arc<[pueue_lib::Task]>) -> Self {
      Self { tasks }
   }
}

impl Command for UpdateTasks {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      core.pueue_tasks = Arc::clone(&self.tasks);
      Event::Render.emit();
      Ok(())
   }
}
