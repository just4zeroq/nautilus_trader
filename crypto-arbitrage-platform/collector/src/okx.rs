use crate::config::{CollectorConfig, RedisConfig, TierConfig};
use crate::redis_publisher::RedisPublisher;
use std::sync::Arc;

pub struct OKXCollector {
    collector_id: String,
    publisher: Arc<RedisPublisher>,
    tier_config: TierConfig,
}

/// OKX WebSocket 限制: 最多 50 个 stream 每连接
const OKX_MAX_STREAMS_PER_CONNECTION: usize = 50;

impl OKXCollector {
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

    /// OKX 限速分组订阅 - 每个连接最多 50 个 stream
    pub async fn subscribe_with_rate_limit(
        &self,
        symbols: &[String],
        tier: &str,
    ) -> anyhow::Result<()> {
        for (i, chunk) in symbols.chunks(OKX_MAX_STREAMS_PER_CONNECTION).enumerate() {
            let connection_id = i + 1;
            for symbol in chunk {
                let topic = format!("okx:{tier}:{}", symbol.to_lowercase());
                let payload = format!(
                    "{{\"type\":\"subscribe\",\"symbol\":\"{}\",\"tier\":\"{}\",\"connection\":{}}}",
                    symbol, tier, connection_id
                );
                self.publisher.publish(&topic, payload.as_bytes()).await?;
            }
            // 避免触发 OKX 限速
            if symbols.len() > OKX_MAX_STREAMS_PER_CONNECTION {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        Ok(())
    }

    /// 订阅 Tier1（完整数据）
    pub async fn subscribe_tier1(&self, symbols: &[String]) -> anyhow::Result<()> {
        self.subscribe_with_rate_limit(symbols, "tier1").await
    }

    /// 订阅 Tier2（ticker + trades）
    pub async fn subscribe_tier2(&self, symbols: &[String]) -> anyhow::Result<()> {
        self.subscribe_with_rate_limit(symbols, "tier2").await
    }

    /// 订阅 Tier3（仅 ticker）
    pub async fn subscribe_tier3(&self, symbols: &[String]) -> anyhow::Result<()> {
        self.subscribe_with_rate_limit(symbols, "tier3").await
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
            collector_id: "test-okx-collector".into(),
            exchange: "okx".into(),
            tiers: vec!["tier1".into(), "tier2".into(), "tier3".into()],
            redis: redis_config.clone(),
        };

        let tier_config = TierConfig {
            tier1_symbols: vec!["BTC-USDT".into(), "ETH-USDT".into()],
            tier2_symbols: vec!["ADA-USDT".into(), "DOGE-USDT".into()],
            tier3_symbols: vec!["SHIB-USDT".into()],
        };

        (collector, redis_config, tier_config)
    }

    #[tokio::test]
    async fn test_okx_collector_creation() {
        let (config, redis_config, tier_config) = create_test_config();
        let collector = OKXCollector::new(&config, &redis_config, tier_config).await;
        assert!(collector.is_ok());
    }

    #[tokio::test]
    async fn test_tier_symbols() {
        let (config, redis_config, tier_config) = create_test_config();
        let collector = OKXCollector::new(&config, &redis_config, tier_config).await.unwrap();
        assert_eq!(collector.tier1_symbols(), &["BTC-USDT", "ETH-USDT"]);
        assert_eq!(collector.tier2_symbols(), &["ADA-USDT", "DOGE-USDT"]);
    }

    #[test]
    fn test_rate_limit_constant() {
        assert_eq!(OKX_MAX_STREAMS_PER_CONNECTION, 50);
    }
}