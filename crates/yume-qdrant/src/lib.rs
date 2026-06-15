//! Qdrant vector database client.
//!
//! Handles search, upsert, collection management, and RAG retrieval
//! against the Qdrant instance. Controlled exclusively by the backend.
//!
//! Currently a stub — populated in Swing 0.4+.

/// Qdrant collection names used by Yume.
pub mod collections {
    pub const MEMORIES: &str = "yume_memories";
    pub const OCR_CHUNKS: &str = "yume_ocr_chunks";
    pub const DOCUMENTS: &str = "yume_documents";
    pub const CONVERSATIONS: &str = "yume_conversations";
}
