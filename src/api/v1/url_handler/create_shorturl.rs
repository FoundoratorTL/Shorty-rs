use axum::{extract::{State, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::query;
use crate::api::AppState;
use crate::errors::AppError;
use crate::utils::url_generator;

#[derive(Deserialize)]
pub struct CreateRequest {
    pub url: String,
    pub custom_alias: Option<String>,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub short_url: String
}

pub async fn handle(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRequest>,
) -> Result<Json<CreateResponse>, AppError> {
    let pool = &state.pool;

    // Basic URL validation
    if !payload.url.starts_with("http://") && !payload.url.starts_with("https://") {
        return Err(AppError::new(StatusCode::BAD_REQUEST, "Invalid URL"));
    }

    // Generate or use custom alias
    let short_code = payload.custom_alias.unwrap_or_else(url_generator::generate_short_code);

    // Insert into DB
    query!(
        "INSERT INTO url_mappings (short_code, original_url) VALUES ($1, $2)",
        short_code,
        payload.url,
    )
        .execute(pool)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Build the public short URL
    let base = std::env::var("BASE_URL").unwrap_or_default();
    let short_url = format!("{}/{}", base.trim_end_matches('/'), short_code);

    Ok(Json(CreateResponse {
        short_url,
    }))
}
