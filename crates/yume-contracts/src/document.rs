//! Document ingest contract types.
//!
//! Covers `POST /v1/document/ingest`.

use serde::{Deserialize, Serialize};

/// Request to ingest a document (OCR text + embeddings) into Qdrant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentIngestRequest {
    pub document_id: String,
    pub source: String,
    pub title: String,
    pub language: String,
    pub privacy_level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_metadata: Option<OcrMetadata>,
    pub chunks: Vec<DocumentChunk>,
}

/// Metadata captured during OCR on Android.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrMetadata {
    pub confidence: f32,
    pub pages: u32,
    pub corrected_by_user: bool,
}

/// A single text chunk with its Android-computed embedding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentChunk {
    pub chunk_id: String,
    pub text: String,
    pub embedding: ChunkEmbedding,
}

/// Embedding vector for a document chunk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkEmbedding {
    pub model_id: String,
    pub dimension: u32,
    pub normalized: bool,
    pub vector: Vec<f32>,
}
