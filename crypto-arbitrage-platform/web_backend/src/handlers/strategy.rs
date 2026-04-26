use crate::models::{StrategyResponse, StrategyStatusResponse};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyListResponse {
    pub strategies: Vec<StrategyResponse>,
    pub total: usize,
}

#[handler]
pub async fn list_strategies(res: &mut Response) {
    let strategies = vec![
        StrategyResponse {
            id: "cross-exchange-1".to_string(),
            name: "Cross Exchange Arbitrage".to_string(),
            strategy_type: "cross_exchange".to_string(),
            enabled: true,
            params: serde_json::json!({
                "spread_threshold": 10.0,
                "max_position": 1.0,
                "order_size": 0.1
            }),
        },
        StrategyResponse {
            id: "triangular-1".to_string(),
            name: "Triangular Arbitrage".to_string(),
            strategy_type: "triangular".to_string(),
            enabled: false,
            params: serde_json::json!({
                "spread_threshold": 0.001,
                "min_profit": 0.0005
            }),
        },
        StrategyResponse {
            id: "statistical-1".to_string(),
            name: "Statistical Arbitrage".to_string(),
            strategy_type: "statistical".to_string(),
            enabled: false,
            params: serde_json::json!({
                "lookback_period": 100,
                "entry_threshold": 2.0,
                "exit_threshold": 0.5
            }),
        },
    ];

    let response = StrategyListResponse {
        total: strategies.len(),
        strategies,
    };

    res.render(Json(response));
}

#[handler]
pub async fn get_strategy(req: &mut Request, res: &mut Response) {
    let strategy_id = req.param::<String>("id").unwrap_or_default();

    let strategy = StrategyResponse {
        id: strategy_id.clone(),
        name: "Strategy".to_string(),
        strategy_type: "cross_exchange".to_string(),
        enabled: true,
        params: serde_json::json!({}),
    };

    res.render(Json(strategy));
}

#[handler]
pub async fn start_strategy(req: &mut Request, res: &mut Response) {
    let strategy_id = req.param::<String>("id").unwrap_or_default();

    let response = StrategyStatusResponse {
        strategy_id,
        status: "started".to_string(),
        message: Some("Strategy started successfully".to_string()),
    };

    res.render(Json(response));
}

#[handler]
pub async fn stop_strategy(req: &mut Request, res: &mut Response) {
    let strategy_id = req.param::<String>("id").unwrap_or_default();

    let response = StrategyStatusResponse {
        strategy_id,
        status: "stopped".to_string(),
        message: Some("Strategy stopped successfully".to_string()),
    };

    res.render(Json(response));
}

#[handler]
pub async fn update_strategy(req: &mut Request, res: &mut Response) {
    let strategy_id = req.param::<String>("id").unwrap_or_default();
    let _body = req.parse_json::<serde_json::Value>().await.unwrap_or_default();

    let response = StrategyStatusResponse {
        strategy_id,
        status: "updated".to_string(),
        message: Some("Strategy updated successfully".to_string()),
    };

    res.render(Json(response));
}
