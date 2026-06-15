//! Embedding contract types.
//!
//! Covers `GET /v1/embedding/config`, `/v1/embedding/jobs/*`.
//! Android owns the embedding runtime; backend provides config and job coordination.

use serde::{Deserialize, Serialize};

/// Embedding model configuration served by the backend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingConfig {
    pub embedding_model_id: String,
    pub dimension: u32,
    pub distance: String,
    pub query_prefix: String,
    pub passage_prefix: String,
    pub max_tokens: u32,
    pub runtime: String,
}

/// A pending embedding job the backend needs Android to compute.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmbeddingJob {
    pub job_id: String,
    pub text: String,
    pub model_id: String,
    pub prefix: String,
    pub created_at: String,
}

/// Response when Android claims pending embedding jobs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmbeddingJobsResponse {
    pub jobs: Vec<EmbeddingJob>,
}

/// Request body when Android submits a completed embedding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingJobCompleteRequest {
    pub model_id: String,
    pub dimension: u32,
    pub normalized: bool,
    pub vector: Vec<f32>,
}

/// Raw embedding vector with metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingVector {
    pub model_id: String,
    pub dimension: u32,
    pub normalized: bool,
    pub vector: Vec<f32>,
}
