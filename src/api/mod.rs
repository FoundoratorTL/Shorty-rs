pub mod route;
mod service;
mod v1;

use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


