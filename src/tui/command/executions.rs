use crate::tui::{
    popup::{HelpPopup, Popup},
    TUIComponents,
};

pub fn execute_display_help(tui: &mut TUIComponents) -> anyhow::Result<()> {
    log::trace!("executing display help cmd");
    tui.push_popup(Popup::HelpPopup(HelpPopup::new()));
    Ok(())
}

pub fn execute_quit(tui: &mut TUIComponents) -> anyhow::Result<()> {
    log::trace!("executing quit cmd");
    tui.should_quit = true;
    Ok(())
}

pub fn execute_cycle_control_forward(tui: &mut TUIComponents) -> anyhow::Result<()> {
    log::trace!("executing cycle window forward");
    tui.next_control_mode();
    Ok(())
}

pub fn execute_cycle_control_backward(tui: &mut TUIComponents) -> anyhow::Result<()> {
    log::trace!("executing cycle window backward");
    tui.prev_control_mode();
    Ok(())
}

pub fn execute_dismiss_popup(tui: &mut TUIComponents) -> anyhow::Result<()> {
    log::trace!("executing dismiss popup");
    tui.popup_stack.pop().expect("expected popup to exist");
    Ok(())
}
