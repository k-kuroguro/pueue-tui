use std::vec;

use ratatui::{
   buffer::Buffer,
   layout::Rect,
   style::Stylize,
   text::{Line, Span},
   widgets::Widget,
};

pub struct StatusBar {
   left: String,
}

impl StatusBar {
   pub fn new(left: &str) -> Self {
      Self {
         left: left.to_string(),
      }
   }
}

impl Widget for StatusBar {
   fn render(self, area: Rect, buf: &mut Buffer) {
      const MIN_SPACE: u16 = 2;
      const ELLIPSIS: &str = "...";
      const ELLIPSIS_LEN: u16 = 3;

      let width = area.width;

      let left_len = self.left.chars().count() as u16;

      let pkg_name = env!("CARGO_PKG_NAME");
      let pkg_ver = env!("CARGO_PKG_VERSION");

      let right_len = pkg_name.chars().count() as u16 + 2 + pkg_ver.chars().count() as u16;

      let left_text = {
         let needed = left_len + right_len + MIN_SPACE;

         if needed < width {
            self.left
         } else {
            let available = width.saturating_sub(right_len + MIN_SPACE);

            if available <= ELLIPSIS_LEN {
               ELLIPSIS.to_string()
            } else {
               let keep = (available - ELLIPSIS_LEN) as usize;
               let s: String = self.left.chars().take(keep).collect();
               format!("{s}{ELLIPSIS}")
            }
         }
      };

      Line::from(left_text).left_aligned().render(area, buf);
      Line::from(vec![
         Span::from(pkg_name).bold(),
         Span::from(format!(" v{pkg_ver}")),
      ])
      .right_aligned()
      .render(area, buf);
   }
}
