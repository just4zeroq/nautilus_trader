mod config;
mod handlers;

use clap::Parser;
use config::AlertConfig;
use config::AlertLevel;
use handlers::{AlertManager, AlertNotification};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "alert_manager")]
#[command(about = "Crypto arbitrage alert manager")]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();
    tracing::info!("Alert Manager starting, config: {}", args.config);

    let config: AlertConfig = serde_yaml::from_str(&std::fs::read_to_string(&args.config)?)?;
    tracing::info!("Loaded {} alert rules", config.alerts.len());

    let manager = Arc::new(AlertManager::new(config));

    let test_alert = AlertNotification {
        rule_name: "Test Alert".into(),
        level: AlertLevel::P2,
        message: "Alert Manager started successfully".into(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    manager.send_alert(&test_alert)?;

    tracing::info!("Alert Manager ready");
    tokio::signal::ctrl_c().await?;

    Ok(())
}
