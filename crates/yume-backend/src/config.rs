//! Backend configuration loaded from environment.

use yume_opencode_client::ProviderConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub env: String,
    pub retention_days: u32,
    pub log_prompts: bool,
    pub log_prompts_redact: bool,
    pub public_base_url: String,
    pub jwt_issuer: String,
    pub google_android_client_id: Option<String>,
    pub qdrant_url: String,
    pub database_url: Option<String>,
    pub keydb_url: Option<String>,
    /// Multi-provider LLM configuration
    pub provider: ProviderConfig,
}

impl Config {
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
            qdrant_url: env_default("QDRANT_URL", "http://localhost:6334"),
            database_url: env_optional("DATABASE_URL"),
            keydb_url: env_optional("KEYDB_URL"),
            provider: ProviderConfig::from_env(),
        }
    }
}

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
