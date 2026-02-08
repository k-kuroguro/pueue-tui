use crate::call;
use crate::command::Command;
use crate::commands::UpdateTasks;
use crate::core::Core;

pub struct Bootstrap;

impl Command for Bootstrap {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      let interval = std::time::Duration::from_secs(1);

      let client = core.pueue_client.clone();

      core.tasks.spawn_periodic(interval, move || {
         let client = client.clone();
         async move {
            if let Ok(state) = client.status().await {
               call!(UpdateTasks::new(state.tasks.values().cloned().collect()));
            }
         }
      });

      Ok(())
   }
}
