//! Backend configuration loaded from environment variables.
//!
//! Every value has a safe default for local development.
//! Secrets must never be hard-coded here.

/// Application configuration, populated from environment.
///
/// Fields are intentionally forward-looking — they will be consumed
/// in later swings as auth, Qdrant, and OpenCode are integrated.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    /// IP:port the backend listens on.
    pub listen_addr: String,

    /// `YUME_ENV` — development, staging, or production.
    pub env: String,

    /// Data retention period in days.
    pub retention_days: u32,

    /// Whether prompt logging is enabled.
    pub log_prompts: bool,

    /// Whether logged prompts should be redacted.
    pub log_prompts_redact: bool,

    /// Public base URL for CORS and OAuth redirects.
    pub public_base_url: String,

    /// JWT token issuer name.
    pub jwt_issuer: String,

    /// Google OAuth Android client ID.
    pub google_android_client_id: Option<String>,

    /// DeepSeek API key (never exposed to Android).
    pub deepseek_api_key: Option<String>,

    /// Private OpenCode server URL.
    pub opencode_base_url: String,

    /// OpenCode basic-auth username.
    pub opencode_username: String,

    /// OpenCode basic-auth password (secret).
    pub opencode_password: Option<String>,

    /// Qdrant gRPC/HTTP URL.
    pub qdrant_url: String,

    /// Postgres connection string.
    pub database_url: Option<String>,

    /// KeyDB connection string (Redis-compatible, faster).
    pub keydb_url: Option<String>,
}

impl Config {
    /// Load configuration from environment variables, falling back to safe defaults.
    ///
    /// Call `dotenvy::dotenv().ok()` before this if you want `.env` support.
    pub fn from_env() -> Self {
        Self {
            listen_addr: env_default("YUME_LISTEN_ADDR", "0.0.0.0:3000"),
            env: env_default("YUME_ENV", "development"),
            retention_days: env_parse("YUME_RETENTION_DAYS", 30),
            log_prompts: env_parse("YUME_LOG_PROMPTS", true),
            log_prompts_redact: env_parse("YUME_LOG_PROMPTS_REDACT", true),
            public_base_url: env_default("YUME_PUBLIC_BASE_URL", "http://localhost:3000"),
            jwt_issuer: env_default("YUME_JWT_ISSUER", "yume"),
            google_android_client_id: env_optional("YUME_GOOGLE_ANDROID_CLIENT_ID"),
            deepseek_api_key: env_optional("DEEPSEEK_API_KEY"),
            opencode_base_url: env_default("OPENCODE_BASE_URL", "http://localhost:4096"),
            opencode_username: env_default("OPENCODE_SERVER_USERNAME", "opencode"),
            opencode_password: env_optional("OPENCODE_SERVER_PASSWORD"),
            qdrant_url: env_default("QDRANT_URL", "http://localhost:6334"),
            database_url: env_optional("DATABASE_URL"),
            keydb_url: env_optional("KEYDB_URL"),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn env_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_optional(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

fn env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
