mod config;
mod handlers;

use config::AlertConfig;
use config::AlertLevel;
use handlers::{AlertManager, AlertNotification};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Alert Manager starting...");

    // 加载配置
    let config: AlertConfig = serde_yaml::from_str(&std::fs::read_to_string("config.yaml")?)?;

    tracing::info!("Loaded {} alert rules", config.alerts.len());

    let manager = Arc::new(AlertManager::new(config));

    // 示例：发送测试告警
    let test_alert = AlertNotification {
        rule_name: "Test Alert".into(),
        level: AlertLevel::P2,
        message: "This is a test alert".into(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    manager.send_alert(&test_alert)?;

    tracing::info!("Alert Manager ready");
    tokio::signal::ctrl_c().await?;

    Ok(())
}