uniffi::include_scaffolding!("yume");

use std::io::{BufRead, BufReader};
use yume_contracts::schema::CURRENT_SCHEMA_VERSION;

pub trait ChatCallback: Send + Sync + 'static {
    fn on_started(&self, cid: String, mid: String);
    fn on_delta(&self, seq: u64, text: String);
    fn on_done(&self, reason: String);
    fn on_error(&self, code: String, msg: String);
}

#[derive(Debug, thiserror::Error)]
pub enum YumeError {
    #[error("invalid input")]
    InvalidInput,
    #[error("network: {0}")]
    NetworkError(String),
    #[error("serialization: {0}")]
    SerializationError(String),
    #[error("server: {0}")]
    ServerError(String),
}

pub struct ChatMessage {
    pub role: String,
    pub content: String,
}
pub struct TextDelta {
    pub text: String,
}

pub fn current_schema_version() -> Result<String, YumeError> {
    Ok(CURRENT_SCHEMA_VERSION.to_string())
}

pub fn build_chat_request_json(msg: String, cid: String) -> Result<String, YumeError> {
    let r = serde_json::json!({
        "schema_version": CURRENT_SCHEMA_VERSION,
        "conversation_id": cid,
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
        "message": {"role":"user","content":msg},
        "stream":true
    });
    serde_json::to_string(&r).map_err(|e| YumeError::SerializationError(e.to_string()))
}

pub fn send_chat_message(
    url: String,
    msg: String,
    cid: String,
    cb: Box<dyn ChatCallback>,
) -> Result<(), YumeError> {
    if msg.is_empty() {
        return Err(YumeError::InvalidInput);
    }
    let body = build_chat_request_json(msg, cid)?;
    std::thread::spawn(move || {
        if let Err(e) = stream_inner(&url, &body, cb.as_ref()) {
            cb.on_error("NETWORK_ERROR".into(), e.to_string());
        }
    });
    Ok(())
}

fn stream_inner(url: &str, body: &str, cb: &dyn ChatCallback) -> Result<(), YumeError> {
    let ep = format!("{}/v1/chat/stream", url);
    let resp = ureq::post(&ep)
        .header("Content-Type", "application/json")
        .header("Accept", "text/event-stream")
        .send(body.as_bytes())
        .map_err(|e| YumeError::NetworkError(e.to_string()))?;
    let r = BufReader::new(resp.into_body().into_reader());
    let (mut ev, mut dt, mut seq): (String, String, u64) = Default::default();
    for l in r.lines() {
        let l = l.map_err(|e| YumeError::NetworkError(e.to_string()))?;
        if l.is_empty() {
            if !dt.is_empty() {
                dispatch(&ev, &dt, &mut seq, cb);
            }
            ev.clear();
            dt.clear();
        } else if let Some(v) = l.strip_prefix("event: ") {
            ev = v.trim().into();
        } else if let Some(v) = l.strip_prefix("data: ") {
            dt = v.trim().into();
        }
    }
    if !dt.is_empty() {
        dispatch(&ev, &dt, &mut seq, cb);
    }
    Ok(())
}

fn dispatch(ev: &str, dt: &str, seq: &mut u64, cb: &dyn ChatCallback) {
    match ev {
        "chat.started" => {
            if let Ok(j) = serde_json::from_str::<serde_json::Value>(dt) {
                cb.on_started(
                    j["conversation_id"].as_str().unwrap_or("").into(),
                    j["message_id"].as_str().unwrap_or("").into(),
                );
            }
        }
        "message.delta" => {
            if let Ok(j) = serde_json::from_str::<serde_json::Value>(dt) {
                if let Some(t) = j["delta"]["text"].as_str() {
                    *seq += 1;
                    cb.on_delta(*seq, t.into());
                }
            }
        }
        "done" => {
            let r = serde_json::from_str::<serde_json::Value>(dt)
                .ok()
                .and_then(|j| j["finish_reason"].as_str().map(String::from))
                .unwrap_or("stop".into());
            cb.on_done(r);
        }
        "error" => {
            let (c, m) = serde_json::from_str::<serde_json::Value>(dt)
                .ok()
                .map(|j| {
                    (
                        j["code"].as_str().unwrap_or("UNKNOWN").into(),
                        j["message"].as_str().unwrap_or("?").into(),
                    )
                })
                .unwrap_or(("PARSE".into(), dt.into()));
            cb.on_error(c, m);
        }
        _ => {}
    }
}
