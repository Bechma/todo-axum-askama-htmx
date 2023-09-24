pub type DBPool = sqlx::SqlitePool;

pub async fn connect_to_database() -> DBPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_url.parse().unwrap())
        .await
        .unwrap()
}

pub fn map_db_err(err: sqlx::Error) -> (axum::http::StatusCode, String) {
    tracing::error!("{}", err);
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string(),
    )
}
