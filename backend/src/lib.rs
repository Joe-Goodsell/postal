use sqlx::sqlite::SqlitePool;

pub struct SqliteAdapter {
    pool: SqlitePool,
}
