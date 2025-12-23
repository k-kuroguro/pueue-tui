use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::action::Action;

#[derive(Default)]
pub struct Home {
   command_tx: Option<UnboundedSender<Action>>,
   table_state: TableState,
   scroll_state: ScrollbarState,
   data: Vec<Vec<String>>,
}

impl Home {
   pub fn new() -> Self {
      Self {
         command_tx: None,
         table_state: TableState::default().with_selected(0),
         scroll_state: ScrollbarState::new(0),
         data: Vec::new(),
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
            let mut data = Vec::new();
            for (_, task) in state.tasks.iter() {
               data.push(vec![
                  task.id.to_string(),
                  task.status.to_string(),
                  task.command.clone(),
                  task.path.to_string_lossy().to_string(),
               ]);
            }
            self.data = data;
            self.scroll_state = self.scroll_state.content_length(self.data.len());
         }
         _ => {}
      }
      Ok(None)
   }

   fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
      let max_id = self
         .data
         .iter()
         .map(|item| item[0].len())
         .max()
         .unwrap_or(2)
         .max(2);
      let widths = vec![
         Constraint::Length(max_id as u16 + 2),
         Constraint::Percentage(25),
         Constraint::Percentage(25),
         Constraint::Percentage(25),
      ];
      let rows: Vec<Row> = self
         .data
         .iter()
         .map(|item| Row::new(item.iter().cloned()))
         .collect();
      let table = Table::new(rows, widths)
         .header(Row::new(vec!["Id", "Status", "Command", "Path"]).style(Style::new().bold()))
         .row_highlight_style(Style::new().blue().on_black());

      frame.render_stateful_widget(table, area, &mut self.table_state);
      frame.render_stateful_widget(
         Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
         area,
         &mut self.scroll_state,
      );
      Ok(())
   }
}

impl Home {
   fn prev_row(&mut self) {
      let i = match self.table_state.selected() {
         Some(i) => {
            if i == 0 {
               self.data.len() - 1
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
            if i >= self.data.len() - 1 {
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
