use color_eyre::Result;

use crate::command::Command;
use crate::core::Core;
use crate::event::Event;

/// Bootstrap - Initialize the application on startup
///
/// This actor is executed once at application startup to:
/// - Start background tasks (e.g., periodic time update)
/// - Initialize state
/// - Perform any other startup actions
pub struct Bootstrap;

impl Bootstrap {
   pub fn execute(core: &mut Core) -> Result<Vec<Command>> {
      // Start periodic time updater (every 1 second)
      let interval = std::time::Duration::from_secs(5);

      core.tasks.spawn_periodic(interval, move || {
         async move {
            let now = chrono::Local::now();
            let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();

            // Emit Command event instead of writing to core directly
            Event::Call(Command::new("update_time").arg(formatted)).emit();

            Ok(())
         }
      });

      // Could add other initialization tasks here
      // e.g., initial data fetch, config loading, etc.

      Ok(vec![])
   }
}
