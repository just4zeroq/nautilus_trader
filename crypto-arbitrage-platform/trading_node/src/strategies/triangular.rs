use serde_yaml::Value;

pub struct TriangularStrategy {
    name: String,
    params: Value,
}

impl TriangularStrategy {
    pub fn new(params: &Value) -> anyhow::Result<Self> {
        Ok(Self {
            name: "TriangularStrategy".into(),
            params: params.clone(),
        })
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