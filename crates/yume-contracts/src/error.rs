//! Shared error contract.
//!
//! Used by every endpoint for consistent error reporting.

use serde::{Deserialize, Serialize};

/// Standard error codes returned by the Yume backend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    Unauthorized,
    RateLimited,
    BadRequest,
    ProviderError,
    StreamInterrupted,
    Internal,
}

/// Standard error response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    pub recoverable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
