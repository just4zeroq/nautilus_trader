use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub streams_prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectorConfig {
    pub collector_id: String,
    pub exchange: String,  // "binance" or "okx"
    pub tiers: Vec<String>,  // ["tier1", "tier2", "tier3"]
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TierConfig {
    pub tier1_symbols: Vec<String>,
    pub tier2_symbols: Vec<String>,
    pub tier3_symbols: Vec<String>,
}