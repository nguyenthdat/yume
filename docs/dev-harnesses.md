# Dev Harnesses

All external services must be mockable. Streaming must be replay-testable.

## 1. Local Stack Harness

Command:

```txt
make dev
```

Runs:

- Qdrant.
- Rust backend.
- OpenCode server.
- Mock DeepSeek or real DeepSeek.
- Optional observability.

Pass criteria:

- `/health` returns OK.
- Qdrant collections are present.
- Backend can call mock/real OpenCode.
- Chat stream works end to end.

## 2. Mock DeepSeek Harness

Command:

```txt
make test-mock-deepseek
```

Cases:

- Normal chat stream.
- Reasoning stream.
- Timeout.
- Rate limit.
- Provider error.
- Malformed streaming event.

## 3. Mock OpenCode Harness

Command:

```txt
make test-mock-opencode
```

Cases:

- Session create.
- Message send.
- Streaming response.
- Agent error.
- Tool permission denied.
- Invalid response.

## 4. SSE Replay Harness

Command:

```txt
make test-sse-replay
```

Cases:

- Normal stream.
- Slow stream.
- Interrupted stream.
- Duplicate event.
- Invalid JSON event.
- Done event missing.
- Network disconnect.

Pass criteria:

- Rust parser emits deterministic events.
- UI renderer can handle partial and invalid streams.
- Errors map to recoverable/non-recoverable app errors.

## 5. Android FFI Smoke Harness

Command:

```txt
make android-ffi-smoke
```

Cases:

- Load native library.
- Create client.
- Build chat request.
- Normalize OCR text.
- Parse stream event.
- Map Rust error to Kotlin error.
- Call Android embedding provider through UniFFI.

## 6. OCR Evaluation Harness

Command:

```txt
make ocr-eval
```

Inputs:

- Receipt images.
- Screenshots.
- Documents.
- Rotated images.
- Low-light images.
- Vietnamese/English mixed text.

Metrics:

- Character error rate.
- Word error rate.
- Cleanup quality.
- Latency.
- Crash rate.

## 7. RAG Evaluation Harness

Command:

```txt
make rag-eval
```

Metrics:

- hit@5.
- hit@10.
- context relevance.
- leakage check.
- retrieval latency.
- Vietnamese/English benchmark quality.

## 8. Agent E2E Harness

Command:

```txt
make agent-e2e
```

Cases:

- Simple chat.
- OCR cleanup.
- RAG grounded answer.
- Insufficient context.
- Conflicting memory.
- Reviewer catches hallucination.
- Answer does not reveal internal metadata.
- Web research with citations.

## 9. Prompt Snapshot Harness

Command:

```txt
make prompt-snapshot
```

Fail if prompt contains:

- Secret.
- Raw embedding.
- Internal user ID.
- Raw vector ID.
- Raw vector score.
- Sensitive tool metadata.
- System prompt leakage.
- Token budget overflow.

## 10. Contract Test Harness

Command:

```txt
make contract-test
```

Contracts:

- `ChatRequest`.
- `ChatResponse`.
- `ChatEvent`.
- `OcrCleanupRequest`.
- `DocumentIngestRequest`.
- `Session`.
- `ErrorResponse`.
- `EmbeddingVector`.

## 11. Load/Latency Harness

Command:

```txt
make load-test
```

Targets:

- Reasonable p95 first-token latency.
- Backend stable with many concurrent streams.
- Qdrant search latency acceptable.
- No memory leak during long streams.
- Stream cancellation frees resources.

## 12. Security Harness

Command:

```txt
make security-test
```

Checks:

- No DeepSeek key in Android.
- OpenCode not exposed publicly.
- Qdrant not exposed publicly.
- No secrets in logs.
- No vector IDs/raw scores returned to user.
- No system prompt leakage.
- Tool permissions locked down.

## CI placement

Per PR:

- contract tests,
- Rust unit tests,
- SSE replay,
- prompt snapshot,
- secret scan,
- Android unit tests where possible.

Nightly:

- OCR eval,
- RAG eval,
- agent E2E,
- load/latency.
