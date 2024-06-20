use ratatui::prelude::{Frame, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::utils;

const HELP_TEXT: &str = r"This is a placeholder for the help text:
- Help point 1.
- Help point 2.
Etc, etc.";

pub struct HelpPopup {}

impl HelpPopup {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let popup_area = utils::create_centred_rect(area, 50, 50);
        let block = Block::new().borders(Borders::ALL);
        let widget = Paragraph::new(HELP_TEXT).block(block);
        f.render_widget(widget, popup_area);
    }
}
