use serde_yaml::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;

/// 统计套利策略
/// 支持均值回归和动量两种信号类型
pub struct StatisticalStrategy {
    name: String,
    params: StatisticalParams,
    /// 历史价格 {symbol: VecDeque<f64>}
    price_history: RwLock<HashMap<String, VecDeque<f64>>>,
    /// 滚动均值 {symbol: f64}
    mean: RwLock<HashMap<String, f64>>,
    /// 滚动标准差 {symbol: f64}
    stddev: RwLock<HashMap<String, f64>>,
}

#[derive(Debug, Clone)]
pub struct StatisticalParams {
    pub symbols: Vec<String>,
    pub lookback_period: usize,     // 历史数据窗口
    pub entry_threshold: f64,       // 入场阈值 (标准差倍数)
    pub exit_threshold: f64,        // 退场阈值
    pub signal_type: SignalType,   // 信号类型
    pub order_size: f64,          // 每次下单数量
}

#[derive(Debug, Clone)]
pub enum SignalType {
    MeanReversion,
    Momentum,
}

#[derive(Debug, Clone)]
pub struct StatisticalSignal {
    pub symbol: String,
    pub signal: Signal,
    pub z_score: f64,
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Long,   // 做多
    Short,  // 做空
    Close,  // 平仓
    Hold,   // 持有
}

impl StatisticalStrategy {
    pub fn new(params: &Value) -> anyhow::Result<Self> {
        let symbols = params.get("symbols")
            .and_then(|v| v.as_sequence())
            .map(|v| v.iter().filter_map(|s| s.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let lookback_period = params.get("lookback_period")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize)
            .unwrap_or(100);

        let entry_threshold = params.get("entry_threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(2.0);

        let exit_threshold = params.get("exit_threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5);

        let signal_type_str = params.get("signal_type")
            .and_then(|v| v.as_str())
            .unwrap_or("mean_reversion");

        let signal_type = match signal_type_str {
            "momentum" | "Momentum" => SignalType::Momentum,
            _ => SignalType::MeanReversion,
        };

        let order_size = params.get("order_size")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.1);

        Ok(Self {
            name: "StatisticalStrategy".into(),
            params: StatisticalParams {
                symbols,
                lookback_period,
                entry_threshold,
                exit_threshold,
                signal_type,
                order_size,
            },
            price_history: RwLock::new(HashMap::new()),
            mean: RwLock::new(HashMap::new()),
            stddev: RwLock::new(HashMap::new()),
        })
    }

    /// 更新价格
    pub fn update_price(&self, symbol: &str, price: f64) {
        let mut history = self.price_history.write().unwrap();
        let deque = history.entry(symbol.to_string()).or_insert_with(VecDeque::new);

        deque.push_back(price);
        if deque.len() > self.params.lookback_period {
            deque.pop_front();
        }

        // 如果收集够足够数据，计算统计量
        if deque.len() >= self.params.lookback_period {
            let (m, s) = Self::calculate_statistics(deque);
            self.mean.write().unwrap().insert(symbol.to_string(), m);
            self.stddev.write().unwrap().insert(symbol.to_string(), s);
        }
    }

    /// 计算均值和标准差
    fn calculate_statistics(data: &VecDeque<f64>) -> (f64, f64) {
        let n = data.len() as f64;
        let sum: f64 = data.iter().sum();
        let mean = sum / n;
        let variance: f64 = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
        let stddev = variance.sqrt();
        (mean, stddev)
    }

    /// 生成交易信号
    pub fn generate_signal(&self, symbol: &str, price: f64) -> Option<StatisticalSignal> {
        let mean = self.mean.read().unwrap().get(symbol)?.clone();
        let stddev = self.stddev.read().unwrap().get(symbol)?.clone();

        if stddev == 0.0 {
            return None;
        }

        let z_score = (price - mean) / stddev;

        let signal = match self.params.signal_type {
            SignalType::MeanReversion => {
                if z_score < -self.params.entry_threshold {
                    Signal::Long   // 价格低于均值太多 -> 买入
                } else if z_score > self.params.entry_threshold {
                    Signal::Short  // 价格高于均值太多 -> 卖出
                } else if z_score.abs() < self.params.exit_threshold {
                    Signal::Close   // 回归均值 -> 平仓
                } else {
                    Signal::Hold    // 无信号
                }
            }
            SignalType::Momentum => {
                if z_score > self.params.entry_threshold {
                    Signal::Long   // 动量向上 -> 买入
                } else if z_score < -self.params.entry_threshold {
                    Signal::Short   // 动量向下 -> 卖出
                } else {
                    Signal::Hold
                }
            }
        };

        Some(StatisticalSignal {
            symbol: symbol.to_string(),
            signal,
            z_score,
            price,
            size: self.params.order_size,
        })
    }

    /// 检查是否有信号
    pub fn check_signal(&self, symbol: &str, price: f64) -> Option<StatisticalSignal> {
        self.update_price(symbol, price);
        self.generate_signal(symbol, price)
    }

    pub fn params(&self) -> &StatisticalParams {
        &self.params
    }

    pub fn signal_type(&self) -> &str {
        match self.params.signal_type {
            SignalType::MeanReversion => "mean_reversion",
            SignalType::Momentum => "momentum",
        }
    }
}

impl super::StrategyTrait for StatisticalStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn strategy_type(&self) -> &str {
        "statistical"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategies::StrategyTrait;

    fn create_mean_reversion_strategy() -> StatisticalStrategy {
        let params = serde_yaml::from_str(r#"
            symbols:
              - BTCUSDT
              - ETHUSDT
            lookback_period: 20
            entry_threshold: 2.0
            exit_threshold: 0.5
            signal_type: mean_reversion
            order_size: 0.1
        "#).unwrap();
        StatisticalStrategy::new(&params).unwrap()
    }

    fn create_momentum_strategy() -> StatisticalStrategy {
        let params = serde_yaml::from_str(r#"
            symbols:
              - BTCUSDT
            lookback_period: 20
            entry_threshold: 2.0
            exit_threshold: 0.5
            signal_type: momentum
            order_size: 0.1
        "#).unwrap();
        StatisticalStrategy::new(&params).unwrap()
    }

    #[test]
    fn test_strategy_creation() {
        let strategy = create_mean_reversion_strategy();
        assert_eq!(strategy.name(), "StatisticalStrategy");
        assert_eq!(strategy.params().lookback_period, 20);
        assert_eq!(strategy.params().entry_threshold, 2.0);
        assert_eq!(strategy.signal_type(), "mean_reversion");
    }

    #[test]
    fn test_price_update() {
        let strategy = create_mean_reversion_strategy();

        // 添加 20 个价格数据
        for i in 0..20 {
            let price = 67000.0 + (i as f64) * 10.0;
            strategy.update_price("BTCUSDT", price);
        }

        // 检查统计量是否计算
        let mean_guard = strategy.mean.read().unwrap();
        let mean = mean_guard.get("BTCUSDT");
        assert!(mean.is_some());
        assert!(*mean.unwrap() > 67000.0); // 最后一个价格是 67190
    }

    #[test]
    fn test_mean_reversion_signal() {
        let strategy = create_mean_reversion_strategy();

        // 添加 20 个历史价格
        for i in 0..20 {
            strategy.update_price("BTCUSDT", 67000.0 + (i as f64) * 10.0);
        }

        // 当前价格远低于均值，应该产生 Long 信号
        let signal = strategy.check_signal("BTCUSDT", 60000.0);
        assert!(signal.is_some());
        let sig = signal.unwrap();
        assert_eq!(sig.signal, Signal::Long);
    }

    #[test]
    fn test_momentum_signal() {
        let strategy = create_momentum_strategy();

        // 添加 20 个历史价格（上涨趋势）
        for i in 0..20 {
            strategy.update_price("BTCUSDT", 67000.0 + (i as f64) * 100.0);
        }

        // 当前价格远高于均值，应该产生 Long 信号
        let signal = strategy.check_signal("BTCUSDT", 70000.0);
        assert!(signal.is_some());
        let sig = signal.unwrap();
        assert_eq!(sig.signal, Signal::Long);
    }
}
