use color_eyre::eyre::Ok;
use crossterm::event::{KeyEvent, KeyEventKind, MouseEvent};

use crate::{app::App, event::Event, executor::Executor};

pub struct Dispatcher<'a> {
   app: &'a mut App,
}

impl<'a> Dispatcher<'a> {
   pub fn new(app: &'a mut App) -> Self {
      Self { app }
   }

   pub fn dispatch(&mut self, event: Event) -> color_eyre::Result<()> {
      match event {
         // System events - handled directly by App for performance
         Event::Resize => self.app.resize(),
         Event::Focus => self.app.focus(),
         Event::Paste(content) => self.app.paste(content),
         Event::Render => self.app.render_request(),
         Event::Quit => self.app.quit(),
         Event::Error(error) => self.dispatch_error(error),

         // Application events
         Event::Call(cmd) => self.dispatch_call(cmd),
         Event::Key(key) => self.dispatch_key(key),
         Event::Mouse(mouse) => self.dispatch_mouse(mouse),
      }
   }

   /// Handle command calls through the executor
   fn dispatch_call(&mut self, cmd: crate::command::Command) -> color_eyre::Result<()> {
      Executor::new(&mut self.app.core).execute(cmd)
   }

   /// Handle key events by routing them through the keymap
   fn dispatch_key(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
      // Only handle key press events, ignore release and repeat
      if key.kind != KeyEventKind::Press {
         return Ok(());
      }

      // Route the key through the keymap for the current layer
      let layer = self.app.core.layer();
      if let Some(cmd) = self.app.core.keymap.get(layer, &key) {
         self.dispatch_call(cmd)?;
      }

      Ok(())
   }

   fn dispatch_mouse(&mut self, _mouse: MouseEvent) -> color_eyre::Result<()> {
      // TODO: Implement mouse handling
      Ok(())
   }

   /// System event: error occurred
   fn dispatch_error(&mut self, _error: color_eyre::Report) -> color_eyre::Result<()> {
      // TODO: Handle errors properly (show notification, log, etc.)
      Ok(())
   }
}
