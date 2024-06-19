use crate::http;
use sqlx::{
    sqlite::SqlitePool,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

pub struct SqliteAdapter {
    // TODO: should not be Option
    pool: Option<SqlitePool>,
}

impl SqliteAdapter {
    pub fn new() -> anyhow::Result<Self> {
        let pool = None;
        Ok(Self { pool })
    }

    pub fn load_all() -> anyhow::Result<Vec<HistoryEntry>> {
        todo!()
    }

    pub fn save_entry() -> anyhow::Result<()> {
        todo!()
    }
}

pub struct HistoryEntry {
    id: Uuid,
    verb: http::HttpVerb,
    query: String,
    date_time: DateTime<Utc>,
}
