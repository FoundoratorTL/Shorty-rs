mod api;
mod config;
mod database;
mod errors;
mod utils;

use api::route::create_router;
use config::Config;
use database::postgres::init_db_pool;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utils::logging::{init_logging, log_request};
use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method},
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging
    init_logging(); // tracing-subscriber init

    // Load configuration
    let config = Config::from_env()?;

    // Initialize PostgresSQL connection pool
    let pool = init_db_pool(&config.database_url).await?; // PgPoolOptions usage

    sqlx::migrate!().run(&pool).await?;

    // Shared application state
    let state = Arc::new(api::AppState::new(pool));

    // CORS and tracing middleware
    let cors = CorsLayer::new()
        // Allow requests from your frontend origin
        .allow_origin("http://localhost:4321".parse::<HeaderValue>().unwrap())
        // Allow GET, POST, PUT, DELETE methods
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        // Allow content-type header
        .allow_headers([HeaderName::from_static("content-type")])
        // Allow credentials (cookies, authorization headers, etc.)
        .allow_credentials(true);
    let trace = TraceLayer::new_for_http(); // HTTP tracing

    // Build and run the Axum app
    let app = create_router(state)
        .layer(cors)
        .layer(trace)
        .layer(axum::middleware::from_fn(log_request));

    tracing::info!("Listening on {}", config.server_addr);

    let listener = TcpListener::bind(&config.server_addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
