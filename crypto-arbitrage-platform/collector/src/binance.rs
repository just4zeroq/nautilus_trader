use crate::config::{CollectorConfig, RedisConfig, TierConfig};
use crate::redis_publisher::RedisPublisher;
use std::sync::Arc;

pub struct BinanceCollector {
    collector_id: String,
    publisher: Arc<RedisPublisher>,
    tier_config: TierConfig,
}

impl BinanceCollector {
    pub async fn new(
        config: &CollectorConfig,
        redis_config: &RedisConfig,
        tier_config: TierConfig,
    ) -> anyhow::Result<Self> {
        let publisher = RedisPublisher::new(redis_config).await?;

        Ok(Self {
            collector_id: config.collector_id.clone(),
            publisher: Arc::new(publisher),
            tier_config,
        })
    }

    pub fn collector_id(&self) -> &str {
        &self.collector_id
    }

    pub fn tier_config(&self) -> &TierConfig {
        &self.tier_config
    }

    pub fn tier1_symbols(&self) -> &[String] {
        &self.tier_config.tier1_symbols
    }

    pub fn tier2_symbols(&self) -> &[String] {
        &self.tier_config.tier2_symbols
    }

    pub fn tier3_symbols(&self) -> &[String] {
        &self.tier_config.tier3_symbols
    }

    /// 模拟订阅 Tier1 数据（完整 orderbook + trades）
    pub async fn subscribe_tier1(&self, symbols: &[String]) -> anyhow::Result<()> {
        for symbol in symbols {
            let topic = format!("binance:tier1:{}", symbol.to_lowercase());
            let payload = format!("{{\"type\":\"subscribe\",\"symbol\":\"{}\",\"tier\":\"tier1\"}}", symbol);
            self.publisher.publish(&topic, payload.as_bytes()).await?;
        }
        Ok(())
    }

    /// 模拟订阅 Tier2 数据（ticker + trades）
    pub async fn subscribe_tier2(&self, symbols: &[String]) -> anyhow::Result<()> {
        for symbol in symbols {
            let topic = format!("binance:tier2:{}", symbol.to_lowercase());
            let payload = format!("{{\"type\":\"subscribe\",\"symbol\":\"{}\",\"tier\":\"tier2\"}}", symbol);
            self.publisher.publish(&topic, payload.as_bytes()).await?;
        }
        Ok(())
    }

    /// Tier3: Binance 支持在一个 stream 订阅所有 ticker
    pub async fn subscribe_tier3(&self, symbols: &[String]) -> anyhow::Result<()> {
        // Binance combined stream: !ticker@arr 获取所有 ticker
        let topic = "binance:tier3:all".to_string();
        let payload = format!("{{\"type\":\"subscribe\",\"channel\":\"!ticker@arr\",\"symbols\":{:?}}}", symbols);
        self.publisher.publish(&topic, payload.as_bytes()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> (CollectorConfig, RedisConfig, TierConfig) {
        let redis_config = RedisConfig {
            host: "localhost".into(),
            port: 6379,
            password: None,
            streams_prefix: "collector".into(),
        };

        let collector = CollectorConfig {
            collector_id: "test-collector".into(),
            exchange: "binance".into(),
            tiers: vec!["tier1".into(), "tier2".into(), "tier3".into()],
            redis: redis_config.clone(),
        };

        let tier_config = TierConfig {
            tier1_symbols: vec!["BTCUSDT".into(), "ETHUSDT".into()],
            tier2_symbols: vec!["ADAUSDT".into(), "DOGEUSDT".into()],
            tier3_symbols: vec!["SHIBUSDT".into()],
        };

        (collector, redis_config, tier_config)
    }

    #[tokio::test]
    async fn test_binance_collector_creation() {
        let (config, redis_config, tier_config) = create_test_config();
        let collector = BinanceCollector::new(&config, &redis_config, tier_config).await;
        assert!(collector.is_ok());
    }

    #[tokio::test]
    async fn test_tier1_symbols() {
        let (config, redis_config, tier_config) = create_test_config();
        let collector = BinanceCollector::new(&config, &redis_config, tier_config).await.unwrap();
        assert_eq!(collector.tier1_symbols(), &["BTCUSDT", "ETHUSDT"]);
    }
}