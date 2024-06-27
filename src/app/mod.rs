use backend::sqlite::{HistoryEntry, SqliteAdapter};

use crate::Config;

use self::query::Query;

pub mod query;

pub struct App {
    sqlite_adapter: SqliteAdapter,
    history: Vec<HistoryEntry>,
    selected_history_entry: Option<u32>,
    config: Config,
    pub current_query: Query,
    is_saved: bool,
}

impl App {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let sqlite_adapter = SqliteAdapter::new()?;
        let history = vec![];
        let selected_history_entry = None;
        let current_query = Query::new();
        Ok(App {
            sqlite_adapter,
            history,
            selected_history_entry,
            config,
            current_query,
            is_saved: true,
        })
    }
}

pub enum AppMode {
    Normal,
    Insert,
    Visual,
}
