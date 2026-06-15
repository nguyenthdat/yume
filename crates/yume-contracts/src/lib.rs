//! Yume shared API contracts.
//!
//! This crate is the single source of truth for all DTOs and schemas
//! shared between the Rust backend, Rust Android Core, and Kotlin FFI layer.
//!
//! ## Module map
//!
//! | Module | Endpoint area |
//! |---|---|
//! | [`chat`] | `POST /v1/chat`, `POST /v1/chat/stream` |
//! | [`session`] | `POST /v1/auth/*`, `POST /v1/session` |
//! | [`ocr`] | `POST /v1/ocr/cleanup` |
//! | [`document`] | `POST /v1/document/ingest` |
//! | [`embedding`] | `GET /v1/embedding/config`, `/v1/embedding/jobs/*` |
//! | [`error`] | Shared error response contract |
//! | [`event`] | SSE `ChatEvent` stream types |
//! | [`schema`] | Schema version constants |

pub mod chat;
pub mod document;
pub mod embedding;
pub mod error;
pub mod event;
pub mod ocr;
pub mod schema;
pub mod session;
