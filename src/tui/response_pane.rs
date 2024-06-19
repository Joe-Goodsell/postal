use crate::App;
use ratatui::{
    prelude::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct ResponsePane {}

impl ResponsePane {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let placeholder_text = "Not implemented";
        let block = Block::new().borders(Borders::ALL).title(" Response ");

        let widget = Paragraph::new(placeholder_text).block(block);

        f.render_widget(widget, area);
    }
}
