mod config;
mod db;
mod handlers;
mod models;

use salvo::prelude::*;
use tracing::info;
use tracing_subscriber;

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
        );

    let addr = "0.0.0.0:8080";
    info!("Starting server on {}", addr);

    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
