mod binance;
mod config;
mod okx;
mod redis_publisher;
mod ws_client;

use clap::Parser;
use config::{CollectorConfig, TierConfig};
use std::time::Duration;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "collector")]
#[command(about = "Crypto arbitrage data collector")]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 解析命令行参数
    let args = Args::parse();

    // 读取配置文件
    let config_content = std::fs::read_to_string(&args.config)?;
    let config: CollectorConfig = serde_yaml::from_str(&config_content)?;

    info!("==============================================");
    info!("Collector Configuration");
    info!("==============================================");
    info!("Collector ID: {}", config.collector_id);
    info!("Exchange: {}", config.exchange);
    info!("Tiers: {:?}", config.tiers);
    info!("Redis: {}:{}", config.redis.host, config.redis.port);
    info!("Streams prefix: {}", config.redis.streams_prefix);
    info!("==============================================");

    // Parse tier config using serde_yaml (already in config_content)
    #[derive(serde::Deserialize)]
    struct TierConfigFile {
        tier1_symbols: Vec<String>,
        tier2_symbols: Vec<String>,
        tier3_symbols: Vec<String>,
    }
    let tier_file: TierConfigFile = serde_yaml::from_str(&config_content)?;
    let tier_config = TierConfig {
        tier1_symbols: tier_file.tier1_symbols,
        tier2_symbols: tier_file.tier2_symbols,
        tier3_symbols: tier_file.tier3_symbols,
    };

    // 根据交易所创建 Collector
    match config.exchange.as_str() {
        "binance" => {
            let collector = binance::BinanceCollector::new(&config, &config.redis, tier_config).await?;
            info!("Binance collector '{}' created successfully", collector.collector_id());
            info!("Tier1 symbols ({}): {:?}", collector.tier1_symbols().len(), collector.tier1_symbols());
            info!("Tier2 symbols ({}): {:?}", collector.tier2_symbols().len(), collector.tier2_symbols());
            info!("Tier3 symbols ({}): ...", collector.tier3_symbols().len());
        }
        "okx" => {
            let collector = okx::OKXCollector::new(&config, &config.redis, tier_config).await?;
            info!("OKX collector '{}' created successfully", collector.collector_id());
            info!("Tier1 symbols ({}): {:?}", collector.tier1_symbols().len(), collector.tier1_symbols());
            info!("Tier2 symbols ({}): {:?}", collector.tier2_symbols().len(), collector.tier2_symbols());
            info!("Tier3 symbols ({}): ...", collector.tier3_symbols().len());
        }
        _ => anyhow::bail!("Unknown exchange: {}. Supported: binance, okx", config.exchange),
    }

    info!("==============================================");
    info!("Collector ready for data collection");
    info!("==============================================");

    // Register with backend via REST + start WebSocket heartbeat client
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".into());
    let node_token = std::env::var("NODE_TOKEN").unwrap_or_else(|_| "dev-token".into());

    let collector_id = config.collector_id.clone();
    let exchange_name = config.exchange.clone();
    let node_name = format!("{} Collector", &exchange_name);

    let ws_client = ws_client::NodeWsClient::new(&collector_id, "collector", &backend_url, &node_token);
    if let Err(e) = ws_client.register(&node_name, Some(&exchange_name)).await {
        tracing::warn!("Node registration failed (backend may not be ready): {}", e);
    }

    tokio::spawn(async move {
        loop {
            if let Err(e) = ws_client.run().await {
                tracing::error!("WebSocket client error: {}", e);
            }
            tracing::info!("Reconnecting WebSocket in 5 seconds...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // 保持进程运行
    tokio::signal::ctrl_c().await?;

    info!("Shutting down collector...");
    Ok(())
}

