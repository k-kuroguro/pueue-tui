use parking_lot::RwLock;
use std::sync::Arc;
use tokio::{
   task::JoinHandle,
   time::{Duration, sleep},
};

/// Background task manager
///
/// Manages long-running background tasks with proper cleanup on shutdown.
pub struct Tasks {
   handles: Vec<JoinHandle<()>>,
}

impl Tasks {
   pub fn new() -> Self {
      Self {
         handles: Vec::new(),
      }
   }

   /// Spawn a periodic task that runs a closure at a fixed interval
   ///
   /// The task will continue running until shutdown or the closure returns an error.
   ///
   /// # Example
   /// ```no_run
   /// tasks.spawn_periodic(Duration::from_secs(1), || async {
   ///     // Update something
   ///     Ok(())
   /// });
   /// ```
   pub fn spawn_periodic<F, Fut>(&mut self, interval: Duration, mut f: F)
   where
      F: FnMut() -> Fut + Send + 'static,
      Fut: std::future::Future<Output = color_eyre::Result<()>> + Send + 'static,
   {
      let handle = tokio::spawn(async move {
         loop {
            sleep(interval).await;

            if let Err(e) = f().await {
               eprintln!("Periodic task error: {}", e);
               break;
            }
         }
      });

      self.handles.push(handle);
   }

   /// Spawn a one-shot background task
   ///
   /// The task runs once and completes. Useful for async initialization or
   /// fire-and-forget operations.
   pub fn spawn<F>(&mut self, future: F)
   where
      F: std::future::Future<Output = ()> + Send + 'static,
   {
      let handle = tokio::spawn(future);
      self.handles.push(handle);
   }

   /// Shutdown all background tasks
   pub fn shutdown(&self) {
      for handle in &self.handles {
         handle.abort();
      }
   }
}

impl Drop for Tasks {
   fn drop(&mut self) {
      self.shutdown();
   }
}
