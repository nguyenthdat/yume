//! SSE chat event stream types.
//!
//! The backend emits a stream of [`ChatEvent`] values over `text/event-stream`.
//! Every event carries a `seq` number where applicable so clients can detect gaps.

use serde::{Deserialize, Serialize};

/// A single SSE chat event emitted by `POST /v1/chat/stream`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum ChatEvent {
    /// Stream has started. Contains conversation and message identifiers.
    #[serde(rename = "chat.started")]
    ChatStarted {
        conversation_id: String,
        message_id: String,
    },

    /// A token-level text delta from the model.
    #[serde(rename = "message.delta")]
    MessageDelta { seq: u64, delta: TextDelta },

    /// A citation referencing a retrieved document or chunk.
    #[serde(rename = "citation")]
    Citation { seq: u64, citation: CitationData },

    /// Token usage for the current response.
    #[serde(rename = "usage")]
    Usage {
        input_tokens: u64,
        output_tokens: u64,
    },

    /// Stream completed normally.
    #[serde(rename = "done")]
    Done { finish_reason: String },

    /// Stream terminated with an error.
    #[serde(rename = "error")]
    Error {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_after_ms: Option<u64>,
        recoverable: bool,
    },
}

/// A single token or text delta from the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextDelta {
    pub text: String,
}

/// Metadata for a citation referencing a source document or chunk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CitationData {
    pub source: String,
    pub title: String,
    pub display_ref: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize_message_delta() {
        let event = ChatEvent::MessageDelta {
            seq: 1,
            delta: TextDelta {
                text: "Xin chào".into(),
            },
        };

        let json = serde_json::to_string(&event).unwrap();
        let parsed: ChatEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event, parsed);

        // Verify exact JSON shape matches API contract
        let expected = r#"{"event":"message.delta","seq":1,"delta":{"text":"Xin chào"}}"#;
        assert_eq!(json, expected);
    }
}
