use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::Error as SqlxError;
use sqlx::migrate::MigrateError;

/// A simple error type that turns into an HTTP JSON response.
pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self { status, message: message.into() }
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        )
    }
}

impl From<MigrateError> for AppError {
    fn from(err: MigrateError) -> Self {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": self.message }));
        (self.status, body).into_response()
    }
}
