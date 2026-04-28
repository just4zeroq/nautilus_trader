use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{Utc, DateTime};

use crate::models::{
    NodeListResponse, NodeResponse, NodeHeartbeatRequest,
    RegisterNodeRequest, RegisterNodeResponse,
};
use salvo::prelude::*;

pub type NodeState = Arc<RwLock<HashMap<String, NodeResponse>>>;

static NODE_STATE: OnceLock<NodeState> = OnceLock::new();

pub fn init_state() -> NodeState {
    let state = Arc::new(RwLock::new(HashMap::new()));
    NODE_STATE.set(state.clone()).expect("Node state already initialized");
    state
}

pub fn get_state() -> &'static NodeState {
    NODE_STATE.get().expect("Node state not initialized")
}

// ---------------------------------------------------------------------------
// REST handlers
// ---------------------------------------------------------------------------

#[handler]
pub async fn list_nodes(res: &mut Response) {
    let state = get_state();
    let nodes = state.read().await;
    let nodes_vec: Vec<NodeResponse> = nodes.values().cloned().collect();
    res.render(Json(NodeListResponse { total: nodes_vec.len(), nodes: nodes_vec }));
}

#[handler]
pub async fn heartbeat(req: &mut Request, res: &mut Response) {
    let body = req.parse_json::<NodeHeartbeatRequest>().await.unwrap_or_default();
    let state = get_state();
    let mut nodes = state.write().await;
    let now = chrono::Utc::now().to_rfc3339();

    nodes.entry(body.node_id.clone())
        .and_modify(|n| {
            n.last_heartbeat = Some(now.clone());
            n.status = "online".to_string();
            n.name.clone_from(&body.name);
            n.exchange.clone_from(&body.exchange);
            n.version.clone_from(&body.version);
        })
        .or_insert(NodeResponse {
            id: body.node_id.clone(),
            name: body.name.clone(),
            node_type: body.node_type.clone(),
            exchange: body.exchange.clone(),
            status: "online".to_string(),
            last_heartbeat: Some(now),
            latency: body.latency,
            version: body.version.clone(),
            token: None,
            ws_connected: false,
            api_port: None,
            ip_address: None,
        });

    res.render(Json(serde_json::json!({"status": "ok"})));
}

#[handler]
pub async fn register_node(req: &mut Request, res: &mut Response) {
    let body = req.parse_json::<RegisterNodeRequest>().await;
    match body {
        Ok(register_req) => {
            let node_id = register_req.node_id.clone().unwrap_or_else(|| {
                let short = Uuid::new_v4().to_string().split('-').next().unwrap_or("x").to_string();
                format!("{}-{}", register_req.node_type, short)
            });
            let token = Uuid::new_v4().to_string();

            {
                let state = get_state();
                let mut nodes = state.write().await;
                nodes.insert(node_id.clone(), NodeResponse {
                    id: node_id.clone(),
                    name: node_id.clone(),
                    node_type: register_req.node_type.clone(),
                    exchange: register_req.exchange.clone(),
                    status: "registered".to_string(),
                    last_heartbeat: None,
                    latency: None,
                    version: Some(register_req.version.clone()),
                    token: Some(token.clone()),
                    ws_connected: false,
                    api_port: Some(register_req.api_port),
                    ip_address: Some(register_req.ip.clone()),
                });
            }

            {
                let token_map = crate::handlers::node_ws::get_token_map();
                let mut map = token_map.write().await;
                map.insert(token.clone(), node_id.clone());
            }

            res.render(Json(RegisterNodeResponse {
                node_id,
                ip: register_req.ip,
                api_port: register_req.api_port,
                token,
                node_type: register_req.node_type,
                status: "registered".to_string(),
            }));
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": format!("Invalid request: {}", e)})));
        }
    }
}

// ---------------------------------------------------------------------------
// Node consumer (Redis Streams → in-memory state)
// ---------------------------------------------------------------------------

pub async fn start_node_consumer(state: NodeState) {
    use message_bus::MessageBus;

    let bus = match MessageBus::new().await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("Node consumer bus init error: {}", e);
            return;
        }
    };

    let mut last_id = "0".to_string();

    loop {
        let result = bus.read("system.heartbeat", &last_id, 10).await;
        match result {
            Ok(messages) => {
                for msg in messages {
                    if let Ok(data) = msg.data_as::<serde_json::Value>() {
                        let node_id = msg.source().unwrap_or("unknown").to_string();
                        let name = data["name"].as_str().unwrap_or(&node_id).to_string();
                        let node_type = data["node_type"].as_str().unwrap_or("unknown").to_string();
                        let exchange = data["exchange"].as_str().filter(|s| !s.is_empty()).map(String::from);

                        let node = NodeResponse {
                            id: node_id,
                            name,
                            node_type,
                            exchange,
                            status: "online".to_string(),
                            last_heartbeat: Some(msg.timestamp().unwrap_or_default().to_string()),
                            latency: data["latency"].as_i64().filter(|&v| v > 0),
                            version: data["version"].as_str().map(String::from),
                            token: None,
                            ws_connected: false,
                            api_port: data["api_port"].as_u64().map(|p| p as u16),
                            ip_address: data["ip"].as_str().map(String::from),
                        };

                        let mut nodes = state.write().await;
                        nodes.insert(node.id.clone(), node);
                    }
                    last_id = msg.id;
                }
            }
            Err(e) => {
                tracing::debug!("XREAD (expected until first heartbeat): {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

pub fn start_stale_checker(state: NodeState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            let now = Utc::now();
            let mut nodes = state.write().await;
            for node in nodes.values_mut() {
                if let Some(ref hb) = node.last_heartbeat {
                    if let Ok(dt) = hb.parse::<DateTime<Utc>>() {
                        if (now - dt).num_seconds() > 60 {
                            node.status = "offline".to_string();
                        }
                    }
                }
            }
        }
    });
}
