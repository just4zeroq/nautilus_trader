use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolResponse {
    pub id: String,
    pub symbol: String,
    pub exchange: String,
    pub tier: i32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolCreateRequest {
    pub symbol: String,
    pub exchange: String,
    pub tier: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolUpdateTierRequest {
    pub tier: i32,
}
