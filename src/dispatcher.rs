use crossterm::event::{KeyEvent, KeyEventKind, MouseEvent};

use crate::{app::App, event::Event};

pub struct Dispatcher<'a> {
   app: &'a mut App,
}

impl<'a> Dispatcher<'a> {
   pub fn new(app: &'a mut App) -> Self {
      Self { app }
   }

   pub fn dispatch(&mut self, event: Event) -> color_eyre::Result<()> {
      match event {
         Event::Call(cmd) => cmd.execute(&mut self.app.core),
         Event::Key(key) => self.dispatch_key(key),
         Event::Mouse(mouse) => self.dispatch_mouse(mouse),
         Event::Resize => self.app.resize(),
         Event::Focus => self.app.focus(),
         Event::Paste(content) => self.app.paste(content),
         Event::Render => self.app.request_render(),
         Event::Error(error) => self.dispatch_error(error),
      }
   }

   fn dispatch_key(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
      if key.kind != KeyEventKind::Press {
         return Ok(());
      }

      let layer = self.app.core.layer();
      if let Some(cmd) = self.app.core.keymap.get(layer, &key) {
         cmd.execute(&mut self.app.core)?;
      }

      Ok(())
   }

   fn dispatch_mouse(&mut self, _mouse: MouseEvent) -> color_eyre::Result<()> {
      Ok(())
   }

   fn dispatch_error(&mut self, _error: color_eyre::Report) -> color_eyre::Result<()> {
      Ok(())
   }
}
