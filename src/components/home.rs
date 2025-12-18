use ratatui::{layout::Rows, prelude::*, widgets::*};
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
      let rows = [
         Row::new(vec!["Cell1", "Cell2", "Cell3"]),
         Row::new(vec!["Data1", "Data2", "Data3"]),
      ];
      let widths = vec![
         Constraint::Percentage(33),
         Constraint::Percentage(33),
         Constraint::Percentage(34),
      ];
      let table = Table::new(rows, widths)
         .style(Style::new().blue())
         .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
               .style(Style::new().bold())
               // To add space between the header and the rest of the rows, specify the margin
               .bottom_margin(1),
         )
         .footer(Row::new(vec!["Updated on Dec 28"]))
         .block(Block::new().title(" Table ").borders(Borders::ALL))
         .row_highlight_style(Style::new().reversed())
         .highlight_symbol(">>");
      frame.render_widget(table, area);
      Ok(())
   }
}
