use axum::{Json, Router, routing::get};

/// A simple health-check endpoint
pub fn healthcheck_router() -> Router {
    Router::new().route("/", get(|| async { Json("OK") }))
}


