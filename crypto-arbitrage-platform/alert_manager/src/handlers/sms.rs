use crate::config::SmsConfig;
use crate::handlers::{AlertHandler, AlertNotification};
use crate::config::AlertLevel;
use base64::Engine;

pub struct SmsHandler {
    config: SmsConfig,
}

impl SmsHandler {
    pub fn new(config: SmsConfig) -> Self {
        Self { config }
    }

    fn build_request_params(&self, phone_numbers: &str, message: &str) -> Vec<(String, String)> {
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let mut params = vec![
            ("AccessKeyId".to_string(), self.config.access_key.clone()),
            ("Action".to_string(), "SendSms".to_string()),
            ("Format".to_string(), "JSON".to_string()),
            ("PhoneNumbers".to_string(), phone_numbers.to_string()),
            ("SignName".to_string(), self.config.sign_name.clone()),
            ("TemplateCode".to_string(), "SMS_xxx".to_string()),
            ("TemplateParam".to_string(), format!("{{\"message\":\"{}\"}}", message)),
            ("Timestamp".to_string(), timestamp),
            ("Version".to_string(), "2017-05-25".to_string()),
        ];

        params
    }

    fn sign(&self, params: &[(String, String)]) -> String {
        // 阿里云签名算法
        let mut sorted: Vec<_> = params.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));

        let query = sorted
            .iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let string_to_sign = format!("GET&%2F&{}", percent_encode(&query));
        let key = format!("{}&", self.config.secret);

        // HMAC-SHA1
        use hmac::{Hmac, Mac};
        type HmacSha1 = Hmac<sha1::Sha1>;
        let mut mac = HmacSha1::new_from_slice(key.as_bytes()).unwrap();
        mac.update(string_to_sign.as_bytes());
        let result = mac.finalize().into_bytes();

        base64::engine::general_purpose::STANDARD.encode(result)
    }
}

fn percent_encode(s: &str) -> String {
    let mut encoded = String::new();
    for byte in s.bytes() {
        if byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.' || byte == b'~' {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{:02X}", byte));
        }
    }
    encoded
}

impl AlertHandler for SmsHandler {
    fn name(&self) -> &str {
        "sms"
    }

    fn handle(&self, alert: &AlertNotification) -> anyhow::Result<()> {
        let phone_numbers = std::env::var("ALERT_PHONE_NUMBERS")
            .unwrap_or_else(|_| "13800138000".to_string());

        let level_prefix = match alert.level {
            AlertLevel::P0 => "[紧急]",
            AlertLevel::P1 => "[严重]",
            AlertLevel::P2 => "[警告]",
            AlertLevel::P3 => "[信息]",
        };

        let message = format!("{} {} - {}", level_prefix, alert.rule_name, alert.message);

        let params = self.build_request_params(&phone_numbers, &message);
        let signature = self.sign(&params);

        let mut all_params = params.clone();
        all_params.push(("Signature".to_string(), signature));

        let query_string = all_params
            .iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("https://dysmsapi.aliyuncs.com/?{}", query_string);

        // 使用 blocking reqwest
        let response = reqwest::blocking::get(&url)?;

        if !response.status().is_success() {
            anyhow::bail!("SMS API error: {}", response.status());
        }

        tracing::info!("SMS alert sent: {}", alert.rule_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> SmsHandler {
        SmsHandler::new(SmsConfig {
            provider: "aliyun".to_string(),
            access_key: "test_key".to_string(),
            secret: "test_secret&".to_string(),
            sign_name: "CryptoTrader".to_string(),
        })
    }

    #[test]
    fn test_build_params() {
        let handler = create_test_handler();
        let params = handler.build_request_params("13800138000", "Test message");

        assert!(params.iter().any(|(k, v)| k == "PhoneNumbers" && v == "13800138000"));
        assert!(params.iter().any(|(k, v)| k == "SignName" && v == "CryptoTrader"));
    }

    #[test]
    fn test_handler_name() {
        let handler = create_test_handler();
        assert_eq!(handler.name(), "sms");
    }

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("ABC"), "ABC");
        assert_eq!(percent_encode("BTC/USDT"), "BTC%2FUSDT");
        assert_eq!(percent_encode("中文"), "%E4%B8%AD%E6%96%87");
    }
}
