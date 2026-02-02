use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{command::Command, core::Layer};

/// Keymap manages the mapping from key events to commands per layer.
/// In the future, this can be loaded from a configuration file.
#[derive(Debug, Clone)]
pub struct Keymap {
   bindings: HashMap<(Layer, KeyCode, KeyModifiers), Command>,
}

impl Keymap {
   /// Create a new keymap with default bindings
   pub fn new() -> Self {
      macro_rules! bind {
         ($layer:expr, $key:expr, $cmd:expr) => {
            (($layer, $key, KeyModifiers::NONE), $cmd)
         };
         ($layer:expr, $key:expr, $mods:expr, $cmd:expr) => {
            (($layer, $key, $mods), $cmd)
         };
      }
      
      let bindings = HashMap::from([
         // === List Layer ===
         // Quit
         bind!(Layer::List, KeyCode::Char('q'), Command::new("quit")),
         bind!(Layer::List, KeyCode::Char('c'), KeyModifiers::CONTROL, Command::new("quit")),
         
         // Navigation
         bind!(Layer::List, KeyCode::Char('j'), Command::new("arrow").arg("1")),
         bind!(Layer::List, KeyCode::Down, Command::new("arrow").arg("1")),
         bind!(Layer::List, KeyCode::Char('k'), Command::new("arrow").arg("-1")),
         bind!(Layer::List, KeyCode::Up, Command::new("arrow").arg("-1")),
         
         // Goto
         bind!(Layer::List, KeyCode::Char('g'), Command::new("goto").arg("top")),
         bind!(Layer::List, KeyCode::Char('G'), KeyModifiers::SHIFT, Command::new("goto").arg("bottom")),
         
         // Actions
         bind!(Layer::List, KeyCode::Char('r'), Command::new("refresh")),
         bind!(Layer::List, KeyCode::Char('?'), Command::new("help")),
         bind!(Layer::List, KeyCode::Char('p'), Command::new("toggle_preview")),
         bind!(Layer::List, KeyCode::Char('/'), Command::new("search")),
         
         // === Search Layer ===
         bind!(Layer::Search, KeyCode::Esc, Command::new("close")),
         bind!(Layer::Search, KeyCode::Enter, Command::new("accept")),
         bind!(Layer::Search, KeyCode::Char('c'), KeyModifiers::CONTROL, Command::new("close")),
         
         // === Help Layer ===
         bind!(Layer::Help, KeyCode::Esc, Command::new("close")),
         bind!(Layer::Help, KeyCode::Char('q'), Command::new("close")),
         bind!(Layer::Help, KeyCode::Char('?'), Command::new("close")),
         bind!(Layer::Help, KeyCode::Char('j'), Command::new("arrow").arg("1")),
         bind!(Layer::Help, KeyCode::Down, Command::new("arrow").arg("1")),
         bind!(Layer::Help, KeyCode::Char('k'), Command::new("arrow").arg("-1")),
         bind!(Layer::Help, KeyCode::Up, Command::new("arrow").arg("-1")),
      ]);

      Self { bindings }
   }

   /// Get the command for a key event in the current layer
   pub fn get(&self, layer: Layer, key: &KeyEvent) -> Option<Command> {
      self.bindings.get(&(layer, key.code, key.modifiers)).cloned()
   }

   /// Add or update a key binding for a specific layer
   pub fn bind(&mut self, layer: Layer, key_code: KeyCode, modifiers: KeyModifiers, command: Command) {
      self.bindings.insert((layer, key_code, modifiers), command);
   }

   /// Remove a key binding from a specific layer
   pub fn unbind(&mut self, layer: Layer, key_code: KeyCode, modifiers: KeyModifiers) {
      self.bindings.remove(&(layer, key_code, modifiers));
   }
}

impl Default for Keymap {
   fn default() -> Self {
      Self::new()
   }
}
