use crate::App;
use ratatui::{
    prelude::Rect,
    style::Color,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::utils;

pub struct ResponsePane {
    pub is_active: bool,
}

impl ResponsePane {
    pub fn new() -> Self {
        Self { is_active: false }
    }
    pub fn render(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let placeholder_text = "Not implemented";

        let border_color: Color = if self.is_active {
            utils::GLOBAL_ACTIVE_BORDER_COLOUR
        } else {
            utils::GLOBAL_INACTIVE_BORDER_COLOUR
        };

        let block = utils::styled_block()
            .title(" Response ")
            .border_style(border_color);

        let widget = Paragraph::new(placeholder_text).block(block);

        f.render_widget(widget, area);
    }
}
