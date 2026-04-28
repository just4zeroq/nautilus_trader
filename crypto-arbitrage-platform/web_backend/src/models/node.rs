use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResponse {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub exchange: Option<String>,
    pub status: String,
    pub last_heartbeat: Option<String>,
    pub latency: Option<i64>,
    pub version: Option<String>,
    pub token: Option<String>,
    pub ws_connected: bool,
    pub api_port: Option<u16>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeHeartbeatRequest {
    pub node_id: String,
    pub name: String,
    pub node_type: String,
    pub exchange: Option<String>,
    pub latency: Option<i64>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeListResponse {
    pub nodes: Vec<NodeResponse>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterNodeRequest {
    pub node_id: Option<String>,
    pub node_type: String,
    pub exchange: Option<String>,
    pub ip: String,
    pub api_port: u16,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterNodeResponse {
    pub node_id: String,
    pub ip: String,
    pub api_port: u16,
    pub token: String,
    pub node_type: String,
    pub status: String,
}
