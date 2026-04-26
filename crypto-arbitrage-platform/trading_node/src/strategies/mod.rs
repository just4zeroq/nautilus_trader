pub mod cross_exchange;
pub mod triangular;
pub mod statistical;

use crate::config::StrategyConfig;

/// 创建策略实例
/// 根据 strategy_type 返回对应的策略实例
pub fn create_strategy(config: &StrategyConfig) -> anyhow::Result<Box<dyn StrategyTrait>> {
    match config.strategy_type.as_str() {
        "cross_exchange" => Ok(Box::new(
            cross_exchange::CrossExchangeStrategy::new(&config.params)?
        )),
        "triangular" => Ok(Box::new(
            triangular::TriangularStrategy::new(&config.params)?
        )),
        "statistical" => Ok(Box::new(
            statistical::StatisticalStrategy::new(&config.params)?
        )),
        _ => anyhow::bail!(
            "Unknown strategy type: {}. Supported: cross_exchange, triangular, statistical",
            config.strategy_type
        ),
    }
}

/// 策略特征 - 所有策略必须实现
pub trait StrategyTrait: Send + Sync {
    fn name(&self) -> &str;
    fn strategy_type(&self) -> &str;
}