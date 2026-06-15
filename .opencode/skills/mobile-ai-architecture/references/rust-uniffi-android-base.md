# Rust UniFFI Android Base Template

Use this as a starting point when the harness is asked to create or review base code. Verify current UniFFI and Android Gradle versions before committing generated code.

## Suggested Layout

```text
rust/
  yume-core/
    Cargo.toml
    build.rs
    src/
      lib.rs
      yume_core.udl
android/
  app/
    build.gradle.kts
    src/main/java/com/example/yume/core/YumeCore.kt
    src/main/jniLibs/
```

## Cargo.toml

```toml
[package]
name = "yume-core"
version = "0.1.0"
edition = "2021"

[lib]
name = "yume_core"
crate-type = ["cdylib", "staticlib", "rlib"]

[dependencies]
thiserror = "1"
uniffi = "0.29"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[build-dependencies]
uniffi = { version = "0.29", features = ["build"] }
```

## build.rs

```rust
fn main() {
    uniffi::generate_scaffolding("src/yume_core.udl").expect("generate UniFFI scaffolding");
}
```

## src/yume_core.udl

```webidl
namespace yume_core {
  dictionary OcrTextBlock {
    string text;
    double confidence;
    string page_id;
  };

  dictionary OcrDocument {
    string id;
    string language;
    sequence<OcrTextBlock> blocks;
    string source;
  };

  dictionary IndexResult {
    string document_id;
    u32 chunk_count;
    sequence<string> warnings;
  };

  dictionary Citation {
    string document_id;
    string page_id;
    string snippet;
  };

  dictionary ChatAnswer {
    string answer;
    sequence<Citation> citations;
    sequence<string> warnings;
  };

  [Error]
  enum YumeError {
    "InvalidInput",
    "IndexUnavailable",
    "ModelUnavailable",
    "Internal"
  };

  IndexResult index_ocr_document(OcrDocument document) throws YumeError;
  ChatAnswer answer_question(string conversation_id, string question) throws YumeError;
  string normalize_ocr_text(string text);
};
```

## src/lib.rs

```rust
uniffi::include_scaffolding!("yume_core");

#[derive(Debug, thiserror::Error)]
pub enum YumeError {
    #[error("invalid input")]
    InvalidInput,
    #[error("index unavailable")]
    IndexUnavailable,
    #[error("model unavailable")]
    ModelUnavailable,
    #[error("internal error")]
    Internal,
}

pub struct OcrTextBlock {
    pub text: String,
    pub confidence: f64,
    pub page_id: String,
}

pub struct OcrDocument {
    pub id: String,
    pub language: String,
    pub blocks: Vec<OcrTextBlock>,
    pub source: String,
}

pub struct IndexResult {
    pub document_id: String,
    pub chunk_count: u32,
    pub warnings: Vec<String>,
}

pub struct Citation {
    pub document_id: String,
    pub page_id: String,
    pub snippet: String,
}

pub struct ChatAnswer {
    pub answer: String,
    pub citations: Vec<Citation>,
    pub warnings: Vec<String>,
}

pub fn normalize_ocr_text(text: String) -> String {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn index_ocr_document(document: OcrDocument) -> Result<IndexResult, YumeError> {
    if document.id.trim().is_empty() {
        return Err(YumeError::InvalidInput);
    }

    let normalized_blocks = document
        .blocks
        .iter()
        .filter(|block| !normalize_ocr_text(block.text.clone()).is_empty())
        .count();

    Ok(IndexResult {
        document_id: document.id,
        chunk_count: normalized_blocks as u32,
        warnings: Vec::new(),
    })
}

pub fn answer_question(conversation_id: String, question: String) -> Result<ChatAnswer, YumeError> {
    if conversation_id.trim().is_empty() || question.trim().is_empty() {
        return Err(YumeError::InvalidInput);
    }

    Ok(ChatAnswer {
        answer: "The retrieval and model adapter are not implemented yet.".to_string(),
        citations: Vec::new(),
        warnings: vec!["stub-answer".to_string()],
    })
}
```

## Android Build Notes

Use `cargo-ndk` or the team's chosen Gradle integration to build Android ABIs into `android/app/src/main/jniLibs/`.

```bash
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o android/app/src/main/jniLibs build --release
uniffi-bindgen generate rust/yume-core/src/yume_core.udl --language kotlin --out-dir android/app/src/main/java
```

Verify these commands against the pinned UniFFI version. UniFFI command-line syntax can change between releases.

## Kotlin Facade

```kotlin
package com.example.yume.core

class YumeCoreFacade {
    fun normalize(text: String): String = normalizeOcrText(text)
}
```

Keep generated UniFFI Kotlin files separate from handwritten facades so regeneration does not delete app code.
