use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Initialize a PgPool with up to 5 connections
pub async fn init_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)            // configure pool size 
        .connect(database_url)
        .await
}
