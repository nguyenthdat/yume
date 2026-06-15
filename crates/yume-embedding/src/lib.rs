//! Text embedding utilities for on-device and server-side models.
//!
//! Manages model configuration, chunking strategies, embedding job
//! orchestration, and normalization for on-device (MediaPipe/TFLite)
//! and server-side embedding models.
//!
//! Currently a stub — populated in Swing 0.7+.

/// Standard embedding dimension for multilingual-e5-small.
pub const DEFAULT_DIMENSION: u32 = 384;

/// Distance metric used for vector similarity.
pub const DEFAULT_DISTANCE: &str = "cosine";

/// Prefix for query embeddings (E5-style models).
pub const QUERY_PREFIX: &str = "query: ";

/// Prefix for passage embeddings (E5-style models).
pub const PASSAGE_PREFIX: &str = "passage: ";
