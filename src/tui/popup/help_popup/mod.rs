use ratatui::prelude::{Frame, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::tui::utils;

use super::PopupWidget;

const HELP_TEXT: &str = r"This is a placeholder for the help text:
- Help point 1.
- Help point 2.
Press 'c' to close";

pub struct HelpPopup {
    is_handled: bool,
}

impl HelpPopup {
    pub fn new() -> Self {
        Self { is_handled: false }
    }
}

impl PopupWidget for HelpPopup {
    fn is_handled(&self) -> bool {
        self.is_handled
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let popup_area = utils::create_centred_rect_abs(area, 50, 45);
        let block = utils::styled_block()
            .title(" Help ")
            .bg(utils::GLOBAL_INACTIVE_BG_COLOUR);
        let widget = Paragraph::new(HELP_TEXT).block(block);
        f.render_widget(Clear, popup_area);
        f.render_widget(widget, popup_area);
    }
}
