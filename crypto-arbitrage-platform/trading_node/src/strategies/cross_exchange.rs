use serde_yaml::Value;

pub struct CrossExchangeStrategy {
    name: String,
    params: Value,
}

impl CrossExchangeStrategy {
    pub fn new(params: &Value) -> anyhow::Result<Self> {
        Ok(Self {
            name: "CrossExchangeStrategy".into(),
            params: params.clone(),
        })
    }
}

impl super::StrategyTrait for CrossExchangeStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn strategy_type(&self) -> &str {
        "cross_exchange"
    }
}