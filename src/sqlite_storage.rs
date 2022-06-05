use sqlx::{ConnectOptions, SqliteConnection};
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;

pub async fn connect_and_create_database_if_not_exists() -> Result<SqliteConnection, sqlx::Error> {
    Ok(SqliteConnectOptions::from_str("sqlite://nbastats.db")?
        .create_if_missing(true)
        .connect().await?)
}

#[test]
fn test_database_connection() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            connect_and_create_database_if_not_exists().await;
        })
}

