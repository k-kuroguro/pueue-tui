use std::vec;

use crossterm::event::{KeyEvent, KeyModifiers};
use pueue_lib::Task;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{
   action::Action,
   widgets::{
      status_bar::StatusBar,
      task_table::{TaskTable, TaskTableState},
   },
};

#[derive(Default)]
pub struct Home {
   command_tx: Option<UnboundedSender<Action>>,
   table_state: TaskTableState,
   tasks: Vec<Task>,
}

impl Home {
   pub fn new() -> Self {
      Self {
         command_tx: None,
         table_state: (TableState::new().with_selected(0), ScrollbarState::new(0)),
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
         }
         _ => {}
      }
      Ok(None)
   }

   fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
      let [table_area, status_bar_area] =
         Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

      let table = TaskTable::new(&self.tasks);
      let status_bar = StatusBar::new("Quit : q");

      frame.render_stateful_widget(table, table_area, &mut self.table_state);
      frame.render_widget(status_bar, status_bar_area);
      Ok(())
   }
}

impl Home {
   fn prev_row(&mut self) {
      let i = match self.table_state.0.selected() {
         Some(i) => {
            if i == 0 {
               self.tasks.len() - 1
            } else {
               i - 1
            }
         }
         None => 0,
      };
      self.table_state.0.select(Some(i));
      self.table_state.1 = self.table_state.1.position(i * 1);
   }

   fn next_row(&mut self) {
      let i = match self.table_state.0.selected() {
         Some(i) => {
            if i >= self.tasks.len() - 1 {
               0
            } else {
               i + 1
            }
         }
         None => 0,
      };
      self.table_state.0.select(Some(i));
      self.table_state.1 = self.table_state.1.position(i * 1);
   }
}
