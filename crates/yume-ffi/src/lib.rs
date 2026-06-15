//! UniFFI bindings for the Yume Android Rust Core.
//!
//! Exposes Rust types and functions to Kotlin via UniFFI-generated JNI.
//! This crate is compiled into `libyume_ffi.so` and loaded by the Android app.
//!
//! The interface is defined in `src/yume.udl` — UniFFI generates both the Rust
//! scaffolding and Kotlin bindings from it at build time.

// Include UniFFI-generated scaffolding (must match the UDL namespace)
uniffi::include_scaffolding!("yume");

use yume_contracts::schema::CURRENT_SCHEMA_VERSION;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum YumeError {
    #[error("invalid input")]
    InvalidInput,
    #[error("serialization error: {0}")]
    SerializationError(String),
}

// ---------------------------------------------------------------------------
// Exported functions (matching UDL definitions)
// ---------------------------------------------------------------------------

/// Returns the current API schema version.
pub fn current_schema_version() -> Result<String, YumeError> {
    Ok(CURRENT_SCHEMA_VERSION.to_string())
}

/// Builds a JSON chat request from message content and conversation ID.
pub fn build_chat_request_json(
    message_content: String,
    conversation_id: String,
) -> Result<String, YumeError> {
    let request = serde_json::json!({
        "schema_version": CURRENT_SCHEMA_VERSION,
        "conversation_id": conversation_id,
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
        "message": {
            "role": "user",
            "content": message_content
        },
        "stream": true
    });

    serde_json::to_string(&request).map_err(|e| YumeError::SerializationError(e.to_string()))
}

// ---------------------------------------------------------------------------
// DTO types (must match UDL dictionary definitions)
// ---------------------------------------------------------------------------

pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct ChatRequest {
    pub schema_version: String,
    pub conversation_id: String,
    pub idempotency_key: String,
    pub message: ChatMessage,
    pub stream: bool,
}

pub struct TextDelta {
    pub text: String,
}

pub struct ChatStarted {
    pub conversation_id: String,
    pub message_id: String,
}

pub struct MessageDelta {
    pub seq: u64,
    pub delta: TextDelta,
}

pub struct ChatUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

pub struct ChatDone {
    pub finish_reason: String,
}

pub struct ChatError {
    pub code: String,
    pub message: String,
    pub retry_after_ms: Option<u64>,
    pub recoverable: bool,
}
