//! Yume backend entry point.
//!
//! Boots an Axum HTTP server with health and chat streaming endpoints.
//! CORS is enabled for development — Android and web clients can connect.

mod config;
mod routes;

use std::sync::Arc;

use axum::{Router, routing::get, routing::post};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load .env if present, ignore errors
    dotenvy::dotenv().ok();

    // Initialise structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cfg = Arc::new(config::Config::from_env());
    tracing::info!(
        env = %cfg.env,
        listen_addr = %cfg.listen_addr,
        "Starting Yume backend",
    );

    // CORS — permissive for development; tighten for staging/prod
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let chat_state = routes::chat::ChatState {
        config: cfg.clone(),
    };

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/v1/chat/stream", post(routes::chat::chat_stream))
        .with_state(chat_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&cfg.listen_addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Listening on {}", cfg.listen_addr);

    axum::serve(listener, app).await.expect("Server error");
}
