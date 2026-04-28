use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::RwLock;
use salvo::prelude::*;
use salvo::websocket::{WebSocketUpgrade, Message};
use serde::{Deserialize, Serialize};

pub type TokenMap = Arc<RwLock<HashMap<String, String>>>;

static TOKEN_MAP: OnceLock<TokenMap> = OnceLock::new();

pub fn init_token_map() -> TokenMap {
    let map = Arc::new(RwLock::new(HashMap::new()));
    TOKEN_MAP.set(map.clone()).expect("Token map already initialized");
    map
}

pub fn get_token_map() -> &'static TokenMap {
    TOKEN_MAP.get().expect("Token map not initialized")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeWsMessage {
    #[serde(rename = "heartbeat")]
    Heartbeat { timestamp: String },
    #[serde(rename = "heartbeat_ack")]
    HeartbeatAck { timestamp: String },
    #[serde(rename = "status")]
    Status { status: String, message: String },
    #[serde(rename = "log")]
    Log { level: String, message: String, timestamp: String },
    #[serde(rename = "command_response")]
    CommandResponse { command_id: String, status: String, data: serde_json::Value },
    #[serde(rename = "command")]
    Command { id: String, action: String, params: serde_json::Value },
}

#[handler]
pub async fn handle_node_ws(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    let token = req.query::<String>("token").unwrap_or_default();
    if token.is_empty() {
        return Err(StatusError::unauthorized().brief("Missing token"));
    }

    // Validate token
    let token_map = get_token_map();
    let node_id = {
        let map = token_map.read().await;
        map.get(&token).cloned()
    };

    let node_id = match node_id {
        Some(id) => id,
        None => {
            return Err(StatusError::unauthorized().brief("Invalid token"));
        }
    };

    WebSocketUpgrade::new()
        .upgrade(req, res, |mut ws| async move {
            // Mark node online
            tracing::info!("Node {} connected via WebSocket", node_id);
            {
                let node_state = crate::handlers::node::get_state();
                let mut nodes = node_state.write().await;
                if let Some(node) = nodes.get_mut(&node_id) {
                    node.status = "online".to_string();
                    node.ws_connected = true;
                }
            }

            // Send initial connection ack
            let ack = serde_json::json!({
                "type": "connected",
                "node_id": node_id,
                "message": "WebSocket connection established"
            });
            let _ = ws.send(Message::text(ack.to_string())).await;

            // Process incoming messages
            loop {
                match ws.recv().await {
                    Some(Ok(msg)) => {
                        if msg.is_close() {
                            tracing::info!("Node {} sent close frame", node_id);
                            break;
                        }

                        if msg.is_text() {
                            let text = match msg.as_str() {
                                Ok(s) => s.to_string(),
                                Err(_) => continue,
                            };

                            match serde_json::from_str::<NodeWsMessage>(&text) {
                                Ok(NodeWsMessage::Heartbeat { timestamp }) => {
                                    // Update node heartbeat timestamp
                                    {
                                        let node_state = crate::handlers::node::get_state();
                                        let mut nodes = node_state.write().await;
                                        if let Some(node) = nodes.get_mut(&node_id) {
                                            node.last_heartbeat = Some(timestamp.clone());
                                            node.status = "online".to_string();
                                        }
                                    }
                                    let ack = serde_json::json!({
                                        "type": "heartbeat_ack",
                                        "timestamp": timestamp,
                                        "node_id": node_id,
                                    });
                                    let _ = ws.send(Message::text(ack.to_string())).await;
                                }
                                Ok(NodeWsMessage::Status { status, message }) => {
                                    {
                                        let node_state = crate::handlers::node::get_state();
                                        let mut nodes = node_state.write().await;
                                        if let Some(node) = nodes.get_mut(&node_id) {
                                            node.status = status.clone();
                                        }
                                    }
                                    tracing::info!("Node {} status update: {} - {}", node_id, status, message);
                                }
                                Ok(NodeWsMessage::Log { level, message, .. }) => {
                                    match level.as_str() {
                                        "error" => tracing::error!("Node {}: {}", node_id, message),
                                        "warn" => tracing::warn!("Node {}: {}", node_id, message),
                                        _ => tracing::info!("Node {} [{}]: {}", node_id, level, message),
                                    }
                                }
                                Ok(NodeWsMessage::CommandResponse { command_id, status, .. }) => {
                                    tracing::info!("Node {} command {} completed with status: {}", node_id, command_id, status);
                                }
                                Ok(NodeWsMessage::Command { .. }) => {
                                    tracing::warn!("Node {} sent unexpected command message (commands are server-to-node only)", node_id);
                                }
                                Ok(_) => {
                                    // Silently ignore heartbeat_ack and other server-only messages
                                }
                                Err(_) => {
                                    tracing::warn!("Node {} sent unrecognized message: {}", node_id, text);
                                }
                            }
                        }
                        // Ignore binary, ping, pong frames
                    }
                    Some(Err(e)) => {
                        tracing::error!("WebSocket error for node {}: {}", node_id, e);
                        break;
                    }
                    None => {
                        tracing::info!("WebSocket stream ended for node {}", node_id);
                        break;
                    }
                }
            }

            // Mark node offline on disconnect
            {
                let node_state = crate::handlers::node::get_state();
                let mut nodes = node_state.write().await;
                if let Some(node) = nodes.get_mut(&node_id) {
                    node.ws_connected = false;
                    node.status = "offline".to_string();
                }
            }
            tracing::info!("Node {} WebSocket disconnected", node_id);
        })
        .await
}
