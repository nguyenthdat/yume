package com.yume.network

/** Kotlin data classes matching the Yume Rust API contracts. */

data class ChatRequest(
    val schema_version: String = "2026-06-15",
    val conversation_id: String = "",
    val idempotency_key: String = java.util.UUID.randomUUID().toString(),
    val message: ChatMessage,
    val retrieval: RetrievalConfig? = null,
    val query_embedding: QueryEmbedding? = null,
    val model_hint: String? = null,
    val stream: Boolean = true
)

data class ChatMessage(
    val role: String = "user",
    val content: String,
    val attachments: List<Attachment> = emptyList()
)

data class Attachment(
    val attachment_id: String,
    val kind: String,
    val text: String,
    val metadata: AttachmentMetadata? = null
)

data class AttachmentMetadata(
    val language: String? = null,
    val page_count: Int? = null,
    val confidence: Float? = null
)

data class RetrievalConfig(
    val enabled: Boolean = false,
    val top_k: Int = 8,
    val sources: List<String> = emptyList()
)

data class QueryEmbedding(
    val model_id: String,
    val dimension: Int,
    val vector: List<Float>,
    val normalized: Boolean
)

// SSE events matching yume-contracts ChatEvent
sealed class ChatEvent {
    data class ChatStarted(
        val conversation_id: String,
        val message_id: String
    ) : ChatEvent()

    data class MessageDelta(
        val seq: Long,
        val delta: TextDelta
    ) : ChatEvent()

    data class Citation(
        val seq: Long,
        val citation: CitationData
    ) : ChatEvent()

    data class Usage(
        val input_tokens: Long,
        val output_tokens: Long
    ) : ChatEvent()

    data class Done(
        val finish_reason: String
    ) : ChatEvent()

    data class Error(
        val code: String,
        val message: String,
        val retry_after_ms: Long? = null,
        val recoverable: Boolean = false
    ) : ChatEvent()
}

data class TextDelta(val text: String)
data class CitationData(val source: String, val title: String, val display_ref: String)
