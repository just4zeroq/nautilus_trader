use crate::config::PhoneConfig;
use crate::handlers::{AlertHandler, AlertNotification};
use crate::config::AlertLevel;
use reqwest::blocking::Client;
use serde_json::json;

pub struct PhoneHandler {
    config: PhoneConfig,
    client: Client,
}

impl PhoneHandler {
    pub fn new(config: PhoneConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    fn build_request(&self, phone_numbers: &str, message: &str) -> serde_json::Value {
        json!({
            "accountSid": self.config.account,
            "token": self.config.token,
            "to": phone_numbers,
            "appId": "your_app_id",
            "templateId": "your_template_id",
            "datas": vec![message]
        })
    }
}

impl AlertHandler for PhoneHandler {
    fn name(&self) -> &str {
        "phone"
    }

    fn handle(&self, alert: &AlertNotification) -> anyhow::Result<()> {
        let phone_numbers = std::env::var("ALERT_PHONE_NUMBERS")
            .unwrap_or_else(|_| "13800138000".to_string());

        let level_prefix = match alert.level {
            AlertLevel::P0 => "紧急报警",
            AlertLevel::P1 => "严重告警",
            AlertLevel::P2 => "警告",
            AlertLevel::P3 => "通知",
        };

        let message = format!("{} {} - {}", level_prefix, alert.rule_name, alert.message);

        let payload = self.build_request(&phone_numbers, &message);

        let response = self.client
            .post(&self.config.api_url)
            .json(&payload)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("Phone API error: {} - {}", status, body);
        }

        tracing::info!("Phone alert sent: {}", alert.rule_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> PhoneHandler {
        PhoneHandler::new(PhoneConfig {
            provider: "容联云".to_string(),
            api_url: "https://your-api-url/voice".to_string(),
            account: "test_account".to_string(),
            token: "test_token".to_string(),
        })
    }

    #[test]
    fn test_build_request() {
        let handler = create_test_handler();
        let payload = handler.build_request("13800138000", "Test message");

        assert_eq!(payload["accountSid"], "test_account");
        assert_eq!(payload["to"], "13800138000");
        assert_eq!(payload["datas"].as_array().unwrap()[0], "Test message");
    }

    #[test]
    fn test_handler_name() {
        let handler = create_test_handler();
        assert_eq!(handler.name(), "phone");
    }
}
