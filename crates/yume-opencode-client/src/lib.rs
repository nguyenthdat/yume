//! OpenCode HTTP client with mock fallback.
//!
//! Attempts to connect to the private OpenCode server for chat completions
//! via an OpenAI-compatible streaming API. Falls back to a simulated mock
//! response when OpenCode is unreachable — this keeps the system testable
//! without a running OpenCode server.
//!
//! # Example
//!
//! ```no_run
//! use yume_opencode_client::OpenCodeStream;
//! use futures::StreamExt;
//!
//! # async {
//! let mut stream = OpenCodeStream::connect(
//!     "http://localhost:4096".into(),
//!     "opencode".into(),
//!     Some("secret".into()),
//!     "Hello".into(),
//! ).await;
//!
//! while let Some(Ok(chunk)) = stream.next().await {
//!     print!("{chunk}");
//! }
//! # };
//! ```

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;
use futures::StreamExt;
use tokio::time::Duration;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors that can occur when communicating with OpenCode.
#[derive(Debug)]
pub enum OpenCodeError {
    /// An HTTP transport error occurred.
    Http(reqwest::Error),
    /// OpenCode server is unreachable.
    Unavailable,
    /// OpenCode returned a non-2xx status.
    ServerError(String),
}

impl std::fmt::Display for OpenCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {e}"),
            Self::Unavailable => write!(f, "OpenCode server unavailable"),
            Self::ServerError(msg) => write!(f, "OpenCode server error: {msg}"),
        }
    }
}

impl std::error::Error for OpenCodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(e) => Some(e),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Mock stream — emits simulated text word-by-word
// ---------------------------------------------------------------------------

/// A mock stream that emits a simulated assistant response word-by-word.
///
/// Used as a fallback when the real OpenCode server is unreachable.
pub struct MockStream {
    chunks: Vec<String>,
    index: usize,
}

impl MockStream {
    fn new() -> Self {
        let message = "Xin chào! Tôi là Yume assistant (phiên bản mock). \
                       OpenCode server chưa được kết nối. \
                       Vui lòng cấu hình OPENCODE_BASE_URL trong .env \
                       để kết nối với OpenCode server thật. \
                       Bạn có thể hỏi tôi bất cứ điều gì khi hệ thống đã sẵn sàng.";
        let chunks: Vec<String> = message
            .split_whitespace()
            .map(|w| format!("{} ", w))
            .collect();
        Self { chunks, index: 0 }
    }
}

impl Stream for MockStream {
    type Item = Result<String, OpenCodeError>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.index >= self.chunks.len() {
            return Poll::Ready(None);
        }
        let word = self.chunks[self.index].clone();
        self.index += 1;
        Poll::Ready(Some(Ok(word)))
    }
}

// ---------------------------------------------------------------------------
// OpenCodeStream — either real or mock
// ---------------------------------------------------------------------------

type DynStream = Pin<Box<dyn Stream<Item = Result<String, OpenCodeError>> + Send>>;

/// A unified stream that either connects to the real OpenCode server
/// or falls back to a mock response.
pub enum OpenCodeStream {
    /// Stream from the real OpenCode server.
    Real(DynStream),
    /// Mock fallback stream.
    Mock(MockStream),
}

impl OpenCodeStream {
    /// Attempt to connect to OpenCode. Falls back to mock on failure.
    ///
    /// Takes owned values so the returned stream is `'static` and can be
    /// freely moved into spawned tasks.
    pub async fn connect(
        opencode_base_url: String,
        opencode_username: String,
        opencode_password: Option<String>,
        message: String,
    ) -> Self {
        match try_connect_opencode(
            opencode_base_url,
            opencode_username,
            opencode_password,
            message,
        )
        .await
        {
            Ok(stream) => Self::Real(stream),
            Err(_) => {
                tracing::warn!("OpenCode server unreachable — falling back to mock mode");
                Self::Mock(MockStream::new())
            }
        }
    }
}

impl Stream for OpenCodeStream {
    type Item = Result<String, OpenCodeError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut *self {
            Self::Real(stream) => stream.as_mut().poll_next(cx),
            Self::Mock(mock) => Pin::new(mock).poll_next(cx),
        }
    }
}

// ---------------------------------------------------------------------------
// Real OpenCode connection
// ---------------------------------------------------------------------------

async fn try_connect_opencode(
    base_url: String,
    username: String,
    password: Option<String>,
    message: String,
) -> Result<DynStream, OpenCodeError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|_| OpenCodeError::Unavailable)?;

    let url = format!("{base_url}/v1/chat/completions");
    let body = serde_json::json!({
        "model": "deepseek-chat",
        "messages": [
            {"role": "user", "content": message}
        ],
        "stream": true
    });

    let mut request_builder = client.post(&url).json(&body);

    if let Some(ref pw) = password {
        request_builder = request_builder.basic_auth(&username, Some(pw));
    }

    let response = request_builder.send().await.map_err(|e| {
        if e.is_connect() || e.is_timeout() {
            OpenCodeError::Unavailable
        } else {
            OpenCodeError::Http(e)
        }
    })?;

    if !response.status().is_success() {
        return Err(OpenCodeError::ServerError(format!(
            "HTTP {}",
            response.status()
        )));
    }

    let stream = response
        .bytes_stream()
        .then(|chunk_result| async {
            match chunk_result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let mut chunks = Vec::new();

                    for line in text.lines() {
                        if let Some(data) = line.strip_prefix("data: ") {
                            if data == "[DONE]" {
                                break;
                            }
                            if let Ok(json) =
                                serde_json::from_str::<serde_json::Value>(data)
                            {
                                if let Some(content) =
                                    json["choices"][0]["delta"]["content"].as_str()
                                {
                                    if !content.is_empty() {
                                        chunks.push(Ok(content.to_string()));
                                    }
                                }
                            }
                        }
                    }
                    chunks
                }
                Err(e) => vec![Err(OpenCodeError::Http(e))],
            }
        })
        .flat_map(futures::stream::iter);

    Ok(Box::pin(stream))
}
