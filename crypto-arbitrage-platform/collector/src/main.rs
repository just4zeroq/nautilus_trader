mod binance;
mod config;
mod okx;
mod redis_publisher;
mod registry;

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

    println!("Collector '{}' configured successfully!", config.collector_id);
    Ok(())
}