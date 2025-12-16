use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::action::Action;

#[derive(Default)]
pub struct Home {
   command_tx: Option<UnboundedSender<Action>>,
   elapsed_tick: u64,
}

impl Home {
   pub fn new() -> Self {
      Self::default()
   }
}

impl Component for Home {
   fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
      self.command_tx = Some(tx);
      Ok(())
   }

   fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
      match action {
         Action::Tick => self.elapsed_tick = self.elapsed_tick.wrapping_add(1),
         Action::Render => {
            // add any logic here that should run on every render
         }
         _ => {}
      }
      Ok(None)
   }

   fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
      frame.render_widget(
         Paragraph::new(format!("hello world, elapsed: {}", self.elapsed_tick)),
         area,
      );
      Ok(())
   }
}
