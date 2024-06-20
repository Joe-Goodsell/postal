use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;

pub const GLOBAL_ACTIVE_BORDER_COLOUR: Color = Color::LightGreen;
pub const GLOBAL_INACTIVE_BORDER_COLOUR: Color = Color::LightGreen;

pub const GLOBAL_ACTIVE_BG_COLOUR: Color = Color::DarkGray;
pub const GLOBAL_INACTIVE_BG_COLOUR: Color = Color::Black;

pub const GLOBAL_ACTIVE_FG_COLOUR: Color = Color::White;
pub const GLOBAL_INACTIVE_FG_COLOUR: Color = Color::Gray;

pub fn create_centred_rect(area: Rect, percent_vert: u16, percent_horiz: u16) -> Rect {
    let vert_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(100 - percent_vert / 2),
            Constraint::Min(percent_vert),
            Constraint::Max(100 - percent_vert / 2),
        ])
        .split(area)[1];
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Max(100 - percent_horiz / 2),
            Constraint::Min(percent_horiz),
            Constraint::Max(100 - percent_horiz / 2),
        ])
        .split(vert_split)[1]
}
