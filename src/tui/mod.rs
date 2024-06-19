use crossterm::event::Event;
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::tui::{
    footer::Footer, history_pane::HistoryPane, popup::*, query_pane::QueryPane,
    response_pane::ResponsePane,
};
use crate::App;

mod footer;
mod history_pane;
mod popup;
mod query_pane;
mod response_pane;
mod utils;

pub enum ControlMode {
    Query,
    Response,
    History,
    Popup,
}

pub struct TUIComponents {
    query_pane: QueryPane,
    response_pane: ResponsePane,
    history_pane: HistoryPane,
    footer: Footer,
    pub control: ControlMode,
    popup_stack: Vec<Popup>,
}

impl TUIComponents {
    pub fn new() -> TUIComponents {
        Self {
            query_pane: QueryPane::new(),
            response_pane: ResponsePane::new(),
            history_pane: HistoryPane::new(),
            footer: Footer::new(),
            control: ControlMode::Query,
            popup_stack: vec![],
        }
    }

    pub fn render(&mut self, f: &mut Frame<'_>, app: &mut App) {
        log::trace!("rendering TUIComponents");
        let area = f.size();

        // render components

        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(2)])
            .split(area);

        self.footer.render(f, app, split[1]);

        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(split[0]);
        let history_pane_area = split[0];

        self.history_pane.render(f, app, history_pane_area);

        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(split[1]);

        let query_pane_area = split[0];
        let response_pane_area = split[1];

        self.response_pane.render(f, app, response_pane_area);
        self.query_pane.render(f, app, query_pane_area);
    }

    pub async fn handle_input(&mut self, input: &Event, app: &mut App) -> anyhow::Result<()> {
        Ok(())
    }
}
