use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StrategyRecord {
    pub id: String,
    pub name: String,
    pub strategy_type: String,
    pub enabled: bool,
    pub params: sqlx::types::Json<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SymbolRecord {
    pub id: String,
    pub symbol: String,
    pub exchange: String,
    pub tier: i32,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PositionRecord {
    pub id: String,
    pub symbol: String,
    pub exchange: String,
    pub side: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AccountRecord {
    pub id: String,
    pub exchange: String,
    pub asset: String,
    pub balance: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AlertRuleRecord {
    pub id: String,
    pub name: String,
    pub level: String,
    pub condition: String,
    pub channels: Vec<String>,
    pub enabled: bool,
    pub phone_interval_secs: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
