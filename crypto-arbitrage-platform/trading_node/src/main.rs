mod config;
mod strategies;

use config::TradingNodeConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("==============================================");
    tracing::info!("Trading Node Configuration");
    tracing::info!("==============================================");

    // 加载配置
    let config: TradingNodeConfig = serde_yaml::from_str(&std::fs::read_to_string("config.yaml")?)?;

    tracing::info!("Trader ID: {}", config.trader_id);
    tracing::info!("Instance ID: {}", config.instance_id);
    tracing::info!("Redis: {}:{}", config.redis.host, config.redis.port);
    tracing::info!("External streams: {:?}", config.redis.external_streams);
    tracing::info!("Strategies: {}", config.strategies.len());
    for strategy in &config.strategies {
        tracing::info!("  - {} ({}) - enabled={}",
            strategy.id,
            strategy.strategy_type,
            strategy.enabled
        );
    }
    tracing::info!("==============================================");

    tracing::info!("Trading Node ready");

    // 保持运行
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down Trading Node...");

    Ok(())
}