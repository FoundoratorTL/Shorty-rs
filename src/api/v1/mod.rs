pub(crate) mod url_handler;

use axum::Router;
use std::sync::Arc;

use crate::api::AppState;
use crate::api::v1::url_handler::create_url_router;

/// API v1 routes (under `/api/v1`)
pub fn router(state: Arc<AppState>) -> Router {
    create_url_router(state)
}
