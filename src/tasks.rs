use tokio::{
   task::JoinHandle,
   time::{Duration, sleep},
};

pub struct Tasks {
   handles: Vec<JoinHandle<()>>,
}

impl Tasks {
   pub fn new() -> Self {
      Self {
         handles: Vec::new(),
      }
   }

   pub fn spawn<F>(&mut self, f: F)
   where
      F: Future<Output = ()> + Send + 'static,
   {
      let handle = tokio::spawn(f);
      self.handles.push(handle);
   }

   pub fn spawn_periodic<F, Fut>(&mut self, interval: Duration, mut f: F)
   where
      F: FnMut() -> Fut + Send + 'static,
      Fut: Future<Output = ()> + Send + 'static,
   {
      let handle = tokio::spawn(async move {
         loop {
            f().await;
            sleep(interval).await;
         }
      });

      self.handles.push(handle);
   }

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
