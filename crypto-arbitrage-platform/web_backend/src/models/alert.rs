use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRuleResponse {
    pub id: String,
    pub name: String,
    pub level: String,
    pub conditions: serde_json::Value,
    pub channels: Vec<String>,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertRuleCreateRequest {
    pub name: String,
    pub level: String,
    pub conditions: serde_json::Value,
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertRuleUpdateRequest {
    pub name: Option<String>,
    pub level: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub channels: Option<Vec<String>>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRuleListResponse {
    pub rules: Vec<AlertRuleResponse>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannelResponse {
    pub id: String,
    pub name: String,
    pub channel_type: String,
    pub config: serde_json::Value,
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertChannelCreateRequest {
    pub name: String,
    pub channel_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertChannelUpdateRequest {
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannelListResponse {
    pub channels: Vec<AlertChannelResponse>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertHistoryResponse {
    pub id: String,
    pub rule_id: String,
    pub rule_name: String,
    pub level: String,
    pub conditions: serde_json::Value,
    pub triggered_value: serde_json::Value,
    pub channels: serde_json::Value,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertHistoryListResponse {
    pub items: Vec<AlertHistoryResponse>,
    pub total: usize,
}
