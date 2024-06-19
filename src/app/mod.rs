use backend::sqlite::{HistoryEntry, SqliteAdapter};

use crate::Config;

pub struct App {
    sqlite_adapter: SqliteAdapter,
    history: Vec<HistoryEntry>,
    selected_history_entry: Option<u32>,
    config: Config,
}

impl App {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let sqlite_adapter = SqliteAdapter::new()?;
        let history = vec![];
        let selected_history_entry = None;
        Ok(App {
            sqlite_adapter,
            history,
            selected_history_entry,
            config,
        })
    }
}

pub enum AppMode {
    Normal,
    Insert,
    Visual,
}
