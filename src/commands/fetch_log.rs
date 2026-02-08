use std::sync::Arc;

use crate::{call, command::Command, commands::UpdateLog, core::Core};

pub struct FetchLog;

impl Command for FetchLog {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      let client = core.pueue_client.clone();
      let task_id = core.selected_task_id();
      core.tasks.spawn({
         let client = client.clone();
         async move {
            if let Some(id) = task_id {
               if let Ok(log) = client.log(id).await {
                  call!(UpdateLog::new(id, log));
               } else {
                  //TODO: Handle error properly.
                  call!(UpdateLog::new(id, Arc::new([])));
               }
            }
         }
      });
      Ok(())
   }
}
