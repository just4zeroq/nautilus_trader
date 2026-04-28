pub mod cross_exchange;
pub mod triangular;
pub mod statistical;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::risk::cross_exchange::CrossExchangeRiskManager;

pub use cross_exchange::CrossExchangeStrategy;
pub use triangular::TriangularStrategy;
pub use statistical::StatisticalStrategy;

/// Strategy engine — manages the lifecycle of running strategies.
pub struct StrategyEngine {
    pub risk_manager: Arc<RwLock<CrossExchangeRiskManager>>,
    pub active: Arc<RwLock<Vec<String>>>,
}

impl StrategyEngine {
    pub fn new(risk_manager: Arc<RwLock<CrossExchangeRiskManager>>) -> Self {
        Self {
            risk_manager,
            active: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start(&self, id: &str) -> bool {
        let mut active = self.active.write().await;
        if active.contains(&id.to_string()) {
            return false;
        }
        active.push(id.to_string());
        true
    }

    pub async fn stop(&self, id: &str) -> bool {
        let mut active = self.active.write().await;
        if let Some(pos) = active.iter().position(|s| s == id) {
            active.remove(pos);
            return true;
        }
        false
    }

    pub async fn is_running(&self, id: &str) -> bool {
        self.active.read().await.contains(&id.to_string())
    }
}

/// Strategy trait — all strategies must implement.
pub trait StrategyTrait: Send + Sync {
    fn name(&self) -> &str;
    fn strategy_type(&self) -> &str;
}

/// Create a strategy instance from config.
pub fn create_strategy(config: &crate::config::StrategyConfig) -> anyhow::Result<Box<dyn StrategyTrait>> {
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
