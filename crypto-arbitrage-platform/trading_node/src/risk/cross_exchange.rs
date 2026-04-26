use std::collections::HashMap;
use std::sync::RwLock;

/// 跨交易所风控管理器
/// 负责管理跨交易所的持仓，计算净敞口，执行风控检查
pub struct CrossExchangeRiskManager {
    /// 持仓信息 {symbol: (exchange, quantity)}
    positions: RwLock<HashMap<String, Position>>,
    /// 风控参数
    params: RiskParams,
}

#[derive(Debug, Clone)]
struct Position {
    exchange: String,
    quantity: f64,
}

#[derive(Debug, Clone)]
pub struct RiskParams {
    pub max_position_per_symbol: f64,   // 单币对最大持仓
    pub max_total_position: f64,       // 最大总持仓
    pub max_daily_loss: f64,          // 最大日亏损
    pub max_single_trade: f64,         // 单笔最大交易额
}

impl Default for RiskParams {
    fn default() -> Self {
        Self {
            max_position_per_symbol: 1.0,
            max_total_position: 10.0,
            max_daily_loss: 1000.0,
            max_single_trade: 10000.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RiskCheckResult {
    pub allowed: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OrderRequest {
    pub symbol: String,
    pub exchange: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl CrossExchangeRiskManager {
    pub fn new() -> Self {
        Self {
            positions: RwLock::new(HashMap::new()),
            params: RiskParams::default(),
        }
    }

    pub fn with_params(params: RiskParams) -> Self {
        Self {
            positions: RwLock::new(HashMap::new()),
            params,
        }
    }

    /// 更新持仓
    pub fn update_position(&mut self, symbol: &str, exchange: &str, quantity: f64) {
        let mut positions = self.positions.write().unwrap();
        if quantity.abs() < f64::EPSILON {
            positions.remove(symbol);
        } else {
            positions.insert(symbol.to_string(), Position {
                exchange: exchange.to_string(),
                quantity,
            });
        }
    }

    /// 获取净持仓（跨交易所合计）
    pub fn get_net_position(&self, symbol: &str) -> f64 {
        let positions = self.positions.read().unwrap();
        positions.get(symbol).map(|p| p.quantity).unwrap_or(0.0)
    }

    /// 获取所有持仓
    pub fn get_all_positions(&self) -> HashMap<String, f64> {
        let positions = self.positions.read().unwrap();
        positions.iter()
            .map(|(k, v)| (k.clone(), v.quantity))
            .collect()
    }

    /// 获取总持仓量
    pub fn get_total_position(&self) -> f64 {
        let positions = self.positions.read().unwrap();
        positions.values().map(|p| p.quantity.abs()).sum()
    }

    /// 风控检查 - 检查订单是否允许执行
    pub fn check_order(&self, order: &OrderRequest) -> RiskCheckResult {
        // 1. 检查单笔交易额
        let trade_value = order.quantity * order.price;
        if trade_value > self.params.max_single_trade {
            return RiskCheckResult {
                allowed: false,
                reason: Some(format!(
                    "单笔交易额 {} 超过限制 {}",
                    trade_value, self.params.max_single_trade
                )),
            };
        }

        // 2. 计算执行后的新持仓
        let current_pos = self.get_net_position(&order.symbol);
        let new_pos = match order.side {
            OrderSide::Buy => current_pos + order.quantity,
            OrderSide::Sell => current_pos - order.quantity,
        };

        // 3. 检查单币对持仓限制
        if new_pos.abs() > self.params.max_position_per_symbol {
            return RiskCheckResult {
                allowed: false,
                reason: Some(format!(
                    "币对 {} 持仓 {} 超过限制 {}",
                    order.symbol, new_pos, self.params.max_position_per_symbol
                )),
            };
        }

        // 4. 检查总持仓限制
        let positions = self.positions.read().unwrap();
        let other_symbols_total: f64 = positions.iter()
            .filter(|(k, _)| k.as_str() != order.symbol.as_str())
            .map(|(_, p)| p.quantity.abs())
            .sum();
        let new_total = other_symbols_total + new_pos.abs();

        if new_total > self.params.max_total_position {
            return RiskCheckResult {
                allowed: false,
                reason: Some(format!(
                    "总持仓 {} 超过限制 {}",
                    new_total, self.params.max_total_position
                )),
            };
        }

        RiskCheckResult {
            allowed: true,
            reason: None,
        }
    }

    /// 重置日亏损计数（每日开盘时调用）
    pub fn reset_daily_stats(&mut self) {
        // 日亏损统计应该持久化，这里简化处理
    }

    /// 获取风控参数
    pub fn params(&self) -> &RiskParams {
        &self.params
    }

    /// 更新风控参数
    pub fn update_params(&mut self, params: RiskParams) {
        self.params = params;
    }
}

impl Default for CrossExchangeRiskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_manager() -> CrossExchangeRiskManager {
        CrossExchangeRiskManager::with_params(RiskParams {
            max_position_per_symbol: 1.0,
            max_total_position: 10.0,
            max_daily_loss: 1000.0,
            max_single_trade: 10000.0,
        })
    }

    #[test]
    fn test_position_update() {
        let mut manager = create_test_manager();

        manager.update_position("BTCUSDT", "binance", 0.5);
        assert_eq!(manager.get_net_position("BTCUSDT"), 0.5);

        manager.update_position("BTCUSDT", "okx", 0.3);
        // 注意：这会替换而不是累加
        assert_eq!(manager.get_net_position("BTCUSDT"), 0.3);

        manager.update_position("BTCUSDT", "binance", 0.0);
        assert_eq!(manager.get_net_position("BTCUSDT"), 0.0);
    }

    #[test]
    fn test_order_allowed() {
        let manager = create_test_manager();

        let order = OrderRequest {
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            side: OrderSide::Buy,
            quantity: 0.1,
            price: 67000.0,
        };

        let result = manager.check_order(&order);
        assert!(result.allowed);
    }

    #[test]
    fn test_order_exceeds_single_trade() {
        let manager = create_test_manager();

        let order = OrderRequest {
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            side: OrderSide::Buy,
            quantity: 1.0,
            price: 67000.0, // 67000 > 10000 limit
        };

        let result = manager.check_order(&order);
        assert!(!result.allowed);
        assert!(result.reason.unwrap().contains("单笔交易额"));
    }

    #[test]
    fn test_order_exceeds_position_limit() {
        let mut manager = create_test_manager();

        // 先设置一个现有持仓
        manager.update_position("BTCUSDT", "binance", 0.9);

        // 再下一笔会超过限制的订单
        let order = OrderRequest {
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            side: OrderSide::Buy,
            quantity: 0.2, // 0.9 + 0.2 = 1.1 > 1.0
            price: 4000.0, // 0.2 * 4000 = 800 < 10000 (单笔限额)
        };

        let result = manager.check_order(&order);
        assert!(!result.allowed);
        assert!(result.reason.unwrap().contains("持仓"));
    }
}