use super::{AlertHandler, AlertNotification};
use crate::config::PhoneConfig;

pub struct PhoneHandler {
    config: PhoneConfig,
}

impl PhoneHandler {
    pub fn new(config: PhoneConfig) -> Self {
        Self { config }
    }
}

impl AlertHandler for PhoneHandler {
    fn name(&self) -> &str {
        "phone"
    }

    fn handle(&self, alert: &AlertNotification) -> anyhow::Result<()> {
        tracing::info!("[Phone] Sending alert: {} - {}", alert.rule_name, alert.message);
        // TODO: Implement actual Phone API call
        Ok(())
    }
}