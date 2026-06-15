---
name: mobile-ai-architecture
description: Design and implement native Android mobile AI architecture using Rust core libraries exposed through UniFFI, OCR capture/recognition, LLM chatbot flows, embeddings, vector search, tool-using agentic RAG, privacy/offline tradeoffs, and QA contracts. Use whenever Android native, Kotlin, Rust UniFFI, OCR, scanned documents, LLM chat, RAG, vector database, embeddings, retrieval agent, mobile AI scaffold, rerun, update, modify, supplement, or redo architecture is mentioned. Do not use for generic web-only apps or backend-only RAG unless Android/Rust/mobile constraints are relevant.
---

# Mobile AI Architecture Skill

Use this skill to create practical architecture and starter implementation plans for native Android mobile AI applications with a Rust core, UniFFI bindings, OCR, LLM chatbot UX, and agentic RAG.

## Operating Model

- Make the architecture boundary-first: Android UI, Android services, Rust core, OCR provider, local storage, vector index, model adapter, and agent tools each need clear ownership.
- Keep base code minimal until contracts are stable. Mobile AI stacks fail most often at build boundaries, threading, model latency, and schema mismatch.
- Prefer typed contracts over opaque maps at the UniFFI boundary. Use serialized payloads only when versioning or provider-neutral adapters make typed APIs too brittle.
- Treat privacy as an architectural input, not a later policy. Decide what stays on-device, what can leave the device, and what must be redacted.
- Design for offline degradation: OCR-only capture, local index search, queued embedding, and cached chat history should fail independently.

## Architecture Checklist

- Android shell: Jetpack Compose or native views, CameraX/document capture, lifecycle-aware services, permissions, background work, and local encrypted storage.
- Rust core: domain models, OCR text normalization, chunking, embedding adapters, retrieval scoring, prompt assembly, and deterministic unit tests.
- UniFFI boundary: coarse API functions, explicit records/enums/errors, async policy, Kotlin package layout, generated bindings, native library packaging, and cargo-ndk commands.
- OCR pipeline: capture/import, preprocessing, recognition, confidence, language, page/layout metadata, deduplication, and human correction loop.
- Chatbot: conversation state, model adapter, streaming policy, prompt templates, safety filters, citations, retry/fallback, and UX for uncertainty.
- Agentic RAG: ingestion queue, chunking, embeddings, vector store, hybrid search, reranking, source citations, tool contracts, permissions, and eval cases.
- QA: compare contracts across boundaries and run incremental verification after each module.

## Rust UniFFI Base Code

When asked to generate or review Rust base code with UniFFI, use `references/rust-uniffi-android-base.md` as the starter template. Adapt package names, module names, versions, and commands to the current repository before editing.

Required starter contracts:

- `OcrDocument`: document id, language, pages, text blocks, confidence, source URI/path, timestamps.
- `IndexResult`: document id, chunk count, embedding status, warnings.
- `ChatRequest`: conversation id, user message, retrieval filters, streaming preference.
- `ChatAnswer`: answer text, citations, tool calls, safety flags, fallback reason.
- `YumeError` or app-specific error enum with recoverable vs fatal cases.

## OCR Guidance

- Capture structured OCR metadata before creating embeddings; RAG quality depends on source, page, region, confidence, and correction status.
- For privacy-first apps, prefer on-device OCR and store only user-approved text. For accuracy-first apps, add cloud OCR as an explicit fallback with consent.
- Normalize text in Rust when it feeds chunking or retrieval so Android and future platforms share behavior.

## Chatbot And Agentic RAG Guidance

- Keep ingestion separate from chat so a failed scan does not corrupt conversation state.
- Use typed agent tools such as `search_documents`, `summarize_document`, `create_note`, and `delete_index_entry`; require explicit permission for destructive tools.
- Attach citations to every answer sourced from OCR documents. Low OCR confidence should lower answer confidence or trigger a clarification.
- Define evaluation prompts for hallucination, conflicting documents, stale index, low-confidence OCR, and no-retrieval results.

## Output Format

Return architecture work in this order:

1. Context and assumptions.
2. Proposed module/file tree.
3. Boundary contracts.
4. Base-code or edit plan.
5. Verification commands.
6. Risks and follow-up decisions.

## Trigger Tests

Should trigger:

- "Design an Android OCR chatbot with Rust UniFFI."
- "Generate base Rust code for a native Android RAG app."
- "Redo only the OCR pipeline based on previous results."
- "Update the agentic RAG architecture for offline embeddings."
- "How should Kotlin call my Rust retrieval engine through UniFFI?"
- "Plan a mobile document scanner that chats with scanned notes."
- "Verify the Rust/Kotlin/OCR/RAG contracts."
- "Add local vector search to the mobile AI architecture."

Should not trigger:

- "Build a web-only RAG dashboard."
- "Write a generic Android to-do list."
- "Explain Rust ownership basics."
- "Create a backend-only LangChain pipeline."
- "Design a PostgreSQL schema unrelated to mobile AI."
- "Summarize this PDF without mobile app work."
- "Add CSS to a landing page."
- "Review a Kotlin file unrelated to OCR, AI, Rust, or RAG."
