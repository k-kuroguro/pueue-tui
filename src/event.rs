use std::sync::{OnceLock, atomic::AtomicBool};

use crossterm::event::{KeyEvent, MouseEvent};
use tokio::sync::mpsc;

use crate::command::Command;

static TX: OnceLock<mpsc::UnboundedSender<Event>> = OnceLock::new();
pub static NEED_RENDER: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum Event {
   Call(Command),
   Key(KeyEvent),
   Mouse(MouseEvent),
   Resize,
   Focus,
   Paste(String),
   Render,
   Quit,
   Error(color_eyre::Report),
}

impl Event {
   pub fn init() -> color_eyre::Result<mpsc::UnboundedReceiver<Event>> {
      let (tx, rx) = mpsc::unbounded_channel();
      TX.set(tx)
         .map_err(|_| color_eyre::eyre::eyre!("Event channel already initialized."))?;
      Ok(rx)
   }

   pub fn emit(self) {
      if let Some(tx) = TX.get() {
         tx.send(self).ok();
      }
   }
}
