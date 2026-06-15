# Rust Crate Design

## Design principles

- Rust owns contracts, core logic, backend, streaming, policy, Qdrant clients, and OpenCode clients.
- Kotlin remains Android UI/platform integration.
- Contracts should be shared across Android Core and Backend.
- Provider-specific details should live behind traits.
- Every external dependency must be mockable.

## Proposed crates

```txt
crates/
  yume-contracts/
  yume-core/
  yume-ffi/
  yume-backend/
  yume-opencode-client/
  yume-qdrant/
  yume-embedding/
  yume-sse/
  yume-policy/
  yume-store/
  yume-observability/
  yume-mock/
```

## `yume-contracts`

Shared DTOs and schemas.

Modules:

```txt
chat.rs
session.rs
ocr.rs
document.rs
embedding.rs
error.rs
event.rs
schema.rs
```

Types:

- `ChatRequest`.
- `ChatResponse`.
- `ChatEvent`.
- `Session`.
- `GoogleAuthRequest`.
- `OcrDocument`.
- `DocumentIngestRequest`.
- `EmbeddingVector`.
- `ErrorResponse`.

## `yume-core`

Android-compatible core logic.

Modules:

```txt
client.rs
conversation.rs
request_builder.rs
sse_parser.rs
ocr_normalize.rs
embedding_jobs.rs
retry.rs
cancel.rs
```

Responsibilities:

- Build request DTOs.
- Parse backend SSE.
- Normalize OCR text.
- Manage local conversation stream state.
- Coordinate with Android embedding provider.
- Map backend errors to app-level errors.

## `yume-ffi`

UniFFI exports.

Modules:

```txt
lib.rs
exported_client.rs
exported_types.rs
foreign_traits.rs
error_mapping.rs
```

Responsibilities:

- Export stable Kotlin-callable API.
- Expose records/enums/errors.
- Define foreign traits for Android platform callbacks/providers.
- Hide internal Rust implementation details.

## `yume-backend`

Axum backend control plane.

Modules:

```txt
main.rs
routes/
middleware/
state.rs
auth.rs
stream.rs
retention.rs
```

Responsibilities:

- Public API.
- Google OAuth verification.
- Local session/JWT.
- SSE response generation.
- Rate limiting.
- Request validation.
- Prompt policy enforcement.
- OpenCode/Qdrant integration.

## `yume-opencode-client`

Private OpenCode HTTP client.

Responsibilities:

- Session/message APIs.
- Event streaming APIs.
- Agent selection.
- Error normalization.
- Mock compatibility.

## `yume-qdrant`

Qdrant collection and query layer.

Responsibilities:

- Collection creation.
- Payload indexes.
- Search filters.
- Upsert/delete.
- Retention cleanup.
- Sanitized context extraction.

## `yume-embedding`

Embedding contracts and validation.

Responsibilities:

- Embedding model config.
- Vector validation.
- Prefix policy: `query: ` and `passage: `.
- Embedding job types.
- Mock embeddings for tests.

Note: actual mobile inference runtime lives in Kotlin platform layer.

## `yume-sse`

Shared SSE utilities.

Responsibilities:

- Parse provider/OpenCode/backend events.
- Generate backend app-level events.
- Replay harness support.
- Duplicate/malformed/missing done handling.

## `yume-policy`

Security and prompt policies.

Responsibilities:

- Prompt redaction.
- Tool permissions.
- Context sanitization.
- Token budget enforcement.
- Metadata leakage checks.

## `yume-store`

Production system-of-record store.

Recommendation: use Postgres for users, sessions, conversations, prompt logs, quotas, retention jobs, and audit events. Qdrant should not be the only system of record.

## Verification commands

Expected future commands:

```txt
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
make contract-test
make sse-replay
make android-ffi-smoke
```
