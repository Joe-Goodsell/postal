use backend::SqliteAdapter;
use crossterm::event::KeyEvent;

use crate::tui::{history_pane::HistoryPane, query_pane::QueryPane, response_pane::ResponsePane};
use crate::App;

mod history_pane;
mod query_pane;
mod response_pane;
mod runner;

pub enum ControlMode {
    Query,
    Response,
    History,
}

pub struct TUIComponents {
    query_pane: QueryPane,
    response_pane: ResponsePane,
    history_pane: HistoryPane,
    pub control: ControlMode,
    sqlite_adapter: SqliteAdapter,
}

impl TUIComponents {
    pub fn new() -> TUIComponents {
        todo!()
    }
    pub async fn handle_input(&mut self, input: &KeyEvent, app: &mut App) -> anyhow::Result<()> {
        Ok(())
    }
}
