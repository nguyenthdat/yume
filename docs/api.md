# Backend API Contracts

## API principles

- Android only talks to Rust Backend Wrapper.
- Backend is the only public API surface.
- OpenCode, DeepSeek, Qdrant, and secrets are never exposed to Android.
- Contracts should be generated from Rust types where possible.
- Every request carries a schema version.
- Streaming events use stable app-level event types independent of DeepSeek/OpenCode internals.

## Authentication endpoints

### `POST /v1/auth/google`

Android obtains a Google ID token through Android Credential Manager / Google Sign-In, then sends it to backend.

Request:

```json
{
  "google_id_token": "eyJ...",
  "device_id": "device-public-id",
  "app_version": "0.1.0",
  "platform": "android",
  "locale": "vi-VN"
}
```

Response:

```json
{
  "user": {
    "display_name": "Dat Nguyen",
    "email": "user@gmail.com"
  },
  "session": {
    "session_id": "sess_...",
    "access_token": "jwt...",
    "refresh_token": "refresh...",
    "expires_at": "2026-06-15T12:00:00Z"
  },
  "capabilities": {
    "streaming": true,
    "ocr_ingest": true,
    "rag": true,
    "local_embedding": true
  }
}
```

Backend must verify:

- Google issuer.
- Audience matches configured Android OAuth client.
- Token expiration.
- `email_verified = true`.
- Stable Google `sub` mapped to local user.

### `POST /v1/auth/refresh`

Refresh local Yume session token.

### `POST /v1/auth/logout`

Invalidate local session/refresh token.

## Session endpoints

### `POST /v1/session`

Creates or resumes a device/app session after auth.

```json
{
  "device_id": "device-public-id",
  "app_version": "0.1.0",
  "platform": "android",
  "locale": "vi-VN"
}
```

### `GET /v1/session/:id`

Returns session state and capabilities.

## Embedding endpoints

Because embeddings run on Android, backend exposes configuration and pending jobs.

### `GET /v1/embedding/config`

```json
{
  "embedding_model_id": "multilingual_e5_small_int8_v1",
  "dimension": 384,
  "distance": "cosine",
  "query_prefix": "query: ",
  "passage_prefix": "passage: ",
  "max_tokens": 512,
  "runtime": "android"
}
```

### `POST /v1/embedding/jobs/claim`

Android claims pending backend-generated texts that need local embeddings, such as memory summaries.

### `POST /v1/embedding/jobs/:id/complete`

Android completes an embedding job.

```json
{
  "model_id": "multilingual_e5_small_int8_v1",
  "dimension": 384,
  "normalized": true,
  "vector": [0.0123, -0.0321]
}
```

Backend validates model ID, dimension, vector norm, finite values, and payload size.

## Chat endpoints

### `POST /v1/chat`

Non-streaming chat for tests and fallback.

### `POST /v1/chat/stream`

Streaming chat endpoint. Uses `text/event-stream` response.

Request:

```json
{
  "schema_version": "2026-06-15",
  "conversation_id": "conv_...",
  "idempotency_key": "uuid",
  "message": {
    "role": "user",
    "content": "Tóm tắt tài liệu này",
    "attachments": [
      {
        "attachment_id": "att_...",
        "kind": "ocr_text",
        "text": "...",
        "metadata": {
          "language": "vi,en",
          "page_count": 1,
          "confidence": 0.91
        }
      }
    ]
  },
  "retrieval": {
    "enabled": true,
    "top_k": 8,
    "sources": ["memories", "ocr_chunks", "documents"]
  },
  "query_embedding": {
    "model_id": "multilingual_e5_small_int8_v1",
    "dimension": 384,
    "vector": [0.01, -0.02],
    "normalized": true
  },
  "model_hint": "balanced",
  "stream": true
}
```

SSE event examples:

```txt
event: chat.started
data: {"conversation_id":"conv_...","message_id":"msg_..."}

event: message.delta
data: {"seq":1,"delta":{"text":"Xin chào"}}

event: citation
data: {"seq":20,"citation":{"source":"ocr_chunk","title":"Receipt 01","display_ref":"doc:receipt-01"}}

event: usage
data: {"input_tokens":1200,"output_tokens":300}

event: done
data: {"finish_reason":"stop"}
```

Error event:

```txt
event: error
data: {"code":"RATE_LIMITED","message":"Too many requests","retry_after_ms":30000,"recoverable":true}
```

## OCR endpoints

### `POST /v1/ocr/cleanup`

Optional server-side cleanup. MVP should prefer deterministic local cleanup in Rust Core.

```json
{
  "text": "...",
  "language_hints": ["vi", "en"],
  "preserve_layout": true
}
```

## Document endpoints

### `POST /v1/document/ingest`

```json
{
  "document_id": "doc_...",
  "source": "android_ocr",
  "title": "Receipt June",
  "language": "vi-en",
  "privacy_level": "private",
  "ocr_metadata": {
    "confidence": 0.88,
    "pages": 1,
    "corrected_by_user": true
  },
  "chunks": [
    {
      "chunk_id": "chunk_001",
      "text": "passage text",
      "embedding": {
        "model_id": "multilingual_e5_small_int8_v1",
        "dimension": 384,
        "normalized": true,
        "vector": []
      }
    }
  ]
}
```

## Model endpoints

### `GET /v1/models`

Returns user-visible model capabilities, not provider secrets.

```json
{
  "default": "balanced",
  "models": [
    {"id":"fast","display_name":"Fast","streaming":true},
    {"id":"balanced","display_name":"Balanced","streaming":true},
    {"id":"research","display_name":"Research","streaming":true}
  ]
}
```

## Error contract

```json
{
  "error": {
    "code": "UNAUTHORIZED|RATE_LIMITED|BAD_REQUEST|PROVIDER_ERROR|STREAM_INTERRUPTED|INTERNAL",
    "message": "Human-safe message",
    "recoverable": true,
    "retry_after_ms": 30000,
    "request_id": "req_..."
  }
}
```
