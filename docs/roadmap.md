# MVP Roadmap

## Phase 0 — Contracts and skeleton

Deliverables:

- Monorepo layout.
- `yume-contracts`.
- Basic backend `/health`.
- Basic Android shell.
- Documentation and initial Makefile targets.

Acceptance:

- Contracts compile.
- Docs linked from README.
- Backend health check passes.

## Phase 1 — Google OAuth and session

Deliverables:

- Android Google Sign-In/Credential Manager.
- `POST /v1/auth/google`.
- JWT/session issuing.
- Basic session refresh/logout.

Acceptance:

- Verified Gmail login creates local session.
- Invalid token is rejected.

## Phase 2 — Streaming foundation

Deliverables:

- `POST /v1/chat/stream` with mock stream.
- Rust Android Core SSE parser.
- Compose streaming renderer.
- SSE replay harness.

Acceptance:

- Normal/slow/interrupted/malformed streams handled.
- UI can cancel stream.

## Phase 3 — Local OCR

Deliverables:

- CameraX / Photo Picker.
- ML Kit Text Recognition v2.
- OCR preview/edit.
- Rust OCR normalization.
- Android FFI smoke harness.

Acceptance:

- Vietnamese/English OCR works on sample receipts/docs.
- OCR result can be sent into chat.

## Phase 4 — On-device embedding

Deliverables:

- MediaPipe Text Embedder prototype.
- Embedding config endpoint.
- Rust/Kotlin embedding provider boundary.
- Benchmark E5 int8 conversion path.

Acceptance:

- Android computes query/chunk embeddings offline.
- Backend validates vectors.

## Phase 5 — Qdrant RAG

Deliverables:

- Qdrant collections.
- Document ingest with Android-provided vectors.
- Retrieval with user filters.
- Sanitized `<rag_context>`.
- RAG eval harness.

Acceptance:

- hit@5/hit@10 baseline measured.
- No metadata leakage in prompts.

## Phase 6 — OpenCode + DeepSeek

Deliverables:

- Private OpenCode Serve.
- DeepSeek provider config.
- `mobile-chat`, `rag-answer`, `answer-reviewer` agents.
- Mock DeepSeek and Mock OpenCode.

Acceptance:

- End-to-end chat stream works.
- DeepSeek key never appears in Android.
- OpenCode is not publicly exposed.

## Phase 7 — Web research agent

Deliverables:

- `deep-research` subagent.
- Controlled public web access.
- Citation requirements.
- Reviewer safety pass.

Acceptance:

- Web research cites sources.
- Internal/private URL access is blocked.

## Phase 8 — Hardening and production prep

Deliverables:

- Prompt snapshot harness.
- Security harness.
- Load/latency harness.
- Observability.
- Retention cleaner.
- Kubernetes manifests/Helm/Kustomize.

Acceptance:

- Security checklist passes.
- Load tests meet target.
- Retention deletes expired data.

## Major risks

| Risk | Mitigation |
|---|---|
| Android embedding quality/latency | Benchmark MediaPipe USE and E5 int8 on real devices early |
| UniFFI streaming lifecycle bugs | Use SSE replay and Android FFI smoke harness from the start |
| OpenCode API drift | Pin production version/digest and maintain mock OpenCode |
| Prompt/metadata leakage | Prompt snapshot + answer reviewer + redaction layer |
| Qdrant schema migration | Version collections by embedding model |
| Local/self-hosted ops burden | Add backup/restore and retention jobs early |

## Production questions still open

1. Exact embedding model for production: MediaPipe USE or E5 int8?
2. Which Kubernetes packaging style: raw manifests, Kustomize, or Helm?
3. Which Postgres deployment: managed, local StatefulSet, or external?
4. What exact latency targets define acceptable p95 first-token latency?
5. What prompt logging level is acceptable in production?
6. Which web domains/categories should research mode block by policy?
