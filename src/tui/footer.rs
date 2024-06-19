use crate::App;
use ratatui::{
    prelude::{Alignment, Frame, Rect, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct Footer {}

impl Footer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let paragraph = Paragraph::new("placeholder text")
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default()),
            );

        f.render_widget(paragraph, area);
    }
}
