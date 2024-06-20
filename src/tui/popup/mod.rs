pub use self::help_popup::HelpPopup;

pub mod help_popup;

pub mod delete_history_entry_popup;

pub enum Popup {
    HelpPopup(HelpPopup),
    DeleteHistoryEntryPopup(),
}
