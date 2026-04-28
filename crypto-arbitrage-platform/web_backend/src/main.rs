mod config;
mod db;
mod handlers;
mod models;

use salvo::prelude::*;
use sqlx::PgPool;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub node_state: handlers::node::NodeState,
}

#[handler]
async fn hello(res: &mut Response) {
    res.render(Text::Plain("Crypto Arbitrage Platform API v1.0"));
}

#[handler]
async fn health(res: &mut Response) {
    res.render(Json(serde_json::json!({
        "status": "healthy",
        "service": "web_backend"
    })));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Load config from environment
    let _ = dotenvy::dotenv();
    let cfg = config::Config::from_env()?;

    // Initialize DB pool
    let pool = db::create_pool(&cfg.database.connection_string()).await?;
    info!("Database connected");

    // Initialize node state from Redis heartbeat stream
    let node_state = handlers::node::init_state();
    let consumer_state = node_state.clone();
    tokio::spawn(async move {
        crate::handlers::node::start_node_consumer(consumer_state).await;
    });
    crate::handlers::node::start_stale_checker(node_state.clone());

    // Initialize token map for WebSocket auth
    handlers::node_ws::init_token_map();

    // Seed known nodes
    {
        let mut nodes = node_state.write().await;
        let mut seed = |id: &str, name: &str, node_type: &str, exchange: Option<&str>| {
            use crate::models::NodeResponse;
            nodes.insert(id.to_string(), NodeResponse {
                id: id.to_string(),
                name: name.to_string(),
                node_type: node_type.to_string(),
                exchange: exchange.map(String::from),
                status: "offline".to_string(),
                last_heartbeat: None,
                latency: None,
                version: None,
                token: None,
                ws_connected: false,
                api_port: None,
                ip_address: None,
            });
        };
        seed("collector-binance-1", "Binance Collector 1", "collector", Some("binance"));
        seed("collector-binance-2", "Binance Collector 2", "collector", Some("binance"));
        seed("collector-okx-1", "OKX Collector 1", "collector", Some("okx"));
        seed("trader-rust-1", "Rust Trader Engine", "trader", None);
        seed("trader-python-1", "Python Trader Engine", "trader", None);
        seed("alert-manager-1", "Alert Manager", "alert", None);
        seed("redis-master", "Redis Cache", "redis", None);
    }

    let app_state = AppState {
        pool,
        node_state: node_state.clone(),
    };

    let router = Router::new()
        .get(hello)
        .push(Router::with_path("/health").get(health))
        .push(
            Router::with_path("/api/v1/strategies")
                .get(handlers::strategy::list_strategies)
                .push(Router::with_path("/<id>").get(handlers::strategy::get_strategy))
                .push(Router::with_path("/<id>/start").post(handlers::strategy::start_strategy))
                .push(Router::with_path("/<id>/stop").post(handlers::strategy::stop_strategy))
                .push(Router::with_path("/<id>").put(handlers::strategy::update_strategy))
        )
        .push(
            Router::with_path("/api/v1/symbols")
                .get(handlers::symbol::list_symbols)
                .push(Router::with_path("/<symbol>").get(handlers::symbol::get_symbol))
                .push(Router::with_path("/<symbol>/tier").put(handlers::symbol::update_symbol_tier))
                .push(Router::with_path("/<symbol>").delete(handlers::symbol::delete_symbol))
        )
        .push(
            Router::with_path("/api/v1/account")
                .get(handlers::account::get_balance)
                .push(Router::with_path("/positions").get(handlers::account::get_positions))
                .push(Router::with_path("/orders").get(handlers::account::get_orders))
        )
        .push(
            Router::with_path("/api/v1/nodes")
                .get(handlers::node::list_nodes)
                .push(Router::with_path("/heartbeat").post(handlers::node::heartbeat))
                .push(Router::with_path("/register").post(handlers::node::register_node))
        )
        .push(Router::with_path("/ws/node").get(handlers::node_ws::handle_node_ws))
        .push(
            Router::with_path("/api/v1/blacklist")
                .get(handlers::blacklist::list_blacklist)
                .post(handlers::blacklist::create_blacklist_item)
                .push(Router::with_path("/<id>").delete(handlers::blacklist::delete_blacklist_item))
        )
        .push(
            Router::with_path("/api/v1/alerts/rules")
                .get(handlers::alert::list_alert_rules)
                .post(handlers::alert::create_alert_rule)
                .push(Router::with_path("/<id>")
                    .put(handlers::alert::update_alert_rule)
                    .delete(handlers::alert::delete_alert_rule))
        )
        .push(
            Router::with_path("/api/v1/alerts/channels")
                .get(handlers::alert::list_alert_channels)
                .post(handlers::alert::create_alert_channel)
                .push(Router::with_path("/<id>")
                    .put(handlers::alert::update_alert_channel)
                    .delete(handlers::alert::delete_alert_channel)
                    .push(Router::with_path("/test").post(handlers::alert::test_alert_channel)))
        )
        .push(
            Router::with_path("/api/v1/alerts/history")
                .get(handlers::alert::list_alert_history)
        )
        .push(
            Router::with_path("/api/v1/accounts")
                .get(handlers::account::list_accounts)
                .push(Router::with_path("/<id>/api-key").post(handlers::account::configure_api_key))
        )
        .hoop(affix_state::inject(app_state));

    let addr = format!("{}:{}", cfg.app.host, cfg.app.port);
    info!("Starting server on {}", addr);

    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
