
pub mod alias_redirection;

use axum::{Json, Router};
use std::sync::Arc;
use crate::api::AppState;


pub fn create_service_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/{short_code}", axum::routing::get(alias_redirection::handle))
        .route("/health", axum::routing::get(async || {Json("OK")}))
        .with_state(state) // share PgPool with handlers 
}