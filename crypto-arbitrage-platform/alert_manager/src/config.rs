use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AlertConfig {
    pub alerts: Vec<AlertRule>,
    pub feishu: FeishuConfig,
    pub sms: Option<SmsConfig>,
    pub phone: Option<PhoneConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub level: AlertLevel,
    pub condition: String,
    pub channels: Vec<String>,
    pub phone_interval_secs: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum AlertLevel {
    P0,
    P1,
    P2,
    P3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeishuConfig {
    pub webhook_url: String,
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SmsConfig {
    pub provider: String,
    pub access_key: String,
    pub secret: String,
    pub sign_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PhoneConfig {
    pub provider: String,
    pub api_url: String,
    pub account: String,
    pub token: String,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            alerts: Vec::new(),
            feishu: FeishuConfig {
                webhook_url: String::new(),
                secret: None,
            },
            sms: None,
            phone: None,
        }
    }
}