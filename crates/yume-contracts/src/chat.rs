//! Chat request and response types.
//!
//! Covers `POST /v1/chat` and `POST /v1/chat/stream`.

use serde::{Deserialize, Serialize};

/// Streaming chat request body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatRequest {
    pub schema_version: String,
    pub conversation_id: String,
    pub idempotency_key: String,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval: Option<RetrievalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_embedding: Option<QueryEmbedding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_hint: Option<String>,
    #[serde(default)]
    pub stream: bool,
}

/// A single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attachments: Vec<Attachment>,
}

/// An attachment to a chat message (e.g. OCR text).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachment {
    pub attachment_id: String,
    pub kind: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AttachmentMetadata>,
}

/// Metadata about an OCR attachment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttachmentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// Retrieval configuration for RAG-augmented chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievalConfig {
    pub enabled: bool,
    pub top_k: u32,
    pub sources: Vec<String>,
}

/// Android-computed query embedding for RAG retrieval.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryEmbedding {
    pub model_id: String,
    pub dimension: u32,
    pub vector: Vec<f32>,
    pub normalized: bool,
}

/// Non-streaming chat response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatResponse {
    pub conversation_id: String,
    pub message_id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<Vec<super::event::CitationData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
}
