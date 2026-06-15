//! OCR contract types.
//!
//! Covers `POST /v1/ocr/cleanup` and OCR document metadata.

use serde::{Deserialize, Serialize};

/// Request to perform server-side OCR text cleanup (optional).
/// MVP should prefer deterministic local cleanup in Rust Core.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OcrCleanupRequest {
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub language_hints: Vec<String>,
    #[serde(default)]
    pub preserve_layout: bool,
}

/// Cleaned OCR text response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OcrCleanupResponse {
    pub cleaned_text: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub corrections: Vec<OcrCorrection>,
}

/// A single correction applied during cleanup.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OcrCorrection {
    pub original: String,
    pub corrected: String,
    pub position: usize,
}

/// OCR document metadata for chat/RAG.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrDocument {
    pub text: String,
    pub language: String,
    pub confidence: f32,
    pub page_count: u32,
    pub source_uri_hash: Option<String>,
    pub corrected_by_user: bool,
    pub timestamp: String,
}
