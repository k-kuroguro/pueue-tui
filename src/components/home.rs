use std::vec;

use crossterm::event::{KeyEvent, KeyModifiers};
use pueue_lib::Task;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, widgets::task_table::TaskTable};

#[derive(Default)]
pub struct Home {
   command_tx: Option<UnboundedSender<Action>>,
   table_state: TableState,
   scroll_state: ScrollbarState,
   tasks: Vec<Task>,
}

impl Home {
   pub fn new() -> Self {
      Self {
         command_tx: None,
         table_state: TableState::default().with_selected(0),
         scroll_state: ScrollbarState::new(0),
         tasks: vec![],
      }
   }
}

impl Component for Home {
   fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
      self.command_tx = Some(tx);
      Ok(())
   }

   fn handle_key_event(
      &mut self,
      key: crossterm::event::KeyEvent,
   ) -> color_eyre::Result<Option<Action>> {
      let KeyEvent {
         code, modifiers, ..
      } = key;

      match modifiers {
         KeyModifiers::NONE => match code {
            crossterm::event::KeyCode::Down => {
               self.next_row();
               return Ok(None);
            }
            crossterm::event::KeyCode::Up => {
               self.prev_row();
               return Ok(None);
            }
            _ => {}
         },
         _ => {}
      }

      Ok(None)
   }

   fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
      match action {
         Action::Tick => {}
         Action::Render => {}
         Action::UpdateStatus(state) => {
            self.tasks = state.tasks.values().cloned().collect();
            self.scroll_state = self.scroll_state; //.content_length(self.data.len());
         }
         _ => {}
      }
      Ok(None)
   }

   fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
      let table = TaskTable::new(&self.tasks);
      frame.render_stateful_widget(table, area, &mut self.table_state);
      // frame.render_stateful_widget(
      //    Scrollbar::default()
      //       .orientation(ScrollbarOrientation::VerticalRight)
      //       .begin_symbol(None)
      //       .end_symbol(None),
      //    area,
      //    &mut self.scroll_state,
      // );
      Ok(())
   }
}

impl Home {
   fn prev_row(&mut self) {
      let i = match self.table_state.selected() {
         Some(i) => {
            if i == 0 {
               self.tasks.len() - 1
            } else {
               i - 1
            }
         }
         None => 0,
      };
      self.table_state.select(Some(i));
      self.scroll_state = self.scroll_state.position(i * 1);
   }

   fn next_row(&mut self) {
      let i = match self.table_state.selected() {
         Some(i) => {
            if i >= self.tasks.len() - 1 {
               0
            } else {
               i + 1
            }
         }
         None => 0,
      };
      self.table_state.select(Some(i));
      self.scroll_state = self.scroll_state.position(i * 1);
   }
}
