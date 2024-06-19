use crate::App;
use backend::http::HttpVerb;
use ratatui::{
    prelude::{Alignment, Color, Constraint, Direction, Layout, Rect, Text},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

pub struct QueryPane {}

impl QueryPane {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        log::trace!("rendering querypane");
        let placeholder_text = "Not implemented";
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(2))
            .title(" Query ");

        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(6), Constraint::Percentage(100)])
            .margin(1)
            .split(area);
        let response_area = split[1];

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
            .border_type(BorderType::Rounded);

        let verb = HttpVerb::POST;

        let fg = match verb {
            HttpVerb::GET => Color::Red,
            HttpVerb::POST => Color::Yellow,
        };

        let widget = Paragraph::new(Text::styled("POST", Style::default().fg(fg)))
            .alignment(Alignment::Center)
            .block(block);

        f.render_widget(widget, area);
    }

    fn render_url(&mut self, f: &mut Frame, app: &mut App, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let widget = Paragraph::new("https://.../").block(block);

        f.render_widget(widget, area);
    }
}
