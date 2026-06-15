//! OpenAI OAuth endpoints.
//!
//! Flow: authorize → user approves → callback → store tokens → refresh

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::AppState;

// ---------------------------------------------------------------------------
// In-memory token store
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct TokenStore {
    inner: Arc<RwLock<Option<OpenAITokens>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAITokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<i64>,
    pub openai_org: Option<String>,
}

impl TokenStore {
    pub fn new() -> Self { Self::default() }
    pub async fn get(&self) -> Option<OpenAITokens> { self.inner.read().await.clone() }
    pub async fn set(&self, tokens: OpenAITokens) { *self.inner.write().await = Some(tokens); }
}

// ---------------------------------------------------------------------------
// OAuthState
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct OAuthState {
    pub config: Arc<crate::config::Config>,
    pub tokens: TokenStore,
}

// ---------------------------------------------------------------------------
// DTOs
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct AuthorizeResponse { pub url: String, pub state: String }

#[derive(Deserialize)]
pub struct CallbackQuery { pub code: Option<String>, pub state: Option<String>, pub error: Option<String> }

#[derive(Deserialize)]
pub struct RefreshBody { pub refresh_token: Option<String> }

#[derive(Serialize)]
pub struct TokenResponse { pub access_token: String, pub refresh_token: String, pub expires_in: Option<i64>, pub token_type: String }

fn oauth_client_id() -> Result<String, (StatusCode, String)> {
    std::env::var("OPENAI_OAUTH_CLIENT_ID").ok().filter(|v| !v.is_empty())
        .ok_or((StatusCode::BAD_REQUEST, "OPENAI_OAUTH_CLIENT_ID not configured".into()))
}

fn oauth_client_secret() -> Result<String, (StatusCode, String)> {
    std::env::var("OPENAI_OAUTH_CLIENT_SECRET").ok().filter(|v| !v.is_empty())
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "OPENAI_OAUTH_CLIENT_SECRET not set".into()))
}

// ---------------------------------------------------------------------------
// POST /v1/auth/openai/authorize
// ---------------------------------------------------------------------------

pub async fn authorize(
    State(app): State<AppState>,
) -> Result<Json<AuthorizeResponse>, (StatusCode, String)> {
    let client_id = oauth_client_id()?;
    let redirect_uri = format!("{}/v1/auth/openai/callback", app.oauth.config.public_base_url);
    let csrf_state = uuid::Uuid::new_v4().to_string();
    let url = format!(
        "https://api.openai.com/v1/oauth/authorize?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code&scope=openid+offline_access+model.read&state={csrf_state}"
    );
    Ok(Json(AuthorizeResponse { url, state: csrf_state }))
}

// ---------------------------------------------------------------------------
// GET /v1/auth/openai/callback
// ---------------------------------------------------------------------------

pub async fn callback(
    State(app): State<AppState>,
    Query(query): Query<CallbackQuery>,
) -> Result<Json<TokenResponse>, (StatusCode, String)> {
    if let Some(error) = query.error { return Err((StatusCode::BAD_REQUEST, format!("OAuth error: {error}"))); }
    let code = query.code.ok_or((StatusCode::BAD_REQUEST, "Missing code".into()))?;
    let cid = oauth_client_id()?;
    let secret = oauth_client_secret()?;
    let redirect_uri = format!("{}/v1/auth/openai/callback", app.oauth.config.public_base_url);

    let http = reqwest::Client::new();
    let resp = http.post("https://api.openai.com/oauth/token")
        .form(&[("grant_type","authorization_code"),("client_id",&*cid),("client_secret",&secret),("code",&code),("redirect_uri",&redirect_uri)])
        .send().await.map_err(|e| (StatusCode::BAD_GATEWAY, format!("Token exchange failed: {e}")))?;

    let json: serde_json::Value = resp.json().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Invalid response: {e}")))?;

    let access = json["access_token"].as_str().unwrap_or("").to_string();
    let refresh = json["refresh_token"].as_str().unwrap_or("").to_string();
    let expires = json["expires_in"].as_i64();

    app.oauth.tokens.set(OpenAITokens {
        access_token: access.clone(),
        refresh_token: refresh.clone(),
        expires_at: expires.map(|e| chrono::Utc::now().timestamp() + e),
        openai_org: None,
    }).await;

    Ok(Json(TokenResponse { access_token: access, refresh_token: refresh, expires_in: expires, token_type: "Bearer".into() }))
}

// ---------------------------------------------------------------------------
// POST /v1/auth/openai/refresh
// ---------------------------------------------------------------------------

pub async fn refresh(
    State(app): State<AppState>,
    Json(body): Json<RefreshBody>,
) -> Result<Json<TokenResponse>, (StatusCode, String)> {
    let stored = app.oauth.tokens.get().await
        .ok_or((StatusCode::UNAUTHORIZED, "No stored tokens".into()))?;
    let refresh_token = body.refresh_token.unwrap_or(stored.refresh_token);
    let cid = oauth_client_id().unwrap_or_default();
    let secret = oauth_client_secret().unwrap_or_default();

    let http = reqwest::Client::new();
    let resp = http.post("https://api.openai.com/oauth/token")
        .form(&[("grant_type","refresh_token"),("client_id",&*cid),("client_secret",&secret),("refresh_token",&refresh_token)])
        .send().await.map_err(|e| (StatusCode::BAD_GATEWAY, format!("Refresh failed: {e}")))?;

    if !resp.status().is_success() {
        let err = resp.text().await.unwrap_or_default();
        return Err((StatusCode::UNAUTHORIZED, format!("Refresh rejected: {err}")));
    }

    let json: serde_json::Value = resp.json().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Invalid response: {e}")))?;

    let access = json["access_token"].as_str().unwrap_or("").to_string();
    let new_refresh = json["refresh_token"].as_str().unwrap_or("").to_string();
    let expires = json["expires_in"].as_i64();

    app.oauth.tokens.set(OpenAITokens {
        access_token: access.clone(),
        refresh_token: if new_refresh.is_empty() { refresh_token } else { new_refresh },
        expires_at: expires.map(|e| chrono::Utc::now().timestamp() + e),
        openai_org: None,
    }).await;

    Ok(Json(TokenResponse { access_token: access, refresh_token: String::new(), expires_in: expires, token_type: "Bearer".into() }))
}
