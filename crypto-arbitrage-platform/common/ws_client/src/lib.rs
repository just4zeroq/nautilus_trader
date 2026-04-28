use std::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};

/// WebSocket client for node registration, heartbeat, and command handling.
///
/// Connects to the web_backend's `/ws/node` endpoint, sends heartbeats every 30s,
/// and responds to incoming commands.
pub struct NodeWsClient {
    node_id: String,
    node_type: String,
    backend_url: String,
    token: String,
}

impl NodeWsClient {
    pub fn new(node_id: &str, node_type: &str, backend_url: &str, token: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            node_type: node_type.to_string(),
            backend_url: backend_url.to_string(),
            token: token.to_string(),
        }
    }

    /// Register this node with the backend via REST POST (best-effort).
    pub async fn register(&self, name: &str, exchange: Option<&str>) -> anyhow::Result<()> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;

        let payload = serde_json::json!({
            "node_id": self.node_id,
            "name": name,
            "node_type": self.node_type,
            "exchange": exchange,
            "version": env!("CARGO_PKG_VERSION"),
        });

        let url = format!("{}/api/v1/nodes/register", self.backend_url);
        let resp = client.post(&url).json(&payload).send().await?;

        if resp.status().is_success() {
            tracing::info!("Node registered with backend (node_id={})", self.node_id);
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            tracing::warn!("Registration returned {}: {}", status, body);
        }

        Ok(())
    }

    fn ws_url(&self) -> String {
        let ws_protocol = if self.backend_url.starts_with("https://") {
            "wss://"
        } else {
            "ws://"
        };
        let host = self
            .backend_url
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        format!("{}{}/ws/node?token={}", ws_protocol, host, self.token)
    }

    /// Run the WebSocket client loop. Returns on disconnect/error; callers should retry.
    pub async fn run(&self) -> anyhow::Result<()> {
        let url = self.ws_url();
        tracing::info!("Connecting to WebSocket: {}", url);

        let (ws_stream, _) = connect_async(&url).await?;
        let (mut write, mut read) = ws_stream.split();

        let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(30));
        heartbeat_interval.tick().await;

        loop {
            tokio::select! {
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            tracing::debug!("WS message received: {}", text);
                            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                                if let Some("command") = value.get("type").and_then(|v| v.as_str()) {
                                    let command_id = value
                                        .get("command_id")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown");
                                    tracing::info!(
                                        "Command received: command_id={}, data={:?}",
                                        command_id,
                                        value.get("data")
                                    );

                                    let response = serde_json::json!({
                                        "type": "command_response",
                                        "ts": chrono::Utc::now().to_rfc3339(),
                                        "data": {
                                            "command_id": command_id,
                                            "status": "ok",
                                            "message": "command received",
                                        },
                                    });
                                    if let Err(e) = write.send(Message::Text(response.to_string())).await {
                                        tracing::error!("Failed to send command response: {}", e);
                                        break;
                                    }
                                }
                            }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            let _ = write.send(Message::Pong(data)).await;
                        }
                        Some(Ok(Message::Close(frame))) => {
                            tracing::warn!("WebSocket closed by server: {:?}", frame);
                            break;
                        }
                        Some(Err(e)) => {
                            tracing::error!("WebSocket error: {}", e);
                            break;
                        }
                        None => {
                            tracing::warn!("WebSocket stream ended");
                            break;
                        }
                        _ => {}
                    }
                }
                _ = heartbeat_interval.tick() => {
                    let heartbeat = serde_json::json!({
                        "type": "heartbeat",
                        "ts": chrono::Utc::now().to_rfc3339(),
                        "data": {
                            "node_id": self.node_id,
                            "node_type": self.node_type,
                            "status": "online",
                        },
                    });
                    if let Err(e) = write.send(Message::Text(heartbeat.to_string())).await {
                        tracing::error!("Failed to send heartbeat: {}", e);
                        break;
                    }
                    tracing::debug!("Heartbeat sent");
                }
            }
        }

        Ok(())
    }
}
