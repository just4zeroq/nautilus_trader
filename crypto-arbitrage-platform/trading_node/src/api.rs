use salvo::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::strategies::StrategyEngine;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StrategyConfig {
    pub id: String,
    pub name: String,
    pub strategy_type: String,
    pub enabled: bool,
    pub params: serde_json::Value,
}

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<RwLock<StrategyEngine>>,
    pub strategies: Arc<RwLock<Vec<StrategyConfig>>>,
}

#[derive(Debug, serde::Serialize)]
pub struct StrategyListResponse {
    pub strategies: Vec<StrategyConfig>,
    pub total: usize,
}

#[handler]
pub async fn list_strategies(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let engine = state.engine.read().await;
    let active = engine.active.read().await;
    let configs = state.strategies.read().await;

    let strategies: Vec<StrategyConfig> = configs.iter().map(|c| StrategyConfig {
        id: c.id.clone(),
        name: c.name.clone(),
        strategy_type: c.strategy_type.clone(),
        enabled: active.contains(&c.id),
        params: c.params.clone(),
    }).collect();

    res.render(Json(StrategyListResponse {
        total: strategies.len(),
        strategies,
    }));
}

#[handler]
pub async fn update_strategies(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let new_strategies = req.parse_json::<Vec<StrategyConfig>>().await.unwrap_or_default();
    *state.strategies.write().await = new_strategies;
    res.render(Json(serde_json::json!({"status": "updated"})));
}

#[handler]
pub async fn start_strategy(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    let engine = state.engine.read().await;
    let ok = engine.start(&id).await;
    if ok {
        res.render(Json(serde_json::json!({"status": "started", "strategy_id": id})));
    } else {
        res.render(Json(serde_json::json!({"status": "already_running", "strategy_id": id})));
    }
}

#[handler]
pub async fn stop_strategy(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    let engine = state.engine.read().await;
    let ok = engine.stop(&id).await;
    if ok {
        res.render(Json(serde_json::json!({"status": "stopped", "strategy_id": id})));
    } else {
        res.render(Json(serde_json::json!({"status": "not_running", "strategy_id": id})));
    }
}

#[handler]
pub async fn health_check(res: &mut Response) {
    res.render(Json(serde_json::json!({"status": "healthy", "service": "trading_node"})));
}

pub async fn run_api_server(state: AppState) {
    let router = Router::new()
        .push(Router::with_path("/health").get(health_check))
        .push(
            Router::with_path("/api/v1/strategies")
                .get(list_strategies)
                .put(update_strategies)
                .push(Router::with_path("/<id>/start").post(start_strategy))
                .push(Router::with_path("/<id>/stop").post(stop_strategy)),
        )
        .hoop(affix_state::inject(state));

    let acceptor = TcpListener::new("0.0.0.0:8081").bind().await;
    Server::new(acceptor).serve(router).await;
}
