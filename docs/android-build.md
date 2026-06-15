# Android + Rust UniFFI Design

## Android scope

Android is responsible for UI and Android platform integration only.

Stack:

- Kotlin.
- Jetpack Compose.
- CameraX or Android Photo Picker.
- ML Kit Text Recognition v2.
- MediaPipe/TFLite text embedding runtime.
- UniFFI/JNI bridge to Rust.
- Light local cache for UI/session state.

Target:

- Latest two Android versions only: Android 15/16.
- This simplifies permission/runtime support but reduces device coverage.

## Android package layout

```txt
com.yume/
  MainActivity.kt

  ui/chat/
    ChatScreen.kt
    ChatViewModel.kt
    MessageBubble.kt
    StreamingMessageRenderer.kt

  ui/conversations/
    ConversationListScreen.kt

  ui/ocr/
    OcrCaptureScreen.kt
    OcrPreviewScreen.kt
    OcrEditScreen.kt

  ui/settings/
    SettingsScreen.kt
    PrivacySettingsScreen.kt

  platform/camera/
    CameraXController.kt

  platform/photo/
    PhotoPickerAdapter.kt

  platform/ocr/
    MlKitTextRecognizer.kt
    OcrResultMapper.kt

  platform/embedding/
    AndroidEmbeddingProvider.kt
    MediaPipeTextEmbedder.kt
    TfliteTextEmbedder.kt

  rust/
    YumeRustClient.kt
    YumeStreamCallback.kt
    YumeErrorMapper.kt

  data/
    ChatRepository.kt
    SessionRepository.kt
```

## OCR pipeline

1. User captures image with CameraX or imports through Photo Picker.
2. Android prepares image orientation/resolution.
3. ML Kit Text Recognition v2 runs locally.
4. Kotlin maps ML Kit blocks/lines/elements into Yume OCR DTOs.
5. User reviews and edits in OCR preview screen.
6. Kotlin sends confirmed text and metadata to Rust Core.
7. Rust normalizes text for chat/RAG.
8. User can send OCR text to chat or ingest as a document.

Recommended MVP OCR dependency:

- Bundled ML Kit Latin model for immediate offline availability.
- Vietnamese/English first.
- Add other language packs later.

Captured OCR metadata:

- Source URI/hash.
- Page index.
- Block/line positions when available.
- Confidence.
- Language hints.
- User-corrected flag.
- Timestamp.

## On-device embedding pipeline

Kotlin owns the platform runtime. Rust owns chunking, request building, and validation flow.

Recommended UniFFI foreign trait shape:

```rust
trait AndroidEmbeddingProvider {
  async fn embed_texts(
    texts: Vec<String>,
    model_id: String
  ) -> Result<Vec<EmbeddingVector>, YumeError>;
}
```

Kotlin implementation options:

- `MediaPipeTextEmbedder` for fast prototype.
- `TfliteTextEmbedder` for quantized multilingual E5 production candidate.

## Rust UniFFI boundary

Export coarse APIs, not many tiny calls.

Suggested exported object:

```txt
YumeClient
  create_session()
  build_chat_request()
  start_chat_stream(request, callback) -> StreamHandle
  cancel_stream(stream_id)
  normalize_ocr_text(input) -> NormalizedOcrText
  parse_stream_event(raw) -> ChatEvent
  prepare_document_ingest(ocr_text, metadata) -> DocumentIngestRequest
```

Exported records/enums should include:

- `ChatRequest`.
- `ChatResponse`.
- `ChatEvent`.
- `OcrCleanupRequest`.
- `DocumentIngestRequest`.
- `Session`.
- `ErrorResponse`.
- `EmbeddingVector`.
- `EmbeddingConfig`.

## Streaming UX and lifecycle

- UI renders token deltas incrementally.
- Stream cancellation maps to Rust `StreamHandle.cancel()`.
- Backgrounding should either keep stream if app remains active or cancel cleanly.
- Reconnection is not automatic unless idempotency key and server replay support are added.
- Every stream event has monotonically increasing `seq` where possible.

## Android FFI smoke harness

Cases:

- Load native library.
- Create Rust client.
- Build chat request.
- Normalize OCR text.
- Parse normal stream event.
- Parse malformed stream event.
- Map Rust error to Kotlin error.
- Start and cancel mock stream.
- Call Kotlin embedding provider through UniFFI trait.

Command target:

```txt
make android-ffi-smoke
```

## Risks

- UniFFI async/callback lifetimes can leak if objects are not destroyed.
- Streaming callback must be lifecycle-aware.
- Large OCR payloads across FFI can be expensive.
- Model assets increase APK size.
- TFLite conversion for multilingual E5 needs benchmarking before production.
