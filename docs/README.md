# Yume Architecture Docs

This folder contains the technical architecture for Yume.

## Documents

1. [System architecture](architecture.md) — full component model, D2 diagram, data flow, boundaries, repo layout, and architectural decisions.
2. [Backend API contracts](api.md) — proposed REST/SSE endpoints, DTOs, errors, and streaming events.
3. [Android + Rust UniFFI design](android-build.md) — Kotlin responsibilities, Rust Android Core, OCR flow, UniFFI bridge, Android 15/16 target, and FFI smoke testing.
4. [Rust crate design](rust-design.md) — monorepo crate layout, dependency boundaries, and Rust ownership of contracts/core/backend logic.
5. [On-device embedding design](embedding.md) — Android-local embedding model choices, offline RAG flow, Qdrant vector compatibility, and embedding jobs.
6. [RAG policy and Qdrant schema](rag-policy.md) — Qdrant collections, payload schemas, retrieval pipeline, memory writeback, and context sanitization.
7. [OpenCode agent and skill policy](agent-policy.md) — OpenCode private runtime, agent/skill config examples, permissions, DeepSeek usage, and web research controls.
8. [Dev harnesses](dev-harnesses.md) — local stack, mocks, SSE replay, Android FFI smoke, OCR/RAG eval, agent E2E, prompt snapshot, security, and load harnesses.
9. [Security model](security.md) — Google OAuth, secrets, network policy, prompt/log redaction, Qdrant isolation, and production checklist.
10. [Deployment](deployment.md) — Docker Compose dev stack and Kubernetes staging/production design.
11. [MVP roadmap](roadmap.md) — implementation phases, acceptance gates, risks, and production questions.

## Current architecture decisions

| Area | Decision |
|---|---|
| Android app | Kotlin + Jetpack Compose |
| Android target | Latest two Android versions: Android 15/16 |
| OCR | Local on Android first, ML Kit Text Recognition v2 |
| Embedding | Local on Android, offline-capable |
| Core logic | Rust |
| Android bridge | UniFFI/JNI |
| Backend | Rust Axum wrapper/control plane |
| Agent runtime | OpenCode Serve, private network only |
| Model provider | DeepSeek API through OpenCode/backend only |
| Vector database | Qdrant controlled by backend |
| Auth | Google OAuth / Gmail |
| Multi-tenant | No |
| Data location | Local/self-hosted Docker/Kubernetes |
| Retention | Default 30 days, configurable |
| Offline support | OCR + embedding |
| Web research | Allowed through controlled research agent |
| Reasoning display | Do not expose reasoning to user |
| Deployment | Docker Compose for local/dev, Kubernetes for staging/prod |
