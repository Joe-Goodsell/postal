use crate::{
    tui::{command::KeyMap, utils},
    App,
};
use backend::http::HttpVerb;
use ratatui::{
    prelude::{Alignment, Color, Constraint, Direction, Layout, Rect, Text},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use super::command::{keymaps::get_global_keymap, Input};

#[derive(PartialEq, Eq)]
pub enum QueryPaneControlMode {
    None,
    Verb,
    URL,
    KeyValue,
}

pub struct QueryPane {
    pub is_active: bool,
    pub control: QueryPaneControlMode,
    none_keymap: KeyMap,
    url_keymap: KeyMap,
    verb_keymap: KeyMap,
    keyval_keymap: KeyMap,
}

impl QueryPane {
    pub fn new() -> Self {
        Self {
            is_active: false,
            control: QueryPaneControlMode::None,
            none_keymap: get_global_keymap(),
            url_keymap: get_global_keymap(),
            verb_keymap: get_global_keymap(),
            keyval_keymap: get_global_keymap(),
        }
    }

    pub async fn handle_input(&mut self, input: &Input, app: &mut App) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        log::trace!("rendering querypane");

        let border_color: Color = if self.is_active {
            utils::GLOBAL_ACTIVE_BORDER_COLOUR
        } else {
            utils::GLOBAL_INACTIVE_BORDER_COLOUR
        };

        let block = utils::styled_block()
            .title(" Query ")
            .border_style(border_color);

        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(6), Constraint::Percentage(100)])
            .margin(1)
            .split(area);
        let key_val_area = split[1];
        self.render_key_val(f, app, key_val_area);

        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(8), Constraint::Percentage(90)])
            .margin(1)
            .split(split[0]);
        let (verb_area, url_area) = (split[0], split[1]);

        f.render_widget(block, area);
        self.render_verb(f, app, verb_area);
        self.render_url(f, app, url_area);
    }

    // *** private renderers ***
    fn render_verb(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title(" Verb ")
            .border_type(BorderType::Rounded);

        let verb = app.current_query.verb.clone();

        let fg = match verb {
            HttpVerb::GET => Color::Red,
            HttpVerb::POST => Color::Yellow,
        };

        let widget = Paragraph::new(Text::styled(verb.to_string(), Style::default().fg(fg)))
            .alignment(Alignment::Center)
            .block(block);

        f.render_widget(widget, area);
    }

    fn render_url(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title(" URL ")
            .border_type(BorderType::Rounded);

        let url_text = app.current_query.url.clone();

        let widget = Paragraph::new(url_text).block(block);

        f.render_widget(widget, area);
    }

    fn render_key_val(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        f.render_widget(block.clone(), area);

        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        f.render_widget(block.clone().title(" Key "), split[0]);
        f.render_widget(block.title(" Value "), split[1]);
    }
}
