use std::sync::Arc;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;
use sqlx::query;
use crate::api;
use crate::config::Config;
use crate::database::postgres::init_db_pool;
use crate::errors::AppError;

pub async fn handle(
    Path(short_code): Path<String>,
) -> Result<Redirect, AppError>
{
    let config = Config::from_env();
    let pool = init_db_pool(&config.unwrap().database_url).await?;

    // Pass the original pool
    let state = Arc::new(api::AppState::new(pool.clone()));

    let record = query!(
        "SELECT original_url FROM url_mappings WHERE short_code = $1",
        short_code
    )
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(rec) = record {
        Ok(Redirect::temporary(&rec.original_url))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Short URL not found"))
    }
}