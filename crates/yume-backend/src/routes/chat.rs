//! Chat streaming endpoint.
//!
//! `POST /v1/chat/stream` — accepts a [`ChatRequest`], streams SSE [`ChatEvent`]s
//! back to the client via `text/event-stream`.

use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    Json,
};
use futures::stream::Stream;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;
use yume_contracts::{
    chat::ChatRequest,
    event::{ChatEvent, TextDelta},
    schema::CURRENT_SCHEMA_VERSION,
};

use crate::config::Config;
use yume_opencode_client::OpenCodeStream;

/// Shared application state for the chat route.
#[derive(Clone)]
pub struct ChatState {
    pub config: Arc<Config>,
}

/// `POST /v1/chat/stream`
///
/// Validates the request, connects to OpenCode (or falls back to mock mode),
/// and streams SSE events back to the client.
pub async fn chat_stream(
    State(state): State<ChatState>,
    Json(request): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    // Validate schema version
    if request.schema_version != CURRENT_SCHEMA_VERSION {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Unsupported schema version: {}. Expected: {}",
                request.schema_version, CURRENT_SCHEMA_VERSION
            ),
        ));
    }

    let conversation_id = uuid::Uuid::new_v4().to_string();
    let message_id = uuid::Uuid::new_v4().to_string();

    // Extract config values for the spawned task
    let opencode_base_url = state.config.opencode_base_url.clone();
    let opencode_username = state.config.opencode_username.clone();
    let opencode_password = state.config.opencode_password.clone();
    let message_content = request.message.content.clone();

    // Connect to OpenCode (or mock)
    let opencode_stream = OpenCodeStream::connect(
        opencode_base_url,
        opencode_username,
        opencode_password,
        message_content,
    )
    .await;

    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(32);

    // Spawn a task to drive the stream and send SSE events
    tokio::spawn(async move {
        // Send chat.started
        let started = ChatEvent::ChatStarted {
            conversation_id: conversation_id.clone(),
            message_id: message_id.clone(),
        };
        let _ = tx
            .send(Ok(Event::default()
                .event("chat.started")
                .data(serde_json::to_string(&started).unwrap())))
            .await;

        // Stream message deltas
        let mut seq: u64 = 0;
        let mut total_output: u64 = 0;
        let mut opencode_stream = Box::pin(opencode_stream);

        while let Some(chunk_result) = opencode_stream.next().await {
            match chunk_result {
                Ok(text) => {
                    if text.is_empty() {
                        continue;
                    }
                    total_output += text.len() as u64;
                    seq += 1;

                    // For mock mode, add realistic delays between words
                    if seq > 1 {
                        tokio::time::sleep(Duration::from_millis(30)).await;
                    }

                    let delta = ChatEvent::MessageDelta {
                        seq,
                        delta: TextDelta { text },
                    };
                    let _ = tx
                        .send(Ok(Event::default()
                            .event("message.delta")
                            .data(serde_json::to_string(&delta).unwrap())))
                        .await;
                }
                Err(e) => {
                    let error = ChatEvent::Error {
                        code: "PROVIDER_ERROR".to_string(),
                        message: e.to_string(),
                        retry_after_ms: Some(5000),
                        recoverable: true,
                    };
                    let _ = tx
                        .send(Ok(Event::default()
                            .event("error")
                            .data(serde_json::to_string(&error).unwrap())))
                        .await;
                    break;
                }
            }
        }

        // Send usage
        let usage = ChatEvent::Usage {
            input_tokens: estimate_tokens(&request.message.content),
            output_tokens: total_output / 4, // rough estimate
        };
        let _ = tx
            .send(Ok(Event::default()
                .event("usage")
                .data(serde_json::to_string(&usage).unwrap())))
            .await;

        // Send done
        let done = ChatEvent::Done {
            finish_reason: "stop".to_string(),
        };
        let _ = tx
            .send(Ok(Event::default()
                .event("done")
                .data(serde_json::to_string(&done).unwrap())))
            .await;
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

/// Rough token count estimate (4 chars ≈ 1 token for English/Vietnamese).
fn estimate_tokens(text: &str) -> u64 {
    (text.chars().count() as u64 / 4).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, routing::post};
    use axum_test::TestServer;

    fn test_app() -> Router {
        let config = Arc::new(Config {
            listen_addr: "127.0.0.1:0".into(),
            env: "test".into(),
            retention_days: 30,
            log_prompts: false,
            log_prompts_redact: true,
            public_base_url: "http://localhost:3000".into(),
            jwt_issuer: "yume".into(),
            google_android_client_id: None,
            deepseek_api_key: None,
            opencode_base_url: "http://localhost:4096".into(),
            opencode_username: "opencode".into(),
            opencode_password: None,
            qdrant_url: "http://localhost:6334".into(),
            database_url: None,
            keydb_url: None,
        });

        let state = ChatState { config };

        Router::new()
            .route("/v1/chat/stream", post(chat_stream))
            .with_state(state)
    }

    #[tokio::test]
    async fn chat_stream_returns_sse_events() {
        let server = TestServer::new(test_app()).unwrap();

        let request = ChatRequest {
            schema_version: CURRENT_SCHEMA_VERSION.into(),
            conversation_id: "".into(),
            idempotency_key: uuid::Uuid::new_v4().to_string(),
            message: yume_contracts::chat::ChatMessage {
                role: "user".into(),
                content: "Xin chào".into(),
                attachments: vec![],
            },
            retrieval: None,
            query_embedding: None,
            model_hint: None,
            stream: true,
        };

        let response = server.post("/v1/chat/stream").json(&request).await;

        // Should get 200 even in mock mode
        response.assert_status_ok();

        // Verify content-type is text/event-stream
        let content_type = response
            .headers()
            .get("content-type")
            .map(|v| v.to_str().unwrap_or(""))
            .unwrap_or("");
        assert!(
            content_type.contains("text/event-stream"),
            "Expected text/event-stream, got: {content_type}"
        );
    }

    #[tokio::test]
    async fn chat_stream_rejects_wrong_schema_version() {
        let server = TestServer::new(test_app()).unwrap();

        let request = ChatRequest {
            schema_version: "2020-01-01".into(),
            conversation_id: "".into(),
            idempotency_key: uuid::Uuid::new_v4().to_string(),
            message: yume_contracts::chat::ChatMessage {
                role: "user".into(),
                content: "Hello".into(),
                attachments: vec![],
            },
            retrieval: None,
            query_embedding: None,
            model_hint: None,
            stream: true,
        };

        let response = server.post("/v1/chat/stream").json(&request).await;
        response.assert_status_bad_request();
    }
}
