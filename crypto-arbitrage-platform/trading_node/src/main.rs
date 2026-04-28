mod api;
mod config;
mod strategies;
mod risk;
mod ws_client;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Rust Trading Node...");

    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".into());
    let node_token = std::env::var("NODE_TOKEN").unwrap_or_else(|_| "dev-token".into());
    let node_id = "trader-rust-1";
    let node_type = "trader";

    // WebSocket client for heartbeat + commands
    let ws_client = ws_client::NodeWsClient::new(node_id, node_type, &backend_url, &node_token);
    if let Err(e) = ws_client.register("Rust Trader Engine", None).await {
        tracing::warn!("Node registration failed (backend may not be ready): {}", e);
    }

    // Risk manager (shared across strategies)
    let risk_manager = Arc::new(RwLock::new(risk::cross_exchange::CrossExchangeRiskManager::new()));

    // Strategy engine — manages running strategies
    let engine = Arc::new(RwLock::new(strategies::StrategyEngine::new(risk_manager.clone())));

    // API state
    let state = api::AppState {
        engine: engine.clone(),
        strategies: Arc::new(RwLock::new(vec![
            api::StrategyConfig {
                id: "cross-exchange-1".into(), name: "Cross Exchange Arbitrage".into(),
                strategy_type: "cross_exchange".into(), enabled: false,
                params: serde_json::json!({"spread_threshold": 10.0, "max_position": 1.0, "order_size": 0.1}),
            },
            api::StrategyConfig {
                id: "triangular-1".into(), name: "Triangular Arbitrage".into(),
                strategy_type: "triangular".into(), enabled: false,
                params: serde_json::json!({"spread_threshold": 0.001, "min_profit": 0.0005}),
            },
        ])),
    };

    let api_state = state.clone();
    tokio::spawn(async move {
        api::run_api_server(api_state).await;
    });

    tokio::spawn(async move {
        loop {
            if let Err(e) = ws_client.run().await {
                tracing::error!("WebSocket client error: {}", e);
            }
            tracing::info!("Reconnecting WebSocket in 5 seconds...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    tracing::info!("Rust Trading Node running. API on :8081");
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");
    Ok(())
}
