//! Yume backend entry point.

mod config;
mod routes;

use std::sync::Arc;

use axum::{Router, routing::get, routing::post};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<config::Config>,
    pub chat: routes::chat::ChatState,
    pub oauth: routes::oauth::OAuthState,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cfg = Arc::new(config::Config::from_env());
    let listen_addr = cfg.listen_addr.clone();
    tracing::info!(env = %cfg.env, listen_addr = %listen_addr, "Starting Yume backend");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let state = AppState {
        chat: routes::chat::ChatState { config: cfg.clone() },
        oauth: routes::oauth::OAuthState {
            config: cfg.clone(),
            tokens: routes::oauth::TokenStore::new(),
        },
        config: cfg,
    };

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/v1/chat/stream", post(routes::chat::chat_stream))
        .route("/v1/auth/openai/authorize", post(routes::oauth::authorize))
        .route("/v1/auth/openai/callback", get(routes::oauth::callback))
        .route("/v1/auth/openai/refresh", post(routes::oauth::refresh))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&listen_addr).await
        .expect("Failed to bind");
    tracing::info!("Listening on {listen_addr}");
    axum::serve(listener, app).await.expect("Server error");
}
