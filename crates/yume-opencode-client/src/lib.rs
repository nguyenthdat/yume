//! Multi-provider LLM client for Yume.
//!
//! Supports: DeepSeek, OpenAI (API key + OAuth), OpenCode Server, Mock.
//!
//! # Architecture
//!
//! ```rust
//! use yume_opencode_client::{Provider, ProviderConfig, UnifiedStream};
//!
//! let cfg = ProviderConfig {
//!     provider: Provider::OpenAI,
//!     deepseek_key: None,
//!     openai_key: Some("sk-...".into()),
//!     openai_org: None,
//!     openai_model: Some("gpt-4o-mini".into()),
//!     openai_oauth: None,
//!     opencode_base: None,
//!     opencode_user: None,
//!     opencode_pass: None,
//! };
//!
//! let stream = UnifiedStream::connect(&cfg, "Hello").await;
//! ```

use std::pin::Pin;
use std::task::{Context, Poll};
use futures::stream::Stream;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

// ===========================================================================
// Provider selection
// ===========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    DeepSeek,
    OpenAI,
    OpenCode,
    Mock,
}

impl Provider {
    pub fn from_env() -> Self {
        match std::env::var("LLM_PROVIDER").ok().as_deref() {
            Some("openai") => Self::OpenAI,
            Some("opencode") => Self::OpenCode,
            Some("mock") => Self::Mock,
            _ => Self::DeepSeek, // default
        }
    }
}

// ===========================================================================
// Provider configuration
// ===========================================================================

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub provider: Provider,

    // DeepSeek
    pub deepseek_key: Option<String>,
    pub deepseek_model: Option<String>,

    // OpenAI (API key)
    pub openai_key: Option<String>,
    pub openai_org: Option<String>,
    pub openai_model: Option<String>,

    // OpenAI (OAuth — future)
    pub openai_oauth: Option<OpenAIOAuthConfig>,

    // OpenCode server
    pub opencode_base: Option<String>,
    pub opencode_user: Option<String>,
    pub opencode_pass: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OpenAIOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

impl ProviderConfig {
    pub fn from_env() -> Self {
        Self {
            provider: Provider::from_env(),
            deepseek_key: env_opt("DEEPSEEK_API_KEY"),
            deepseek_model: env_opt("DEEPSEEK_MODEL"),
            openai_key: env_opt("OPENAI_API_KEY"),
            openai_org: env_opt("OPENAI_ORG_ID"),
            openai_model: env_opt("OPENAI_MODEL"),
            openai_oauth: None, // future
            opencode_base: env_opt("OPENCODE_BASE_URL"),
            opencode_user: env_opt("OPENCODE_SERVER_USERNAME"),
            opencode_pass: env_opt("OPENCODE_SERVER_PASSWORD"),
        }
    }
}

fn env_opt(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

// ===========================================================================
// Error
// ===========================================================================

#[derive(Debug)]
pub enum OpenCodeError {
    Http(reqwest::Error),
    Unavailable,
    ServerError(String),
    ApiError { code: String, message: String },
    AuthError(String),
}

impl std::fmt::Display for OpenCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP: {e}"),
            Self::Unavailable => f.write_str("unavailable"),
            Self::ServerError(m) => write!(f, "server: {m}"),
            Self::ApiError { code, message } => write!(f, "API {code}: {message}"),
            Self::AuthError(m) => write!(f, "auth: {m}"),
        }
    }
}

impl std::error::Error for OpenCodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self { Self::Http(e) => Some(e), _ => None }
    }
}

// ===========================================================================
// DTOs
// ===========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageResponse {
    pub info: MessageInfo,
    pub parts: Vec<ResponsePart>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageInfo {
    pub id: String,
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponsePart {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    stream: bool,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    pub healthy: bool,
    pub version: Option<String>,
}

// ===========================================================================
// Unified stream (DeepSeek → OpenAI → OpenCode → Mock)
// ===========================================================================

pub type DynStream = Pin<Box<dyn Stream<Item = Result<String, OpenCodeError>> + Send>>;

pub enum UnifiedStream {
    Real(DynStream),
    Mock(MockStream),
}

impl UnifiedStream {
    pub async fn connect(cfg: &ProviderConfig, message: &str) -> Self {
        match cfg.provider {
            Provider::DeepSeek => {
                if let Some(ref key) = cfg.deepseek_key {
                    if !key.is_empty() {
                        let model = cfg.deepseek_model.as_deref().unwrap_or("deepseek-chat");
                        match try_deepseek(key, model, message).await {
                            Ok(s) => return Self::Real(s),
                            Err(e) => tracing::warn!("DeepSeek: {e}"),
                        }
                    }
                }
            }
            Provider::OpenAI => {
                if let Some(ref key) = cfg.openai_key {
                    if !key.is_empty() {
                        let model = cfg.openai_model.as_deref().unwrap_or("gpt-4o-mini");
                        match try_openai(key, cfg.openai_org.as_deref(), model, message).await {
                            Ok(s) => return Self::Real(s),
                            Err(e) => tracing::warn!("OpenAI: {e}"),
                        }
                    }
                }
                // Try OAuth if no API key
                if let Some(ref oauth) = cfg.openai_oauth {
                    match try_openai_oauth(oauth, message).await {
                        Ok(s) => return Self::Real(s),
                        Err(e) => tracing::warn!("OpenAI OAuth: {e}"),
                    }
                }
            }
            Provider::OpenCode => {
                if let (Some(base), Some(user)) = (&cfg.opencode_base, &cfg.opencode_user) {
                    let client = OpenCodeClient::new(
                        base.clone(), user.clone(), cfg.opencode_pass.clone(),
                    );
                    match client.chat_stream(message).await {
                        Ok(s) => return Self::Real(s),
                        Err(e) => tracing::warn!("OpenCode: {e}"),
                    }
                }
            }
            Provider::Mock => {}
        }
        tracing::warn!("All providers unavailable — mock mode");
        Self::Mock(MockStream::new())
    }
}

impl Stream for UnifiedStream {
    type Item = Result<String, OpenCodeError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut *self {
            Self::Real(s) => s.as_mut().poll_next(cx),
            Self::Mock(m) => Pin::new(m).poll_next(cx),
        }
    }
}

// ===========================================================================
// OpenAI provider
// ===========================================================================

async fn try_openai(
    key: &str, org: Option<&str>, model: &str, msg: &str,
) -> Result<DynStream, OpenCodeError> {
    let mut req = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {key}"));
    if let Some(o) = org { req = req.header("OpenAI-Organization", o); }

    let resp = req.json(&OpenAIChatRequest {
        model: model.into(),
        messages: vec![OpenAIMessage { role: "user".into(), content: msg.into() }],
        stream: true,
    }).send().await.map_err(map_reqwest)?;

    if resp.status().as_u16() == 401 {
        return Err(OpenCodeError::AuthError("Invalid OpenAI API key".into()));
    }
    if !resp.status().is_success() {
        let st = resp.status().as_u16();
        return Err(OpenCodeError::ServerError(format!("OpenAI HTTP {st}")));
    }
    parse_openai_sse(resp).await
}

async fn try_openai_oauth(
    oauth: &OpenAIOAuthConfig, msg: &str,
) -> Result<DynStream, OpenCodeError> {
    // Exchange refresh token for access token
    let client = reqwest::Client::new();
    let token_resp: serde_json::Value = client
        .post("https://api.openai.com/oauth/token")
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", &oauth.client_id),
            ("client_secret", &oauth.client_secret),
            ("refresh_token", &oauth.refresh_token),
        ])
        .send().await.map_err(|e| OpenCodeError::AuthError(e.to_string()))?
        .json().await.map_err(|e| OpenCodeError::Http(e))?;

    let access_token = token_resp["access_token"].as_str()
        .ok_or_else(|| OpenCodeError::AuthError("No access_token in OAuth response".into()))?;

    try_openai(access_token, None, "gpt-4o-mini", msg).await
}

// ===========================================================================
// DeepSeek provider
// ===========================================================================

async fn try_deepseek(
    key: &str, model: &str, msg: &str,
) -> Result<DynStream, OpenCodeError> {
    let resp = reqwest::Client::new()
        .post("https://api.deepseek.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {key}"))
        .json(&serde_json::json!({
            "model": model,
            "messages": [{"role":"user","content":msg}],
            "stream": true
        }))
        .send().await.map_err(map_reqwest)?;

    if !resp.status().is_success() {
        return Err(OpenCodeError::ServerError(format!("DeepSeek HTTP {}", resp.status())));
    }
    parse_openai_sse(resp).await
}

// ===========================================================================
// SSE parser (shared by OpenAI + DeepSeek)
// ===========================================================================

async fn parse_openai_sse(resp: reqwest::Response) -> Result<DynStream, OpenCodeError> {
    let stream = resp.bytes_stream().then(|chunk| async {
        match chunk {
            Ok(b) => {
                let t = String::from_utf8_lossy(&b);
                let mut cs = Vec::new();
                for l in t.lines() {
                    if let Some(d) = l.strip_prefix("data: ") {
                        if d == "[DONE]" { break; }
                        if let Ok(j) = serde_json::from_str::<serde_json::Value>(d) {
                            if let Some(c) = j["choices"][0]["delta"]["content"].as_str() {
                                if !c.is_empty() { cs.push(Ok(c.to_string())); }
                            }
                        }
                    }
                }
                cs
            }
            Err(e) => vec![Err(OpenCodeError::Http(e))],
        }
    }).flat_map(futures::stream::iter);
    Ok(Box::pin(stream))
}

// ===========================================================================
// OpenCode Server client (project-context)
// ===========================================================================

struct OpenCodeClient {
    http: reqwest::Client,
    base_url: String,
    auth_header: Option<String>,
}

impl OpenCodeClient {
    fn new(base: String, user: String, pass: Option<String>) -> Self {
        let auth = pass.map(|pw| {
            format!("Basic {}", base64_encode(&format!("{user}:{pw}")))
        });
        Self { http: reqwest::Client::new(), base_url: base, auth_header: auth }
    }

    async fn chat_stream(&self, message: &str) -> Result<DynStream, OpenCodeError> {
        let sid = self.create_session().await?;
        let resp = self.send_message(&sid, message).await?;
        let texts: Vec<String> = resp.parts.iter().filter_map(|p| p.text.clone()).collect();
        if texts.is_empty() {
            Err(OpenCodeError::ServerError("Empty response".into()))
        } else {
            Ok(Box::pin(futures::stream::iter(texts.into_iter().map(Ok))))
        }
    }

    async fn create_session(&self) -> Result<String, OpenCodeError> {
        let resp = self.req("POST", "/session")
            .json(&serde_json::json!({"title": "Yume chat"}))
            .send().await.map_err(map_reqwest)?;
        let s: Session = resp.json().await.map_err(|e| OpenCodeError::Http(e))?;
        Ok(s.id)
    }

    async fn send_message(&self, sid: &str, msg: &str) -> Result<MessageResponse, OpenCodeError> {
        let resp = self.req("POST", &format!("/session/{sid}/message"))
            .json(&serde_json::json!({
                "parts": [{"type": "text", "text": msg}]
            }))
            .send().await.map_err(map_reqwest)?;
        resp.json().await.map_err(|e| OpenCodeError::Http(e))
    }

    fn req(&self, method: &str, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{path}", self.base_url);
        let mut b = self.http.request(method.parse().unwrap(), &url);
        if let Some(ref h) = self.auth_header { b = b.header("Authorization", h); }
        b
    }
}

// ===========================================================================
// Mock stream
// ===========================================================================

pub struct MockStream {
    chunks: Vec<String>,
    index: usize,
}

impl MockStream {
    pub fn new() -> Self {
        let msg = "Xin chào! Tôi là Yume assistant (phiên bản mock). \
                   Vui lòng cấu hình LLM provider trong .env: \
                   LLM_PROVIDER=deepseek|openai|opencode.";
        let chunks: Vec<String> = msg.split_whitespace().map(|w| format!("{w} ")).collect();
        Self { chunks, index: 0 }
    }
}

impl Stream for MockStream {
    type Item = Result<String, OpenCodeError>;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.index >= self.chunks.len() { return Poll::Ready(None); }
        let w = self.chunks[self.index].clone();
        self.index += 1;
        Poll::Ready(Some(Ok(w)))
    }
}

// ===========================================================================
// Helpers
// ===========================================================================

fn map_reqwest(e: reqwest::Error) -> OpenCodeError {
    if e.is_connect() || e.is_timeout() { OpenCodeError::Unavailable }
    else { OpenCodeError::Http(e) }
}

fn base64_encode(s: &str) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(s.as_bytes())
}
