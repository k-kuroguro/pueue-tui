use parking_lot::RwLock;
use std::sync::Arc;

use crate::keymap::Keymap;
use crate::tasks::Tasks;

/// Layer represents different UI modes/screens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
   /// Main task list view
   List,
   /// Search mode (input prompt + filtered results)
   Search,
   /// Help popup overlay
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

   // Layer management (stack-based like yazi)
   layer_stack: Vec<Layer>,

   // UI state
   pub preview_visible: bool,
   pub search_query: String,

   // Background tasks
   pub tasks: Tasks,

   // Current time (updated by background task)
   pub current_time: Arc<RwLock<String>>,
}

impl Core {
   pub fn new() -> Self {
      let current_time = Arc::new(RwLock::new(String::new()));

      Self {
         should_quit: false,
         keymap: Keymap::new(),
         layer_stack: vec![Layer::List],
         preview_visible: false,
         search_query: String::new(),
         tasks: Tasks::new(),
         current_time,
      }
   }

   /// Get the current active layer
   pub fn layer(&self) -> Layer {
      *self.layer_stack.last().unwrap_or(&Layer::List)
   }

   /// Push a new layer onto the stack (e.g., opening help or search)
   pub fn push_layer(&mut self, layer: Layer) {
      self.layer_stack.push(layer);
   }

   /// Pop the current layer and return to the previous one
   pub fn pop_layer(&mut self) -> Layer {
      if self.layer_stack.len() > 1 {
         self.layer_stack.pop().unwrap()
      } else {
         // Keep at least one layer (List)
         Layer::List
      }
   }

   /// Replace the current layer (for switching between main screens)
   pub fn switch_layer(&mut self, layer: Layer) {
      if let Some(last) = self.layer_stack.last_mut() {
         *last = layer;
      }
   }

   /// Toggle preview visibility
   pub fn toggle_preview(&mut self) {
      self.preview_visible = !self.preview_visible;
   }
}
