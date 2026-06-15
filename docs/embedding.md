# On-device Embedding Design

## Decision

Yume should run embeddings on Android to satisfy cost, privacy, and offline requirements.

Backend remains the retrieval/control plane, but it does not call an embedding API for MVP.

## Model recommendation

Priority order:

1. Prototype: MediaPipe Text Embedder + Universal Sentence Encoder TFLite.
2. Production candidate: `intfloat/multilingual-e5-small`, quantized int8 and converted to ONNX/TFLite.
3. Fallback: `sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2`.
4. Avoid for MVP: `BAAI/bge-m3`, because it is too heavy for mobile despite high quality.

## Comparison

| Model | Cost | Android fit | Vietnamese/English | Dim | Recommendation |
|---|---:|---|---|---:|---|
| MediaPipe USE TFLite | $0 | Very easy | Medium | Usually 512 | Fast prototype |
| `multilingual-e5-small` | $0 | Needs conversion/quantization | Good | 384 | Preferred target |
| `paraphrase-multilingual-MiniLM-L12-v2` | $0 | Needs conversion/quantization | Good enough | 384 | Fallback |
| `bge-m3` | $0 | Heavy | Very good | 1024 | Not MVP |

## Recommended target

Use `multilingual_e5_small_int8_v1` as the long-term model ID.

Initial MVP can ship with MediaPipe USE if E5 conversion is not ready. If starting with USE, keep Qdrant collections versioned so migration to E5 is clean.

## Embedding config

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

## Query flow

```txt
User message
→ Android applies query prefix
→ Android computes query embedding offline
→ Rust Core sends ChatRequest with query_embedding
→ Backend validates vector
→ Backend searches Qdrant
→ Backend builds sanitized context
→ OpenCode/DeepSeek answer
```

## Document ingest flow

```txt
OCR/document text
→ Rust Core normalizes and chunks
→ Android applies passage prefix
→ Android computes chunk embeddings offline
→ Rust Core sends chunks + vectors
→ Backend validates vectors
→ Qdrant upsert
```

## Memory writeback challenge

Backend-created memory summaries need vectors, but embedding is Android-only.

Solution:

```txt
Backend creates pending embedding job
→ Android claims job
→ Android embeds summary locally
→ Android uploads vector
→ Backend upserts memory into Qdrant
```

## Vector validation

Backend must reject:

- wrong model ID,
- wrong dimension,
- NaN/Inf,
- extreme norm,
- oversized vector payload,
- model versions not compatible with collection.

## Qdrant versioning

If model changes, prefer either:

- new collection version, e.g. `ocr_chunks_v2`, or
- named vectors, e.g. `text_e5_v1`, `text_use_v1`.

For MVP simplicity, use versioned collections.

## Benchmark requirements

Before production:

- Android 15/16 real device latency.
- Battery impact.
- APK size impact.
- Vietnamese/English retrieval hit@5/hit@10.
- OCR-to-RAG quality.
- Memory pressure under batch embedding.
