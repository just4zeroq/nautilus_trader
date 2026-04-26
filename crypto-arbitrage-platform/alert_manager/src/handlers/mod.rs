pub mod feishu;
pub mod sms;
pub mod phone;

use crate::config::{AlertConfig, AlertLevel};
use std::sync::Arc;

pub struct AlertManager {
    config: Arc<AlertConfig>,
    handlers: Vec<Box<dyn AlertHandler>>,
}

pub trait AlertHandler: Send + Sync {
    fn name(&self) -> &str;
    fn handle(&self, alert: &AlertNotification) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct AlertNotification {
    pub rule_name: String,
    pub level: AlertLevel,
    pub message: String,
    pub timestamp: i64,
}

impl AlertManager {
    pub fn new(config: AlertConfig) -> Self {
        let handlers = Self::create_handlers(&config);
        Self {
            config: Arc::new(config),
            handlers,
        }
    }

    fn create_handlers(config: &AlertConfig) -> Vec<Box<dyn AlertHandler>> {
        let mut handlers: Vec<Box<dyn AlertHandler>> = Vec::new();

        handlers.push(Box::new(feishu::FeishuHandler::new(config.feishu.clone())));

        if let Some(ref sms_config) = config.sms {
            handlers.push(Box::new(sms::SmsHandler::new(sms_config.clone())));
        }

        if let Some(ref phone_config) = config.phone {
            handlers.push(Box::new(phone::PhoneHandler::new(phone_config.clone())));
        }

        handlers
    }

    pub fn send_alert(&self, notification: &AlertNotification) -> anyhow::Result<()> {
        for handler in &self.handlers {
            if let Err(e) = handler.handle(notification) {
                tracing::error!("Failed to send alert via {}: {}", handler.name(), e);
            }
        }
        Ok(())
    }

    pub fn config(&self) -> &AlertConfig {
        &self.config
    }
}