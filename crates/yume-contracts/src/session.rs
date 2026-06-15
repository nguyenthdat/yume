//! Auth and session contract types.
//!
//! Covers `POST /v1/auth/google`, `POST /v1/auth/refresh`,
//! `POST /v1/auth/logout`, and `POST /v1/session`.

use serde::{Deserialize, Serialize};

/// Request sent by Android after Google Sign-In / Credential Manager.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoogleAuthRequest {
    pub google_id_token: String,
    pub device_id: String,
    pub app_version: String,
    pub platform: String,
    pub locale: String,
}

/// Response returned after successful Google auth.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoogleAuthResponse {
    pub user: User,
    pub session: Session,
    pub capabilities: Capabilities,
}

/// Authenticated user profile (derived from Google token claims).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub display_name: String,
    pub email: String,
}

/// Local Yume session tokens.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Session {
    pub session_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
}

/// Feature flags returned after auth.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capabilities {
    pub streaming: bool,
    pub ocr_ingest: bool,
    pub rag: bool,
    pub local_embedding: bool,
}

/// Request to refresh a Yume session token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Request to create or resume a device session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionRequest {
    pub device_id: String,
    pub app_version: String,
    pub platform: String,
    pub locale: String,
}

/// Response for session create/resume.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionResponse {
    pub session: Session,
    pub capabilities: Capabilities,
}
