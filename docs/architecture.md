# Yume System Architecture

## Context

Yume is a native Android AI chat app similar to Claude Chat, backed by a stronger agentic backend. The app uses Kotlin for UI and Android platform APIs, Rust for shared core logic and backend control plane, OpenCode as the private multi-agent runtime, DeepSeek as the primary model provider, Qdrant for memory/RAG, and local Android OCR.

## Primary goals

- Claude/ChatGPT-style mobile chat.
- Realtime streaming responses.
- Local OCR for images/documents on Android.
- Long-term memory and RAG through Qdrant.
- OpenCode Serve as private multi-agent orchestration runtime.
- DeepSeek API as primary LLM provider.
- Rust as the primary implementation language for core logic, backend wrapper, API contracts, RAG pipeline, streaming, and client library.
- Kotlin only for Android UI and Android platform integration.

## Non-negotiable boundaries

- Android must not call DeepSeek directly.
- Android must not call OpenCode directly.
- Rust backend wrapper is the public control plane.
- OpenCode is an internal/private runtime, not a public API.
- Qdrant is controlled by backend Rust.
- DeepSeek API key must never appear in Android.
- Streaming must be testable by replay harness.
- External services must be mockable.
- Local/dev, staging, and production deployment must be easy.

## High-level flow

```txt
Android Kotlin UI
→ Rust Android Core via UniFFI/JNI
→ Rust Backend Wrapper via HTTPS/SSE
→ Qdrant retrieve memory/RAG context
→ OpenCode Serve private agent orchestration
→ DeepSeek API generate answer
→ Stream tokens back to Android
```

## Architecture diagram

![Yume architecture](images/architecture.png)

Source D2: [`diagrams/architecture.d2`](diagrams/architecture.d2). Generated PNG: [`images/architecture.png`](images/architecture.png).

## Component breakdown

### Android App

Stack:

- Kotlin.
- Jetpack Compose.
- CameraX or Android Photo Picker.
- ML Kit Text Recognition v2 for local OCR.
- MediaPipe/TFLite for local text embeddings.
- UniFFI/JNI bridge into Rust Android Core.
- Lightweight local storage for UI cache/session state.

Owns:

- Chat UI.
- Conversation list UI.
- Streaming message rendering.
- Image picker/camera capture.
- OCR local recognition.
- OCR preview/edit screen.
- Settings/privacy screens.
- Android-local embedding runtime.
- Calling Rust Core.

Does not own:

- Prompt orchestration.
- RAG retrieval policy.
- Agent routing.
- DeepSeek/OpenCode credentials.
- Long-term memory policy.
- Tool permissions.

### Rust Android Core

Runs on Android through UniFFI/JNI.

Responsibilities:

- DTO/API contracts.
- Chat request builder.
- Streaming event parser.
- Conversation client state.
- OCR text normalization.
- Attachment metadata preparation.
- Backend API client.
- Error mapping for Kotlin UI.
- Retry/cancel stream logic.
- Embedding job orchestration while Kotlin owns the platform runtime.

### Rust Backend Wrapper

Recommended stack:

- Axum.
- Tokio.
- Tower middleware.
- Reqwest.
- Serde/Schemars.
- Tracing/OpenTelemetry.

Responsibilities:

- Google OAuth token verification.
- Local session/JWT issuing.
- API gateway for Android.
- SSE streaming proxy/normalizer.
- Rate limit/quota.
- Qdrant search/writeback.
- Prompt/context assembly.
- OpenCode private client.
- Agent route selection.
- Secret management.
- Retention cleanup.
- Observability.
- Security guardrails.

### OpenCode Server

OpenCode runs via `opencode serve` in a private network. It is not public.

Responsibilities:

- Multi-agent orchestration.
- Primary agent and subagents.
- Skills.
- Deep research agent.
- DeepSeek provider calls.
- Final answer synthesis.
- Reviewer pass.

### Qdrant

Qdrant is controlled by backend Rust. MVP should not allow OpenCode to query Qdrant directly.

Used for:

- User memory.
- Conversation summaries.
- OCR document chunks.
- Uploaded document chunks.
- Agent run summaries.

## Detailed data flows

### Chat stream

1. Android user sends a message.
2. Android computes query embedding locally if retrieval is enabled.
3. Kotlin calls Rust Core through UniFFI/JNI.
4. Rust Core builds `ChatRequest` and calls `POST /v1/chat/stream`.
5. Backend authenticates JWT/session.
6. Backend applies quota, rate limit, prompt policy, and request validation.
7. Backend searches Qdrant using the Android-provided query embedding.
8. Backend builds sanitized `<rag_context>`.
9. Backend calls OpenCode Serve in private network.
10. OpenCode routes to agents/subagents and calls DeepSeek.
11. DeepSeek streams model chunks.
12. OpenCode returns agent/model events.
13. Backend normalizes all events into stable `ChatEvent` SSE.
14. Rust Core parses SSE.
15. Kotlin renders message deltas.

### OCR ingest

1. Android captures/imports image.
2. ML Kit recognizes text locally.
3. Android shows OCR preview/edit UI.
4. User confirms or edits text.
5. Rust Core normalizes OCR text and chunks it.
6. Android embedding provider embeds chunks locally.
7. Rust Core sends `DocumentIngestRequest` containing text chunks and vectors.
8. Backend validates vectors and stores sanitized chunks in Qdrant.

### Memory writeback with Android-only embeddings

Backend cannot embed memory summaries by itself if embeddings are Android-only.

Solution:

1. Backend creates a pending memory embedding job.
2. Android claims embedding jobs when online.
3. Android computes vector locally.
4. Android uploads vector.
5. Backend validates and upserts into Qdrant.

## Repository structure

```txt
yume/
  README.md
  AGENTS.md
  opencode.jsonc
  docker-compose.dev.yml
  Makefile
  .env.example

  android/
    app/
      src/main/java/com/yume/
        MainActivity.kt
        ui/
        platform/
        rust/

  crates/
    yume-core/
    yume-ffi/
    yume-contracts/
    yume-backend/
    yume-opencode-client/
    yume-qdrant/
    yume-embedding/
    yume-sse/
    yume-policy/
    yume-store/
    yume-observability/
    yume-mock/

  opencode/
    agents/
      mobile-chat.md
      deep-research.md
      ocr-cleanup.md
      rag-answer.md
      reasoning.md
      answer-reviewer.md
    skills/
      qdrant-rag/SKILL.md
      ocr-cleanup/SKILL.md
      deepseek-chat/SKILL.md
      safe-final-answer/SKILL.md
      agent-routing/SKILL.md
      android-ocr/SKILL.md

  harness/
    local-stack/
    mock-deepseek/
    mock-opencode/
    sse-replay/
    rag-eval/
    ocr-eval/
    android-ffi-smoke/
    agent-e2e/
    prompt-snapshot/
    contract/
    security/
    load/

  infra/
    qdrant/
    opencode/
    backend/
    observability/

  docs/
    architecture.md
    api.md
    dev-harnesses.md
    rag-policy.md
    agent-policy.md
    android-build.md
    rust-design.md
    embedding.md
    security.md
    deployment.md
    roadmap.md
```

## Key architectural decisions

| Decision | Recommendation | Trade-off |
|---|---|---|
| Android direct DeepSeek/OpenCode | Never | More backend complexity, much better security |
| Backend as control plane | Yes | Backend owns auth/quota/policy complexity |
| SSE through backend | Yes | Backend must normalize provider/OpenCode streams |
| OCR local first | Yes | Better privacy/offline, cloud OCR may be more accurate later |
| Embedding local on Android | Yes | Zero provider cost/offline, more mobile model work |
| Qdrant controlled by backend | Yes | Safer MVP, less direct agent flexibility |
| OpenCode private runtime | Yes | Requires client adapter and version pinning |
| Rust owns contracts | Yes | More build complexity, fewer schema mismatches |
| Reasoning display | No | Safer, avoids chain-of-thought exposure |
| Data location | Local/self-hosted Docker/Kubernetes | Easier control, operator owns backups/security |
