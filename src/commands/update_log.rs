use std::sync::Arc;

use crate::{command::Command, core::Core, event::Event};

pub struct UpdateLog {
   task_id: usize,
   log: Arc<[u8]>,
}

impl UpdateLog {
   pub fn new(task_id: usize, log: Arc<[u8]>) -> Self {
      Self { task_id, log }
   }
}

impl Command for UpdateLog {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      core.pueue_log = Some((self.task_id, Arc::clone(&self.log)));
      Event::Render.emit();
      Ok(())
   }
}
