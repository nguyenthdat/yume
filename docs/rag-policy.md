# RAG Policy and Qdrant Schema

## Core policy

- Backend Rust controls all Qdrant access.
- OpenCode should not query Qdrant directly in MVP.
- Backend sends only sanitized context to OpenCode.
- Do not expose raw Qdrant IDs, vector IDs, embeddings, raw scores, internal user IDs, or secret metadata.
- Every search must filter by user identity.

## Collections

Recommended collections:

- `memories`
- `conversation_summaries`
- `documents`
- `ocr_chunks`
- `agent_runs`

Use cosine distance. If using `multilingual-e5-small`, vector size is 384.

## Common payload fields

```json
{
  "user_id_hash": "...",
  "visibility": "private",
  "source_type": "ocr|chat|upload|agent",
  "language": "vi-en",
  "created_at": "2026-06-15T00:00:00Z",
  "updated_at": "2026-06-15T00:00:00Z",
  "expires_at": "2026-07-15T00:00:00Z",
  "safety_level": "normal",
  "embedding_model_id": "multilingual_e5_small_int8_v1",
  "embedding_runtime": "android",
  "embedding_dimension": 384,
  "embedding_created_at": "2026-06-15T00:00:00Z",
  "chunk_hash": "sha256...",
  "text_hash": "sha256..."
}
```

## `memories`

```json
{
  "memory_id": "mem_...",
  "memory_type": "preference|fact|project|instruction",
  "summary": "User prefers Rust backend and Kotlin UI.",
  "confidence": 0.92,
  "importance": 0.8,
  "source_conversation_id": "conv_...",
  "consent": "implicit"
}
```

## `conversation_summaries`

```json
{
  "conversation_id": "conv_...",
  "summary_id": "sum_...",
  "turn_start": 1,
  "turn_end": 30,
  "summary": "...",
  "model": "deepseek-v4-pro"
}
```

## `documents`

```json
{
  "document_id": "doc_...",
  "chunk_id": "doc_..._chunk_001",
  "title": "Receipt June",
  "chunk_index": 1,
  "mime_type": "text/plain",
  "source": "upload|ocr",
  "checksum": "sha256...",
  "text": "sanitized chunk text"
}
```

## `ocr_chunks`

```json
{
  "document_id": "doc_...",
  "chunk_id": "ocr_...",
  "page": 1,
  "block_range": [3, 8],
  "confidence": 0.86,
  "corrected_by_user": true,
  "text": "sanitized OCR text"
}
```

## `agent_runs`

Store summaries only, not chain-of-thought.

```json
{
  "agent_run_id": "run_...",
  "agent": "rag-answer",
  "task_type": "chat|ocr_cleanup|research",
  "outcome_summary": "...",
  "safety_flags": [],
  "conversation_id": "conv_..."
}
```

## Retrieval pipeline

1. Validate session/user.
2. Validate Android-provided query embedding.
3. Search Qdrant collections with strict user filters.
4. Deduplicate by document/chunk hash.
5. Apply relevance threshold.
6. Optional rerank later.
7. Apply token budget.
8. Sanitize payload.
9. Format `<rag_context>`.
10. Send to OpenCode.
11. Review final answer.
12. Write back summaries/memory jobs.

## Prompt context format

```xml
<rag_context>
[context_1]
source: memory
title: user preference
text: User prefers Rust backend, Kotlin UI, and local Android OCR.

[context_2]
source: ocr_chunk
title: OCR document
text: ...
</rag_context>

<user_message>
...
</user_message>
```

## Conflict policy

- Explicit user instructions override implicit memories.
- Newer explicit memory overrides older explicit memory unless low confidence.
- If memories conflict, answer with uncertainty or ask clarification.
- If context is insufficient, say context is insufficient instead of hallucinating.

## Retention

Default TTL is 30 days, configurable.

All Qdrant payloads should carry `expires_at`. Backend retention job deletes expired points.
