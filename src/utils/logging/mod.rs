use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, fmt};

/// Initialize `tracing` subscriber with environment filter and fmt layer
pub fn init_logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap())) // RUST_LOG support 
        .with(fmt::layer())
        .init(); // shorthand init 
}
pub async fn log_request(req: Request<Body>, next: Next) -> Response
{
    // Log the request details
    info!("Incoming request: {} {}", req.method(), req.uri());
    // Continue to the next middleware or url_handler
    let response: Response = next.run(req).await;
    // Log the response status
    info!("Incoming request: {}", response.status());
    response
}
