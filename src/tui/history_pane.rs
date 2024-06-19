use crate::App;

use ratatui::{
    prelude::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct HistoryPane {
    pub active: bool,
}

impl HistoryPane {
    pub fn new() -> Self {
        Self { active: false }
    }

    // *** renderers ***

    pub fn render(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let placeholder_text = "Not implemented";

        let block = Block::new().borders(Borders::ALL).title(" History ");

        let widget = Paragraph::new(placeholder_text).block(block);

        f.render_widget(widget, area);
    }

    // *** private render functions ***
    fn render_list(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        todo!()
    }
}
