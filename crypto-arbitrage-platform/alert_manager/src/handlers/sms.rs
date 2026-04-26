use super::{AlertHandler, AlertNotification};
use crate::config::SmsConfig;

pub struct SmsHandler {
    config: SmsConfig,
}

impl SmsHandler {
    pub fn new(config: SmsConfig) -> Self {
        Self { config }
    }
}

impl AlertHandler for SmsHandler {
    fn name(&self) -> &str {
        "sms"
    }

    fn handle(&self, alert: &AlertNotification) -> anyhow::Result<()> {
        tracing::info!("[SMS] Sending alert: {} - {}", alert.rule_name, alert.message);
        // TODO: Implement actual SMS API call
        Ok(())
    }
}