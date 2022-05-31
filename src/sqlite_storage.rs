use sqlx::sqlite::SqlitePoolOptions;

pub async fn connectToDatabase() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect("");
    unimplemented!();
}