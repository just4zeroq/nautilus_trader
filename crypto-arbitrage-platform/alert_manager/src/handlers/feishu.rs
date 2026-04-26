use super::{AlertHandler, AlertNotification};
use crate::config::FeishuConfig;

pub struct FeishuHandler {
    config: FeishuConfig,
}

impl FeishuHandler {
    pub fn new(config: FeishuConfig) -> Self {
        Self { config }
    }
}

impl AlertHandler for FeishuHandler {
    fn name(&self) -> &str {
        "feishu"
    }

    fn handle(&self, alert: &AlertNotification) -> anyhow::Result<()> {
        tracing::info!("[Feishu] Sending alert: {} - {}", alert.rule_name, alert.message);
        // TODO: Implement actual Feishu webhook call
        Ok(())
    }
}