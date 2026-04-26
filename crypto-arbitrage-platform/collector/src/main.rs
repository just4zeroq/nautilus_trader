mod binance;
mod config;
mod okx;
mod redis_publisher;
mod registry;

use clap::Parser;
use config::{CollectorConfig, TierConfig};
use registry::CollectorRegistry;
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

    // 创建 TierConfig
    let tier_config = TierConfig {
        tier1_symbols: get_symbols_from_config(&config_content, "tier1_symbols"),
        tier2_symbols: get_symbols_from_config(&config_content, "tier2_symbols"),
        tier3_symbols: get_symbols_from_config(&config_content, "tier3_symbols"),
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

    // 保持进程运行
    tokio::signal::ctrl_c().await?;

    info!("Shutting down collector...");
    Ok(())
}

/// 从配置内容中提取 symbols 列表
fn get_symbols_from_config(content: &str, key: &str) -> Vec<String> {
    // 简单的 YAML 解析，直接提取 symbols
    // 实际使用时应该使用 serde_yaml 完整解析
    let mut result = Vec::new();

    // 这里简化处理，实际项目中应该完整解析 YAML
    let search_pattern = format!("{}:", key);
    if let Some(start_idx) = content.find(&search_pattern) {
        let line_start = content[..start_idx].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = content[start_idx..].find('\n').map(|i| start_idx + i).unwrap_or(content.len());
        let line = &content[line_start..line_end];

        // 提取 - 开头的行作为 symbols
        for l in line.lines().skip(1) {
            let trimmed = l.trim();
            if trimmed.starts_with('-') {
                let symbol = trimmed.trim_start_matches('-').trim().trim_matches('"').trim_matches('\'');
                if !symbol.is_empty() && !symbol.contains(':') {
                    result.push(symbol.to_string());
                }
            } else if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            } else if trimmed.contains(':') {
                break;
            }
        }
    }

    result
}
