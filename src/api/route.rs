use axum::Router;
use std::sync::Arc;

use crate::api::{AppState};
use crate::api::service::create_service_router;
use crate::api::v1::router as v1_router;

/// Compose top-level routes
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(create_service_router(state.clone()))
        .merge(v1_router(state))
}
