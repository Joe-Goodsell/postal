use ratatui::{layout::Rect, Frame};

pub use self::help_popup::HelpPopup;

pub mod help_popup;

pub mod delete_history_entry_popup;

pub enum Popup {
    HelpPopup(HelpPopup),
}

pub trait PopupWidget {
    fn is_handled(&self) -> bool;
    fn render(&self, f: &mut Frame, area: Rect);
}
