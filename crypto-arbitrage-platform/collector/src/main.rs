mod binance;
mod config;
mod okx;
mod redis_publisher;
mod registry;

use crate::binance::BinanceCollector;
use crate::okx::OKXCollector;
use clap::Parser;
use config::CollectorConfig;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "collector")]
#[command(about = "Crypto arbitrage data collector")]
struct Args {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config_content = std::fs::read_to_string(&args.config)?;
    let config: CollectorConfig = serde_yaml::from_str(&config_content)?;

    info!("Starting collector: {}", config.collector_id);
    info!("Exchange: {}", config.exchange);
    info!("Tiers: {:?}", config.tiers);
    info!("Redis: {}:{}", config.redis.host, config.redis.port);

    // Tier config for Binance collector
    let tier_config = config::TierConfig {
        tier1_symbols: vec!["BTCUSDT".into(), "ETHUSDT".into()],
        tier2_symbols: vec!["ADAUSDT".into(), "DOGEUSDT".into()],
        tier3_symbols: vec!["SHIBUSDT".into()],
    };

    match config.exchange.as_str() {
        "binance" => {
            let collector = BinanceCollector::new(&config, &config.redis, tier_config).await?;
            info!("Binance collector created: {}", collector.collector_id());
            info!("Tier1 symbols: {:?}", collector.tier1_symbols());
            info!("Tier2 symbols: {:?}", collector.tier2_symbols());
            info!("Tier3 symbols: {:?}", collector.tier3_symbols());
        }
        "okx" => {
            let collector = OKXCollector::new(&config, &config.redis, tier_config).await?;
            info!("OKX collector created: {}", collector.collector_id());
            info!("Tier1 symbols: {:?}", collector.tier1_symbols());
        }
        _ => anyhow::bail!("Unknown exchange: {}", config.exchange),
    }

    println!("Collector '{}' configured successfully!", config.collector_id);
    Ok(())
}