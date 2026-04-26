use serde_yaml::Value;
use std::collections::HashMap;
use std::sync::RwLock;

/// 三角套利策略
/// 监控单交易所内的三角货币对汇率，寻找套利机会
pub struct TriangularStrategy {
    name: String,
    params: TriangularParams,
    /// 存储各货币对的最新价格 {symbol: (bid, ask)}
    prices: RwLock<HashMap<String, (f64, f64)>>,
}

#[derive(Debug, Clone)]
pub struct TriangularParams {
    pub venue: String,             // 交易所，如 "binance"
    pub legs: Vec<TriangleLeg>,    // 三角货币对
    pub min_profit: f64,          // 最小利润阈值 (如 0.001 = 0.1%)
    pub order_size: f64,          // 每次下单数量
}

#[derive(Debug, Clone)]
pub struct TriangleLeg {
    pub base: String,    // 基础货币，如 "BTC"
    pub quote: String,  // 报价货币，如 "USDT"
}

#[derive(Debug, Clone)]
pub struct TriangleOpportunity {
    pub legs: Vec<TriangleLeg>,
    pub buy_symbol: String,
    pub sell_symbol: String,
    pub profit_rate: f64,      // 利润率
    pub estimated_profit: f64,  // 预估利润 (USDT)
    pub size: f64,             // 交易数量
}

impl TriangularStrategy {
    pub fn new(params: &Value) -> anyhow::Result<Self> {
        let venue = params.get("venue")
            .and_then(|v| v.as_str())
            .unwrap_or("binance")
            .to_string();

        let legs = params.get("legs")
            .and_then(|v| v.as_sequence())
            .map(|v| {
                v.iter().filter_map(|leg| {
                    let base = leg.get("base")?.as_str()?.to_string();
                    let quote = leg.get("quote")?.as_str()?.to_string();
                    Some(TriangleLeg { base, quote })
                }).collect()
            })
            .unwrap_or_else(|| {
                vec![
                    TriangleLeg { base: "BTC".into(), quote: "USDT".into() },
                    TriangleLeg { base: "ETH".into(), quote: "USDT".into() },
                    TriangleLeg { base: "ETH".into(), quote: "BTC".into() },
                ]
            });

        let min_profit = params.get("min_profit")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.001);

        let order_size = params.get("order_size")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.1);

        Ok(Self {
            name: "TriangularStrategy".into(),
            params: TriangularParams {
                venue,
                legs,
                min_profit,
                order_size,
            },
            prices: RwLock::new(HashMap::new()),
        })
    }

    /// 更新货币对价格
    pub fn update_price(&self, symbol: &str, bid: f64, ask: f64) {
        let mut prices = self.prices.write().unwrap();
        prices.insert(symbol.to_string(), (bid, ask));
    }

    /// 生成交易对符号
    fn make_symbol(base: &str, quote: &str) -> String {
        format!("{}{}", base, quote)
    }

    /// 计算三角套利利润率
    ///
    /// 假设我们有 USDT -> BTC -> ETH -> USDT 的路径
    /// 利润率 = (输入金额 * 汇率1 * 汇率2 * 汇率3) / 输入金额 - 1
    pub fn calculate_triangle_profit(&self, leg1: &TriangleLeg, leg2: &TriangleLeg, leg3: &TriangleLeg) -> Option<f64> {
        let prices = self.prices.read().unwrap();

        // 路径: USDT -> BTC (用 USDT 买 BTC)
        let symbol1 = Self::make_symbol(&leg1.base, &leg1.quote); // BTCUSDT
        // 路径: BTC -> ETH (用 BTC 买 ETH)
        let symbol2 = Self::make_symbol(&leg2.base, &leg2.quote); // ETHBTC
        // 路径: ETH -> USDT (用 ETH 换回 USDT)
        let symbol3 = Self::make_symbol(&leg3.base, &leg3.quote); // ETHUSDT

        let (_bid1, ask1) = prices.get(&symbol1)?; // BTCUSDT: bid=买一价, ask=卖一价
        let (_bid2, ask2) = prices.get(&symbol2)?; // ETHBTC: bid=买一价, ask=卖一价
        let (bid3, _ask3) = prices.get(&symbol3)?; // ETHUSDT: bid=买一价, ask=卖一价

        // 使用 ask 价格买入（需要付出更多），使用 bid 价格卖出（得到更少）
        // 这是交易者的视角

        // 假设我们有 1 USDT
        // 1. 用 USDT 买 BTC: 得到 BTC = 1 / ask1
        // 2. 用 BTC 买 ETH: 得到 ETH = (1 / ask1) / ask2
        // 3. 用 ETH 卖 USDT: 得到 USDT = ((1 / ask1) / ask2) * bid3

        let start = 1.0_f64;
        let step1 = start / ask1; // USDT -> BTC
        let step2 = step1 / ask2; // BTC -> ETH
        let end = step2 * bid3;  // ETH -> USDT

        let profit_rate = (end / start) - 1.0;
        Some(profit_rate)
    }

    /// 检查三角套利机会
    pub fn check_opportunity(&self) -> Option<TriangleOpportunity> {
        // 遍历所有可能的三角组合
        for leg1 in &self.params.legs {
            for leg2 in &self.params.legs {
                for leg3 in &self.params.legs {
                    // 确保三个 leg 形成一个环
                    if leg1.quote != leg3.base {
                        continue;
                    }
                    if leg2.base != leg1.base || leg2.quote != leg3.quote {
                        continue;
                    }

                    if let Some(profit_rate) = self.calculate_triangle_profit(leg1, leg2, leg3) {
                        if profit_rate > self.params.min_profit {
                            let symbol_buy = Self::make_symbol(&leg1.base, &leg1.quote);
                            let symbol_sell = Self::make_symbol(&leg3.base, &leg3.quote);

                            return Some(TriangleOpportunity {
                                legs: vec![leg1.clone(), leg2.clone(), leg3.clone()],
                                buy_symbol: symbol_buy,
                                sell_symbol: symbol_sell,
                                profit_rate,
                                estimated_profit: self.params.order_size * profit_rate,
                                size: self.params.order_size,
                            });
                        }
                    }
                }
            }
        }
        None
    }

    pub fn params(&self) -> &TriangularParams {
        &self.params
    }
}

impl super::StrategyTrait for TriangularStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn strategy_type(&self) -> &str {
        "triangular"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::StrategyTrait;

    fn create_test_strategy() -> TriangularStrategy {
        let params = serde_yaml::from_str(r#"
            venue: binance
            legs:
              - base: BTC
                quote: USDT
              - base: ETH
                quote: USDT
              - base: ETH
                quote: BTC
            min_profit: 0.001
            order_size: 1.0
        "#).unwrap();
        TriangularStrategy::new(&params).unwrap()
    }

    #[test]
    fn test_strategy_creation() {
        let strategy = create_test_strategy();
        assert_eq!(strategy.name(), "TriangularStrategy");
        assert_eq!(strategy.params().min_profit, 0.001);
        assert_eq!(strategy.params().legs.len(), 3);
    }

    #[test]
    fn test_price_update() {
        let strategy = create_test_strategy();

        strategy.update_price("BTCUSDT", 67000.0, 67010.0);
        strategy.update_price("ETHUSDT", 3500.0, 3505.0);
        strategy.update_price("ETHBTC", 0.0520, 0.0521);

        // 此时应该有套利机会
        let _opp = strategy.check_opportunity();
        // 如果有正向利润，应该返回机会
        // 利润率 = (1/67010 * 1/0.0521 * 3500) - 1
        // = 0.05206 - 1 = -0.0006 (负数，正常市场)
        // 如果实际计算为正，说明有套利机会
    }

    #[test]
    fn test_make_symbol() {
        assert_eq!(TriangularStrategy::make_symbol("BTC", "USDT"), "BTCUSDT");
        assert_eq!(TriangularStrategy::make_symbol("ETH", "BTC"), "ETHBTC");
    }
}
