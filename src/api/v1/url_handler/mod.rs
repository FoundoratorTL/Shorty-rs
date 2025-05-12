pub mod create_shorturl;

use axum::Router;
use std::sync::Arc;
use crate::api::AppState;
use crate::api::service::alias_redirection;

/// Nest URL routes: POST `/` to shorten, GET `/:short_code` to redirect
pub fn create_url_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/shorten", axum::routing::post(create_shorturl::handle))
        .with_state(state) // share PgPool with handlers 
}
