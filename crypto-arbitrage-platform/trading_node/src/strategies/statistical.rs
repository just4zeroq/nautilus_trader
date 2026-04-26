use serde_yaml::Value;

pub struct StatisticalStrategy {
    name: String,
    params: Value,
}

impl StatisticalStrategy {
    pub fn new(params: &Value) -> anyhow::Result<Self> {
        Ok(Self {
            name: "StatisticalStrategy".into(),
            params: params.clone(),
        })
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