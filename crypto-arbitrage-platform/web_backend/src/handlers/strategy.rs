use crate::models::{StrategyResponse, StrategyStatusResponse};
use crate::AppState;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyListResponse {
    pub strategies: Vec<StrategyResponse>,
    pub total: usize,
}

fn row_to_strategy(row: &sqlx::postgres::PgRow) -> StrategyResponse {
    StrategyResponse {
        id: row.get("id"),
        name: row.get("name"),
        strategy_type: row.get("strategy_type"),
        enabled: row.get("enabled"),
        params: row.get::<serde_json::Value, _>("params"),
    }
}

#[handler]
pub async fn list_strategies(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query("SELECT id, name, strategy_type, enabled, params FROM strategies ORDER BY id")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let strategies: Vec<StrategyResponse> = rows.iter().map(row_to_strategy).collect();
    res.render(Json(StrategyListResponse { total: strategies.len(), strategies }));
}

#[handler]
pub async fn get_strategy(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    match sqlx::query("SELECT id, name, strategy_type, enabled, params FROM strategies WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(row)) => res.render(Json(row_to_strategy(&row))),
        _ => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({"error": "strategy not found"})));
        }
    }
}

#[handler]
pub async fn start_strategy(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    let result = sqlx::query("UPDATE strategies SET enabled = true, updated_at = NOW() WHERE id = $1")
        .bind(&id)
        .execute(&state.pool)
        .await;
    match result {
        Ok(r) if r.rows_affected() > 0 => res.render(Json(StrategyStatusResponse {
            strategy_id: id, status: "started".into(), message: Some("Strategy started".into()),
        })),
        _ => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({"error": "strategy not found"})));
        }
    }
}

#[handler]
pub async fn stop_strategy(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    let result = sqlx::query("UPDATE strategies SET enabled = false, updated_at = NOW() WHERE id = $1")
        .bind(&id)
        .execute(&state.pool)
        .await;
    match result {
        Ok(r) if r.rows_affected() > 0 => res.render(Json(StrategyStatusResponse {
            strategy_id: id, status: "stopped".into(), message: Some("Strategy stopped".into()),
        })),
        _ => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({"error": "strategy not found"})));
        }
    }
}

#[handler]
pub async fn update_strategy(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let body = req.parse_json::<serde_json::Value>().await.unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let enabled = body.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let params = body.get("params").cloned().unwrap_or(serde_json::json!({}));

    let result = sqlx::query(
        "UPDATE strategies SET name = $1, enabled = $2, params = $3, updated_at = NOW() WHERE id = $4"
    )
        .bind(name)
        .bind(enabled)
        .bind(&params)
        .bind(&id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => res.render(Json(StrategyStatusResponse {
            strategy_id: id, status: "updated".into(), message: Some("Strategy updated".into()),
        })),
        _ => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({"error": "strategy not found"})));
        }
    }
}
