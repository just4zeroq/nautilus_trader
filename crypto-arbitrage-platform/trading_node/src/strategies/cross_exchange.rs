use serde_yaml::Value;
use std::collections::HashMap;
use std::sync::RwLock;

/// 跨交易所价差套利策略
pub struct CrossExchangeStrategy {
    name: String,
    params: CrossExchangeParams,
    /// 存储各交易对的价格 {symbol: PricePair}
    prices: RwLock<HashMap<String, PricePair>>,
}

#[derive(Debug, Clone)]
pub struct CrossExchangeParams {
    pub symbols: Vec<String>,
    pub spread_threshold: f64,    // 触发交易的价差阈值 (USDT)
    pub max_position: f64,       // 最大持仓
    pub order_size: f64,         // 每次下单数量
}

#[derive(Debug, Clone, Default)]
struct PricePair {
    binance_bid: f64,
    binance_ask: f64,
    okx_bid: f64,
    okx_ask: f64,
}

impl CrossExchangeStrategy {
    pub fn new(params: &Value) -> anyhow::Result<Self> {
        let symbols = params.get("symbols")
            .and_then(|v| v.as_sequence())
            .map(|v| v.iter().filter_map(|s| s.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let spread_threshold = params.get("spread_threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(10.0);

        let max_position = params.get("max_position")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        let order_size = params.get("order_size")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.1);

        Ok(Self {
            name: "CrossExchangeStrategy".into(),
            params: CrossExchangeParams {
                symbols,
                spread_threshold,
                max_position,
                order_size,
            },
            prices: RwLock::new(HashMap::new()),
        })
    }

    /// 更新交易所价格
    pub fn update_price(&self, exchange: &str, symbol: &str, bid: f64, ask: f64) {
        let mut prices = self.prices.write().unwrap();
        let pair = prices.entry(symbol.to_string()).or_insert_with(PricePair::default);

        match exchange.to_uppercase().as_str() {
            "BINANCE" | "BINANCE_SPOT" => {
                pair.binance_bid = bid;
                pair.binance_ask = ask;
            }
            "OKX" | "OKX_SPOT" => {
                pair.okx_bid = bid;
                pair.okx_ask = ask;
            }
            _ => {}
        }
    }

    /// 检查是否有套利机会
    pub fn check_arbitrage(&self, symbol: &str) -> Option<ArbitrageOpportunity> {
        let prices = self.prices.read().unwrap();
        let pair = prices.get(symbol)?;

        if pair.binance_bid == 0.0 || pair.okx_bid == 0.0 {
            return None;
        }

        // 情况1: Binance bid < OKX ask -> 在 Binance 买，OKX 卖
        // 价格从 OKX 高价卖，Binance 低价买
        let spread1 = pair.okx_ask - pair.binance_bid;
        if spread1 > self.params.spread_threshold {
            return Some(ArbitrageOpportunity {
                symbol: symbol.to_string(),
                direction: ArbDirection::BinanceBuyOkxSell,
                buy_exchange: "binance".to_string(),
                sell_exchange: "okx".to_string(),
                buy_price: pair.binance_bid,
                sell_price: pair.okx_ask,
                spread: spread1,
                size: self.params.order_size,
            });
        }

        // 情况2: OKX bid < Binance ask -> 在 OKX 买，Binance 卖
        let spread2 = pair.binance_ask - pair.okx_bid;
        if spread2 > self.params.spread_threshold {
            return Some(ArbitrageOpportunity {
                symbol: symbol.to_string(),
                direction: ArbDirection::OkxBuyBinanceSell,
                buy_exchange: "okx".to_string(),
                sell_exchange: "binance".to_string(),
                buy_price: pair.okx_bid,
                sell_price: pair.binance_ask,
                spread: spread2,
                size: self.params.order_size,
            });
        }

        None
    }

    pub fn params(&self) -> &CrossExchangeParams {
        &self.params
    }
}

#[derive(Debug, Clone)]
pub struct ArbitrageOpportunity {
    pub symbol: String,
    pub direction: ArbDirection,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub spread: f64,
    pub size: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArbDirection {
    BinanceBuyOkxSell,
    OkxBuyBinanceSell,
}

impl super::StrategyTrait for CrossExchangeStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn strategy_type(&self) -> &str {
        "cross_exchange"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::StrategyTrait;

    fn create_test_strategy() -> CrossExchangeStrategy {
        let params = serde_yaml::from_str(r#"
            symbols:
              - BTCUSDT
              - ETHUSDT
            spread_threshold: 10.0
            max_position: 1.0
            order_size: 0.1
        "#).unwrap();
        CrossExchangeStrategy::new(&params).unwrap()
    }

    #[test]
    fn test_strategy_creation() {
        let strategy = create_test_strategy();
        assert_eq!(strategy.name(), "CrossExchangeStrategy");
        assert_eq!(strategy.params().spread_threshold, 10.0);
        assert_eq!(strategy.params().order_size, 0.1);
    }

    #[test]
    fn test_price_update() {
        let strategy = create_test_strategy();

        strategy.update_price("binance", "BTCUSDT", 67500.0, 67501.0);
        strategy.update_price("okx", "BTCUSDT", 67510.0, 67511.0);

        let opp = strategy.check_arbitrage("BTCUSDT");
        assert!(opp.is_some());

        let opp = opp.unwrap();
        assert_eq!(opp.symbol, "BTCUSDT");
        // OKX ask (67511) - Binance bid (67500) = 11 > threshold (10)
        assert!(opp.spread > 10.0);
    }

    #[test]
    fn test_no_opportunity_below_threshold() {
        let strategy = create_test_strategy();

        // 价差只有 5 USDT，低于阈值
        strategy.update_price("binance", "BTCUSDT", 67500.0, 67501.0);
        strategy.update_price("okx", "BTCUSDT", 67504.0, 67505.0);

        let opp = strategy.check_arbitrage("BTCUSDT");
        // 价差 = 67505 - 67500 = 5 < 10，不应触发
        assert!(opp.is_none());
    }

    #[test]
    fn test_opposite_direction() {
        let params = serde_yaml::from_str(r#"
            symbols:
              - BTCUSDT
            spread_threshold: 10.0
            max_position: 1.0
            order_size: 0.1
        "#).unwrap();
        let strategy = CrossExchangeStrategy::new(&params).unwrap();

        // 反过来：Binance ask > OKX bid -> OKX 买，Binance 卖
        strategy.update_price("binance", "BTCUSDT", 67510.0, 67511.0);
        strategy.update_price("okx", "BTCUSDT", 67500.0, 67501.0);

        let opp = strategy.check_arbitrage("BTCUSDT");
        assert!(opp.is_some());

        let opp = opp.unwrap();
        // Binance ask (67511) - OKX bid (67500) = 11 > 10
        assert_eq!(opp.direction, ArbDirection::OkxBuyBinanceSell);
        assert_eq!(opp.buy_exchange, "okx");
        assert_eq!(opp.sell_exchange, "binance");
    }
}
