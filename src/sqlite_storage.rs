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

async fn create_table(conn : &mut SqliteConnection, table_name : &str, headers: &Vec<&str>) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let formatted_columns = 
        headers.iter().enumerate().fold(String::from(""),|accum, (count, header_name)| {
            let datatype = match count {
                0 => "INTEGER",
                1 => "TEXT",
                _ => "REAL",
            };
            let column_string = format!("{} {}, ", header_name, datatype);
            format!("{} {}", accum, column_string.as_str())
        });
    let formatted_query = format!("CREATE TABLE IF NOT EXISTS {} ({});", table_name, formatted_columns);
    Ok(sqlx::query(&formatted_query).execute(conn).await?)
}