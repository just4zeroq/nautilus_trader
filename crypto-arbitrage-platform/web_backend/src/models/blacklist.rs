use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistItemResponse {
    pub id: String,
    pub symbol: String,
    pub exchange: String,
    pub reason: Option<String>,
    pub created_at: String,
    pub created_by: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlacklistCreateRequest {
    pub symbol: String,
    pub exchange: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistListResponse {
    pub items: Vec<BlacklistItemResponse>,
    pub total: usize,
}
