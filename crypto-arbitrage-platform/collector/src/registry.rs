use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collector 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectorInfo {
    pub id: String,
    pub exchange: String,
    pub capacity: usize,
    pub current_load: usize,
    pub tiers: Vec<String>,
}

/// 交易对分配记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolAssignment {
    pub symbol: String,
    pub exchange: String,
    pub tier: String,
    pub collector_id: String,
}

/// Collector 注册表 - 管理交易对分配
pub struct CollectorRegistry {
    collectors: HashMap<String, CollectorInfo>,
    assignments: HashMap<String, SymbolAssignment>,
}

impl CollectorRegistry {
    pub fn new() -> Self {
        Self {
            collectors: HashMap::new(),
            assignments: HashMap::new(),
        }
    }

    /// 注册新的 Collector
    pub fn register(&mut self, collector: CollectorInfo) {
        self.collectors.insert(collector.id.clone(), collector);
    }

    /// 注销 Collector
    pub fn unregister(&mut self, collector_id: &str) -> Option<CollectorInfo> {
        // 移除该 collector 的所有分配
        self.assignments.retain(|_, a| a.collector_id != collector_id);
        self.collectors.remove(collector_id)
    }

    /// 动态分配交易对到最空闲的 Collector
    pub fn assign_symbol(
        &mut self,
        symbol: &str,
        exchange: &str,
        tier: &str,
    ) -> anyhow::Result<String> {
        // 找到适合的 collector: exchange 匹配 + 有剩余容量
        let suitable: Vec<_> = self.collectors.values()
            .filter(|c| c.exchange == exchange && c.current_load < c.capacity)
            .collect();

        if suitable.is_empty() {
            anyhow::bail!(
                "No available collector for {}/{} (all collectors at capacity or wrong exchange)",
                exchange,
                symbol
            );
        }

        // 选择负载最低的 Collector
        let chosen = suitable.iter().min_by_key(|c| c.current_load).unwrap();
        let collector_id = chosen.id.clone();

        // 更新分配
        self.assignments.insert(symbol.to_string(), SymbolAssignment {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            tier: tier.to_string(),
            collector_id: collector_id.clone(),
        });

        // 更新 collector 负载
        if let Some(c) = self.collectors.get_mut(&collector_id) {
            c.current_load += 1;
        }

        Ok(collector_id)
    }

    /// 升级交易对到更高层级
    pub fn upgrade_symbol(&mut self, symbol: &str, target_tier: &str) -> anyhow::Result<()> {
        if let Some(assignment) = self.assignments.get_mut(symbol) {
            assignment.tier = target_tier.to_string();
            Ok(())
        } else {
            anyhow::bail!("Symbol {} not found", symbol)
        }
    }

    /// 降级交易对到更低层级
    pub fn downgrade_symbol(&mut self, symbol: &str, target_tier: &str) -> anyhow::Result<()> {
        self.upgrade_symbol(symbol, target_tier)
    }

    /// 获取交易对的分配信息
    pub fn get_assignment(&self, symbol: &str) -> Option<&SymbolAssignment> {
        self.assignments.get(symbol)
    }

    /// 获取所有分配
    pub fn all_assignments(&self) -> &HashMap<String, SymbolAssignment> {
        &self.assignments
    }

    /// 获取某个 Collector 的所有分配
    pub fn get_collector_assignments(&self, collector_id: &str) -> Vec<&SymbolAssignment> {
        self.assignments.values()
            .filter(|a| a.collector_id == collector_id)
            .collect()
    }

    /// 获取某个 Collector 的负载信息
    pub fn get_collector_load(&self, collector_id: &str) -> Option<(usize, usize)> {
        self.collectors.get(collector_id)
            .map(|c| (c.current_load, c.capacity))
    }
}

impl Default for CollectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_registry() -> CollectorRegistry {
        let mut registry = CollectorRegistry::new();

        registry.register(CollectorInfo {
            id: "binance-t1".into(),
            exchange: "binance".into(),
            capacity: 100,
            current_load: 0,
            tiers: vec!["tier1".into()],
        });

        registry.register(CollectorInfo {
            id: "okx-t1".into(),
            exchange: "okx".into(),
            capacity: 100,
            current_load: 0,
            tiers: vec!["tier1".into()],
        });

        registry
    }

    #[test]
    fn test_symbol_assignment() {
        let mut registry = create_test_registry();

        let collector_id = registry.assign_symbol("BTCUSDT", "binance", "tier1").unwrap();
        assert_eq!(collector_id, "binance-t1");

        // 验证分配已记录
        let assignment = registry.get_assignment("BTCUSDT").unwrap();
        assert_eq!(assignment.exchange, "binance");
        assert_eq!(assignment.tier, "tier1");

        // 验证负载已更新
        let (load, capacity) = registry.get_collector_load("binance-t1").unwrap();
        assert_eq!(load, 1);
        assert_eq!(capacity, 100);
    }

    #[test]
    fn test_upgrade_symbol() {
        let mut registry = create_test_registry();

        registry.assign_symbol("BTCUSDT", "binance", "tier3").unwrap();
        registry.upgrade_symbol("BTCUSDT", "tier2").unwrap();

        let assignment = registry.get_assignment("BTCUSDT").unwrap();
        assert_eq!(assignment.tier, "tier2");
    }

    #[test]
    fn test_unregister_collector() {
        let mut registry = create_test_registry();

        registry.assign_symbol("BTCUSDT", "binance", "tier1").unwrap();
        registry.unregister("binance-t1").unwrap();

        // 分配应该被清除
        assert!(registry.get_assignment("BTCUSDT").is_none());
    }

    #[test]
    fn test_no_available_collector() {
        let mut registry = CollectorRegistry::new();

        // 注册一个已满的 collector
        registry.register(CollectorInfo {
            id: "binance-t1".into(),
            exchange: "binance".into(),
            capacity: 1,
            current_load: 1,  // 已满
            tiers: vec!["tier1".into()],
        });

        // 尝试分配应该失败
        let result = registry.assign_symbol("BTCUSDT", "binance", "tier1");
        assert!(result.is_err());
    }
}