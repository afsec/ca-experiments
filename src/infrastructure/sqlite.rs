use sqlx::SqlitePool;
use tracing::log::LevelFilter;

pub(crate) async fn create_sqlite_pool(filename_path: &'static str) -> anyhow::Result<SqlitePool> {
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
    use sqlx::ConnectOptions;
    use std::str::FromStr;
    use std::time::Duration;

    let busy_timeout = Duration::from_secs(2);

    let sqlite_path = if filename_path == ":memory:" {
        format!("sqlite:{}", filename_path)
    } else {
        format!("sqlite://{}", filename_path)
    };

    let mut connect_options = SqliteConnectOptions::from_str(sqlite_path.as_str())?
        .busy_timeout(busy_timeout)
        // Why we set to `Delete`: https://www.sqlite.org/pragma.html#pragma_journal_mode
        // > "The DELETE journaling mode is the normal behavior".
        .journal_mode(SqliteJournalMode::Delete)
        .create_if_missing(true);
    connect_options.log_statements(LevelFilter::Debug);
    let db_conn_pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
    {
        Ok(pool) => {
            tracing::debug!("Database connection Ok");
            pool
        }
        Err(error) => {
            tracing::error!("Database connection is not possible! Reason: {error:?}");
            panic!("Impossible state reached!")
        }
    };
    Ok(db_conn_pool)
}
