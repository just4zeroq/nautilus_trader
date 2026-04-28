//! Unified message bus over Redis Streams.
//!
//! # Stream naming
//! `{prefix}:{domain}.{topic}` — default prefix `nt`.
//! Example stream key: `nt:system.heartbeat`
//!
//! # Message envelope (XADD fields)
//! - `source` — sender node ID
//! - `type` — message type name
//! - `ts` — ISO 8601 timestamp
//! - `data` — JSON string payload

use redis::{AsyncCommands, Client};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

pub const DEFAULT_PREFIX: &str = "nt";
pub const DEFAULT_REDIS_URL: &str = "redis://127.0.0.1:6379";

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum BusError {
    Connection(String),
    Publish(String),
    Read(String),
    Subscribe(String),
    Serialize(String),
}

impl std::fmt::Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connection(s) => write!(f, "connection error: {s}"),
            Self::Publish(s) => write!(f, "publish error: {s}"),
            Self::Read(s) => write!(f, "read error: {s}"),
            Self::Subscribe(s) => write!(f, "subscribe error: {s}"),
            Self::Serialize(s) => write!(f, "serialize error: {s}"),
        }
    }
}

impl std::error::Error for BusError {}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

/// A raw message received from the bus.
#[derive(Debug, Clone)]
pub struct BusMessage {
    /// Redis stream entry ID.
    pub id: String,
    /// Full stream key (e.g. `nt:system.heartbeat`).
    pub stream: String,
    /// Raw XADD field-value pairs.
    pub fields: HashMap<String, String>,
}

impl BusMessage {
    /// Read the `data` field and deserialize as JSON.
    pub fn data_as<T: serde::de::DeserializeOwned>(&self) -> Result<T, BusError> {
        let raw = self
            .fields
            .get("data")
            .ok_or_else(|| BusError::Serialize("missing data field".into()))?;
        serde_json::from_str(raw).map_err(|e| BusError::Serialize(e.to_string()))
    }

    /// Get the `source` field value.
    pub fn source(&self) -> Option<&str> {
        self.fields.get("source").map(String::as_str)
    }

    /// Get the `type` field value.
    pub fn msg_type(&self) -> Option<&str> {
        self.fields.get("type").map(String::as_str)
    }

    /// Get the `ts` field value.
    pub fn timestamp(&self) -> Option<&str> {
        self.fields.get("ts").map(String::as_str)
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// A unified message bus client backed by Redis Streams.
///
/// # Example
///
/// ```no_run
/// # async fn example() -> Result<(), message_bus::BusError> {
/// use message_bus::MessageBus;
///
/// let bus = MessageBus::new().await?;
///
/// // Publish a heartbeat
/// bus.publish("system.heartbeat", "trader-rust-1", &serde_json::json!({
///     "node_type": "trader",
///     "status": "online",
/// })).await?;
///
/// // Read new messages
/// let msgs = bus.read("system.heartbeat", "0", 10).await?;
/// for msg in msgs {
///     println!("{}: {:?}", msg.id, msg.fields);
/// }
/// # Ok(())
/// # }
/// ```
pub struct MessageBus {
    client: Client,
    redis_url: String,
    prefix: String,
}

impl MessageBus {
    /// Connect using `REDIS_URL` env var (default `redis://127.0.0.1:6379`)
    /// with prefix `{prefix}`.
    pub async fn new() -> Result<Self, BusError> {
        let url = std::env::var("REDIS_URL").unwrap_or_else(|_| DEFAULT_REDIS_URL.into());
        Self::connect(&url, DEFAULT_PREFIX).await
    }

    /// Connect to a specific Redis URL with a custom stream prefix.
    pub async fn connect(redis_url: &str, prefix: &str) -> Result<Self, BusError> {
        let client =
            Client::open(redis_url).map_err(|e| BusError::Connection(format!("invalid URL: {e}")))?;
        Ok(Self {
            client,
            redis_url: redis_url.to_string(),
            prefix: prefix.to_string(),
        })
    }

    /// Return the stream key for a given topic.
    pub fn stream_key(&self, topic: &str) -> String {
        format!("{}:{}", self.prefix, topic)
    }

    /// Build a full stream key from prefix + topic.
    pub fn build_stream_key(prefix: &str, topic: &str) -> String {
        format!("{prefix}:{topic}")
    }

    // -----------------------------------------------------------------------
    // Publish
    // -----------------------------------------------------------------------

    /// Publish a JSON-serializable payload to a topic.
    ///
    /// Writes to `{prefix}:{topic}` with envelope fields:
    /// `source`, `type`, `ts`, `data`.
    pub async fn publish<T: Serialize>(
        &self,
        topic: &str,
        source: &str,
        payload: &T,
    ) -> Result<String, BusError> {
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| BusError::Connection(e.to_string()))?;

        let stream_key = self.stream_key(topic);
        let ts = chrono::Utc::now().to_rfc3339();
        let data =
            serde_json::to_string(payload).map_err(|e| BusError::Serialize(e.to_string()))?;

        let id: String = con
            .xadd(
                &stream_key,
                "*",
                &[
                    ("source", source),
                    ("type", "message"),
                    ("ts", &ts),
                    ("data", &data),
                ],
            )
            .await
            .map_err(|e| BusError::Publish(e.to_string()))?;

        tracing::trace!("Published to {stream_key} [id={id}]");
        Ok(id)
    }

    // -----------------------------------------------------------------------
    // Read (one-shot poll)
    // -----------------------------------------------------------------------

    /// Read messages from a topic stream since `last_id`.
    ///
    /// - Use `"0"` to get all messages
    /// - Use `"$"` for messages after now (blocking)
    /// - Use last received message ID for incremental reads
    pub async fn read(
        &self,
        topic: &str,
        last_id: &str,
        count: usize,
    ) -> Result<Vec<BusMessage>, BusError> {
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| BusError::Connection(e.to_string()))?;

        let stream_key = self.stream_key(topic);

        let result: Vec<(String, Vec<(String, HashMap<String, String>)>)> = redis::cmd("XREAD")
            .arg("COUNT")
            .arg(count)
            .arg("STREAMS")
            .arg(&stream_key)
            .arg(last_id)
            .query_async(&mut con)
            .await
            .map_err(|e| BusError::Read(e.to_string()))?;

        let mut messages = Vec::new();
        for (_stream, entries) in result {
            for (id, fields) in entries {
                messages.push(BusMessage {
                    id,
                    stream: stream_key.clone(),
                    fields,
                });
            }
        }

        Ok(messages)
    }

    /// Returns a `MessageStream` that polls for new messages at `interval`.
    ///
    /// Each call to `stream.next().await` returns any new messages since last poll.
    pub fn subscribe(&self, topic: &str, interval: Duration) -> MessageStream {
        MessageStream::new(
            self.prefix.clone(),
            topic.to_string(),
            self.redis_url.clone(),
            interval,
        )
    }
}

// ---------------------------------------------------------------------------
// Subscription stream (long-poll)
// ---------------------------------------------------------------------------

/// An async stream of bus messages for a topic.
///
/// Call `next().await` in a loop to receive messages as they arrive.
/// Uses long-poll XREAD internally.
pub struct MessageStream {
    client: Client,
    topic: String,
    prefix: String,
    last_id: String,
    interval: Duration,
}

impl MessageStream {
    fn new(prefix: String, topic: String, redis_url: String, interval: Duration) -> Self {
        let client = Client::open(redis_url.as_str())
            .expect("Failed to create Redis client for subscription");
        Self {
            client,
            topic,
            prefix,
            last_id: "0".to_string(),
            interval,
        }
    }

    /// Set the starting stream ID (default `"0"` = all messages).
    pub fn start_from(mut self, last_id: &str) -> Self {
        self.last_id = last_id.to_string();
        self
    }

    /// Block until the next batch of messages arrives and return them.
    ///
    /// Returns `Ok(vec![])` on timeout (no new messages within polling interval).
    pub async fn next(&mut self) -> Result<Vec<BusMessage>, BusError> {
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| BusError::Connection(e.to_string()))?;

        let stream_key = format!("{}:{}", self.prefix, self.topic);

        let result: Vec<(String, Vec<(String, HashMap<String, String>)>)> = redis::cmd("XREAD")
            .arg("COUNT")
            .arg(10)
            .arg("BLOCK")
            .arg(self.interval.as_millis() as u64)
            .arg("STREAMS")
            .arg(&stream_key)
            .arg(&self.last_id)
            .query_async(&mut con)
            .await
            .map_err(|e| BusError::Read(e.to_string()))?;

        let mut messages = Vec::new();
        for (_stream, entries) in result {
            for (id, fields) in entries {
                self.last_id.clone_from(&id);
                messages.push(BusMessage {
                    id,
                    stream: stream_key.clone(),
                    fields,
                });
            }
        }

        Ok(messages)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_key_format() {
        assert_eq!(MessageBus::build_stream_key("nt", "system.heartbeat"), "nt:system.heartbeat");
        assert_eq!(MessageBus::build_stream_key("collector", "binance.quotes"), "collector:binance.quotes");
    }

    #[test]
    fn test_bus_message_accessors() {
        let mut fields = HashMap::new();
        fields.insert("source".into(), "test-node".into());
        fields.insert("type".into(), "test".into());
        fields.insert("ts".into(), "2026-01-01T00:00:00Z".into());
        fields.insert("data".into(), r#"{"key":"value"}"#.into());

        let msg = BusMessage {
            id: "0-0".into(),
            stream: "nt:test".into(),
            fields,
        };

        assert_eq!(msg.source(), Some("test-node"));
        assert_eq!(msg.msg_type(), Some("test"));
        assert_eq!(msg.timestamp(), Some("2026-01-01T00:00:00Z"));

        let data: serde_json::Value = msg.data_as().unwrap();
        assert_eq!(data["key"], "value");
    }
}
