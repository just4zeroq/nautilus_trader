mod config;

use config::TradingNodeConfig;
use nautilus_core::UUID4;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("==============================================");
    info!("Trading Node Configuration");
    info!("==============================================");

    // 加载配置
    let config: TradingNodeConfig = serde_yaml::from_str(&std::fs::read_to_string("config.yaml")?)?;

    info!("Trader ID: {}", config.trader_id);
    info!("Instance ID: {}", config.instance_id);
    info!("Redis: {}:{}", config.redis.host, config.redis.port);
    info!("External streams: {:?}", config.redis.external_streams);
    info!("Strategies: {}", config.strategies.len());
    for strategy in &config.strategies {
        info!("  - {} ({}) - enabled={}",
            strategy.id,
            strategy.strategy_type,
            strategy.enabled
        );
    }
    info!("==============================================");

    info!("Trading Node ready");

    // 保持运行
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Trading Node...");

    Ok(())
}