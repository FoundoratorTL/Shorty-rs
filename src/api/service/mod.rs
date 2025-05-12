pub(crate) mod healthcheck;
pub mod alias_redirection;

use axum::Router;
use std::sync::Arc;
use crate::api::AppState;

pub fn create_service_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/{short_code}", axum::routing::get(alias_redirection::handle))
        .with_state(state) // share PgPool with handlers 
}