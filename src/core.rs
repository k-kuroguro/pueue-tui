use parking_lot::RwLock;
use ratatui::widgets::{ScrollbarState, TableState};
use std::sync::Arc;

use crate::cli::CliArgs;
use crate::client::Client;
use crate::keymap::Keymap;
use crate::tasks::Tasks;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
   List,
   Help,
}

impl Default for Layer {
   fn default() -> Self {
      Self::List
   }
}

/// Core state of the application
pub struct Core {
   pub should_quit: bool,
   pub keymap: Keymap,

   layer_stack: Vec<Layer>,

   pub tasks: Tasks,

   pub pueue_tasks: Arc<[pueue_lib::Task]>,

   pub pueue_client: Client,
   pub pueue_log: Option<(usize, Arc<[u8]>)>,

   pub list_state: (TableState, ScrollbarState),

   pub log_preview: bool,
}

impl Core {
   pub async fn new(options: &CliArgs) -> color_eyre::Result<Self> {
      Ok(Self {
         should_quit: false,
         keymap: Keymap::new(),
         layer_stack: vec![Layer::List],
         tasks: Tasks::new(),
         pueue_tasks: Arc::new([]),
         pueue_client: Client::new(&options.config, &options.profile).await?,
         pueue_log: None,
         list_state: (TableState::new().with_selected(0), ScrollbarState::new(0)),
         log_preview: false,
      })
   }

   pub fn layers(&self) -> &[Layer] {
      &self.layer_stack
   }

   pub fn layer(&self) -> Layer {
      *self.layer_stack.last().unwrap_or(&Layer::List)
   }

   pub fn push_layer(&mut self, layer: Layer) {
      self.layer_stack.push(layer);
   }

   pub fn pop_layer(&mut self) -> Layer {
      if self.layer_stack.len() > 1 {
         self.layer_stack.pop().unwrap()
      } else {
         Layer::List
      }
   }

   pub fn selected_task_id(&self) -> Option<usize> {
      let selected = self.list_state.0.selected()?;
      self.pueue_tasks.get(selected).map(|task| task.id)
   }

   pub fn selected_task(&self) -> Option<&pueue_lib::Task> {
      let selected = self.list_state.0.selected()?;
      self.pueue_tasks.get(selected)
   }
}
