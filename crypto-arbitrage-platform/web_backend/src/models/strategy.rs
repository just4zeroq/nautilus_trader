use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyResponse {
    pub id: String,
    pub name: String,
    pub strategy_type: String,
    pub enabled: bool,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyStartRequest {
    pub strategy_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyStopRequest {
    pub strategy_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyStatusResponse {
    pub strategy_id: String,
    pub status: String,
    pub message: Option<String>,
}
