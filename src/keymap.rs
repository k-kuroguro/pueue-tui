use std::{collections::HashMap, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
   command::Command,
   commands::{Arrow, Close, Help, Quit, ToggleLogPreview},
   core::Layer,
};

pub struct Keymap {
   bindings: HashMap<(Layer, KeyCode, KeyModifiers), Rc<dyn Command>>,
}

impl Keymap {
   pub fn new() -> Self {
      macro_rules! bind {
         ($layer:expr, $key:expr, $cmd:expr) => {
            (
               ($layer, $key, KeyModifiers::NONE),
               Rc::new($cmd) as Rc<dyn Command>,
            )
         };
         ($layer:expr, $key:expr, $mods:expr, $cmd:expr) => {
            (($layer, $key, $mods), Rc::new($cmd) as Rc<dyn Command>)
         };
      }

      let bindings: HashMap<(Layer, KeyCode, KeyModifiers), Rc<dyn Command>> = HashMap::from([
         bind!(Layer::List, KeyCode::Char('q'), Quit),
         bind!(Layer::List, KeyCode::Char('c'), KeyModifiers::CONTROL, Quit),
         bind!(Layer::List, KeyCode::Up, Arrow::up()),
         bind!(Layer::List, KeyCode::Down, Arrow::down()),
         bind!(Layer::List, KeyCode::Char('h'), Help),
         bind!(Layer::List, KeyCode::Char('l'), ToggleLogPreview),
         //
         bind!(Layer::Help, KeyCode::Char('q'), Quit),
         bind!(Layer::Help, KeyCode::Esc, Close),
         bind!(Layer::Help, KeyCode::Char('h'), Close),
      ]);

      Self { bindings }
   }

   pub fn get(&self, layer: Layer, key: &KeyEvent) -> Option<Rc<dyn Command>> {
      self
         .bindings
         .get(&(layer, key.code, key.modifiers))
         .cloned()
   }
}

impl Default for Keymap {
   fn default() -> Self {
      Self::new()
   }
}
