use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Stateful StatusBar widget - independent from Core
/// 
/// This widget manages its own state and can be reused in any context.
/// It doesn't know about Core, making it highly reusable and testable.
#[derive(Debug, Clone)]
pub struct StatusBar {
    /// The message to display
    pub message: String,
    /// The style for the message
    pub style: Style,
    /// Whether to show a border
    pub bordered: bool,
}

impl StatusBar {
    /// Create a new StatusBar with the given message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            style: Style::default().fg(Color::Gray),
            bordered: true,
        }
    }

    /// Set the message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Set the style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set whether to show borders
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = if self.bordered {
            Paragraph::new(self.message)
                .block(Block::default().title("Status").borders(Borders::ALL))
                .style(self.style)
        } else {
            Paragraph::new(self.message).style(self.style)
        };

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_creation() {
        let status = StatusBar::new("Test message");
        assert_eq!(status.message, "Test message");
        assert!(status.bordered);
    }

    #[test]
    fn test_status_bar_builder() {
        let status = StatusBar::new("Test")
            .message("Updated")
            .bordered(false)
            .style(Style::default().fg(Color::Red));
        
        assert_eq!(status.message, "Updated");
        assert!(!status.bordered);
    }
}
