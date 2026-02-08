mod widgets;

use std::io::Read;

use ratatui::{
   Frame,
   layout::{Constraint, Flex, Layout, Rect},
   style::Style,
   text,
   widgets::{Block, Clear, Paragraph},
};
use snap::read::FrameDecoder;

use crate::core::{Core, Layer};

pub fn render(frame: &mut Frame, core: &mut Core) -> color_eyre::Result<()> {
   let area = frame.area();

   for layer in core.layers().to_vec() {
      match layer {
         Layer::List => render_list_layer(frame, area, core)?,
         Layer::Help => render_help_layer(frame, area)?,
      }
   }

   Ok(())
}

fn render_list_layer(
   frame: &mut Frame,
   area: ratatui::layout::Rect,
   core: &mut Core,
) -> color_eyre::Result<()> {
   let area = {
      if core.log_preview {
         let [log_area, area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);
         if core.pueue_log.is_none() {
            frame.render_widget(Clear, area);
         } else {
            let task_id = core.pueue_log.as_ref().unwrap().0;
            let selected_id = core.selected_task_id();

            if Some(task_id) == selected_id {
               let mut decompressor =
                  FrameDecoder::new(core.pueue_log.as_ref().unwrap().1.as_ref()); // TODO: Use if let
               let mut s = String::new();
               decompressor.read_to_string(&mut s)?;

               if s.is_empty() {
                  let text = text::Text::from("No log").style(Style::new().bold());
                  let paragraph = Paragraph::new(text.clone());
                  let area = log_area.centered(
                     Constraint::Length(text.width() as u16),
                     Constraint::Length(1),
                  );
                  frame.render_widget(paragraph, area);
               } else {
                  let log_text = Paragraph::new(s);
                  frame.render_widget(log_text, log_area);
               }
            } else {
               frame.render_widget(Clear, area);
            }
         }
         area
      } else {
         area
      }
   };

   let [table_area, status_bar_area] =
      Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

   let table = widgets::task_table::TaskTable::new(&core.pueue_tasks);
   let status_bar = widgets::status_bar::StatusBar::new("Quit: q | Help: h | Log preview: l");

   frame.render_stateful_widget(table, table_area, &mut core.list_state);
   frame.render_widget(status_bar, status_bar_area);

   Ok(())
}

fn render_help_layer(frame: &mut Frame, area: ratatui::layout::Rect) -> color_eyre::Result<()> {
   let area = popup_area(area, 60, 60);
   let text = Paragraph::new(text::Text::from(
      "Help\n\nq: Quit\nh: Help\nEsc: Close Help\nl: Toggle Log Preview",
   ))
   .block(Block::bordered());
   frame.render_widget(Clear, area);
   frame.render_widget(text, area);
   Ok(())
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
   let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
   let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
   let [area] = vertical.areas(area);
   let [area] = horizontal.areas(area);
   area
}
