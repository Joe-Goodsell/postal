use crate::tui::{
    popup::{HelpPopup, Popup},
    TUIComponents,
};

pub fn execute_display_help(tui: &mut TUIComponents) -> anyhow::Result<()> {
    tui.push_popup(Popup::HelpPopup(HelpPopup::new()));
    Ok(())
}

pub fn execute_quit(tui: &mut TUIComponents) -> anyhow::Result<()> {
    tui.should_quit = true;
    Ok(())
}
