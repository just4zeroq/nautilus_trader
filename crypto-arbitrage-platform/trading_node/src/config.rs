use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TradingNodeConfig {
    pub trader_id: String,
    pub instance_id: String,
    pub redis: RedisConfig,
    pub exchanges: ExchangeConfig,
    pub strategies: Vec<StrategyConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub external_streams: Vec<String>,
    pub streams_prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeConfig {
    pub binance: ExchangeCredentials,
    pub okx: ExchangeCredentials,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeCredentials {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StrategyConfig {
    pub id: String,
    pub strategy_type: String,
    pub enabled: bool,
    pub params: serde_yaml::Value,
}