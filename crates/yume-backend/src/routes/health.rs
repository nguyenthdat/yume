//! Health-check endpoint.
//!
//! `GET /health` — liveness probe, returns JSON status.

use axum::{http::StatusCode, response::Json};
use serde::Serialize;

/// Health-check response body.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub environment: String,
}

/// `GET /health`
///
/// Returns HTTP 200 with a JSON status payload indicating the service is alive.
pub async fn health() -> (StatusCode, Json<HealthResponse>) {
    let env = std::env::var("YUME_ENV").unwrap_or_else(|_| "development".to_string());

    let body = HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        environment: env,
    };

    (StatusCode::OK, Json(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use axum_test::TestServer;
    use serde_json::Value;

    fn app() -> Router {
        Router::new().route("/health", get(health))
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let server = TestServer::new(app());

        let response = server.get("/health").await;

        response.assert_status_ok();

        let json: Value = response.json();
        assert_eq!(json["status"], "ok");
        assert!(json["version"].is_string());
        assert!(json["environment"].is_string());
    }
}
