use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    pub exchange: String,
    pub asset: String,
    pub balance: f64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionResponse {
    pub id: String,
    pub symbol: String,
    pub exchange: String,
    pub side: String,
    pub quantity: f64,
    pub avg_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: String,
    pub symbol: String,
    pub exchange: String,
    pub side: String,
    pub order_type: String,
    pub price: f64,
    pub quantity: f64,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub id: String,
    pub exchange: String,
    pub label: Option<String>,
    pub api_key_configured: bool,
    pub api_key_mask: Option<String>,
    pub assets: Vec<AccountBalanceResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountListResponse {
    pub accounts: Vec<AccountResponse>,
    pub total: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiKeyConfigRequest {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: Option<String>,
}
