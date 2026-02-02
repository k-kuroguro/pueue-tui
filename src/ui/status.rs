use ratatui::{
   buffer::Buffer,
   layout::Rect,
   style::{Color, Style},
   widgets::Widget,
};

use super::widgets;
use crate::core::Core;

/// UI Adapter for StatusBar
///
/// This struct acts as an adapter between Core and the stateful StatusBar widget.
/// It converts Core's state into the widget's state, keeping the widget independent.
pub struct Status<'a> {
   core: &'a Core,
}

impl<'a> Status<'a> {
   pub fn new(core: &'a Core) -> Self {
      Self { core }
   }

   /// Convert Core state to status message
   fn build_message(&self) -> String {
      // Get current time (synchronous read with parking_lot)
      let current_time = self.core.current_time.read();
      let time_display = if current_time.is_empty() {
         "Loading...".to_string()
      } else {
         current_time.clone()
      };

      let layer = self.core.layer();
      let base_msg = match layer {
         crate::core::Layer::List => "Press 'q' to quit, 's' to search, 'h' for help",
         crate::core::Layer::Search => "Search | Press ESC to cancel",
         crate::core::Layer::Help => "Help | Press ESC to close",
      };

      format!("{} | Time: {}", base_msg, time_display)
   }

   /// Determine style based on Core state
   fn build_style(&self) -> Style {
      match self.core.layer() {
         crate::core::Layer::List => Style::default().fg(Color::Gray),
         crate::core::Layer::Search => Style::default().fg(Color::Yellow),
         crate::core::Layer::Help => Style::default().fg(Color::Cyan),
      }
   }
}

impl Widget for Status<'_> {
   fn render(self, area: Rect, buf: &mut Buffer) {
      // Convert Core state to widget state (隠蔽)
      let widget = widgets::StatusBar::new(self.build_message())
         .style(self.build_style())
         .bordered(true);

      // Render the actual widget
      widget.render(area, buf);
   }
}
