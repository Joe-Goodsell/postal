use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, BorderType, Borders, Padding};

pub const GLOBAL_ACTIVE_BORDER_COLOUR: Color = Color::Yellow;
pub const GLOBAL_INACTIVE_BORDER_COLOUR: Color = Color::Gray;

pub const GLOBAL_ACTIVE_BG_COLOUR: Color = Color::DarkGray;
pub const GLOBAL_INACTIVE_BG_COLOUR: Color = Color::Black;

pub const GLOBAL_ACTIVE_FG_COLOUR: Color = Color::White;
pub const GLOBAL_INACTIVE_FG_COLOUR: Color = Color::Gray;

pub fn styled_block() -> Block<'static> {
    Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::proportional(2))
}

pub fn create_centred_rect_abs(area: Rect, width: u16, height: u16) -> Rect {
    // let vert_size = ((1.0 / percent_vert as f32) * area.width as f32) as u16;
    // let horiz_size = ((1.0 / percent_horiz as f32) * area.height as f32) as u16;
    let vert_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min((100 - height) / 2),
            Constraint::Min(height),
            Constraint::Min((100 - height) / 2),
        ])
        .split(area)[1];
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min((100 - width) / 2),
            Constraint::Min(width),
            Constraint::Min((100 - width) / 2),
        ])
        .split(vert_split)[1]
}
