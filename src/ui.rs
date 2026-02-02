mod status;
mod widgets;

use ratatui::{
   Frame,
   layout::{Constraint, Direction, Layout},
   style::{Color, Style},
   widgets::{Block, Borders, Paragraph},
};

use self::status::Status;
use crate::core::Core;

pub use status::Status as UiStatus;

pub fn render(frame: &mut Frame, core: &Core) {
   let chunks = Layout::default()
      .direction(Direction::Vertical)
      .margin(1)
      .constraints([Constraint::Min(3), Constraint::Length(3)])
      .split(frame.area());

   let main_block = Paragraph::new("main_content")
      .block(Block::default().title("Main").borders(Borders::ALL))
      .style(Style::default().fg(Color::White));

   frame.render_widget(main_block, chunks[0]);

   // Use the UI adapter which converts Core state to widget state
   let status = Status::new(core);
   frame.render_widget(status, chunks[1]);
}
