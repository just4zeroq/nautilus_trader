use crate::config::FeishuConfig;
use crate::handlers::{AlertHandler, AlertNotification};
use crate::config::AlertLevel;
use reqwest::blocking::Client;
use serde_json::json;

pub struct FeishuHandler {
    config: FeishuConfig,
    client: Client,
}

impl FeishuHandler {
    pub fn new(config: FeishuConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    fn build_message(&self, alert: &AlertNotification) -> serde_json::Value {
        let level_str = match alert.level {
            AlertLevel::P0 => ("🔴 紧急", "red"),
            AlertLevel::P1 => ("🟠 严重", "orange"),
            AlertLevel::P2 => ("🟡 警告", "yellow"),
            AlertLevel::P3 => ("🔵 信息", "blue"),
        };

        let timestamp = chrono::DateTime::from_timestamp(alert.timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "未知时间".to_string());

        json!({
            "msg_type": "interactive",
            "card": {
                "header": {
                    "title": {
                        "tag": "plain_text",
                        "content": format!("{} {}", level_str.0, alert.rule_name)
                    },
                    "template": level_str.1
                },
                "elements": [
                    {
                        "tag": "div",
                        "text": {
                            "tag": "lark_md",
                            "content": alert.message.clone()
                        }
                    },
                    {
                        "tag": "hr"
                    },
                    {
                        "tag": "note",
                        "elements": [
                            {
                                "tag": "plain_text",
                                "content": format!("时间: {}", timestamp)
                            }
                        ]
                    }
                ]
            }
        })
    }
}

impl AlertHandler for FeishuHandler {
    fn name(&self) -> &str {
        "feishu"
    }

    fn handle(&self, alert: &AlertNotification) -> anyhow::Result<()> {
        let message = self.build_message(alert);

        let response = self.client
            .post(&self.config.webhook_url)
            .json(&message)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("Feishu API returned error: {} - {}", status, body);
        }

        tracing::info!("Feishu alert sent: {}", alert.rule_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> FeishuHandler {
        FeishuHandler::new(FeishuConfig {
            webhook_url: "https://open.feishu.cn/open-apis/bot/v2/hook/test".to_string(),
            secret: None,
        })
    }

    #[test]
    fn test_build_message_p0() {
        let handler = create_test_handler();
        let alert = AlertNotification {
            rule_name: "账户亏损超限".to_string(),
            level: AlertLevel::P0,
            message: "日亏损达到 $1500".to_string(),
            timestamp: 1714166400, // 2024-04-27 00:00:00
        };

        let msg = handler.build_message(&alert);

        // 验证消息结构
        assert_eq!(msg["msg_type"], "interactive");
        assert_eq!(msg["card"]["header"]["template"], "red");
        assert!(msg["card"]["header"]["title"]["content"].as_str().unwrap().contains("账户亏损超限"));
    }

    #[test]
    fn test_handler_name() {
        let handler = create_test_handler();
        assert_eq!(handler.name(), "feishu");
    }
}