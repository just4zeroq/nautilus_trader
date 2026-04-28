use crate::models::SymbolResponse;
use crate::AppState;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolListResponse {
    pub symbols: Vec<SymbolResponse>,
    pub total: usize,
}

fn row_to_symbol(row: &sqlx::postgres::PgRow) -> SymbolResponse {
    SymbolResponse {
        id: row.get("id"),
        symbol: row.get("symbol"),
        exchange: row.get("exchange"),
        tier: row.get("tier"),
        enabled: row.get("enabled"),
    }
}

#[handler]
pub async fn list_symbols(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query("SELECT id, symbol, exchange, tier, enabled FROM symbols ORDER BY exchange, symbol")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let symbols: Vec<SymbolResponse> = rows.iter().map(row_to_symbol).collect();
    res.render(Json(SymbolListResponse { total: symbols.len(), symbols }));
}

#[handler]
pub async fn get_symbol(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let symbol = req.param::<String>("symbol").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    match sqlx::query("SELECT id, symbol, exchange, tier, enabled FROM symbols WHERE symbol = $1")
        .bind(&symbol)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(row)) => res.render(Json(row_to_symbol(&row))),
        _ => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({"error": "symbol not found"})));
        }
    }
}

#[handler]
pub async fn update_symbol_tier(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let symbol = req.param::<String>("symbol").unwrap_or_default();
    let body = req.parse_json::<serde_json::Value>().await.unwrap_or_default();
    let tier = body.get("tier").and_then(|v| v.as_i64()).unwrap_or(3) as i32;
    let state = depot.obtain::<AppState>().unwrap();

    let result = sqlx::query("UPDATE symbols SET tier = $1, updated_at = NOW() WHERE symbol = $2")
        .bind(tier)
        .bind(&symbol)
        .execute(&state.pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            let row = sqlx::query("SELECT id, symbol, exchange, tier, enabled FROM symbols WHERE symbol = $1")
                .bind(&symbol)
                .fetch_one(&state.pool)
                .await
                .unwrap();
            res.render(Json(row_to_symbol(&row)));
        }
        _ => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({"error": "symbol not found"})));
        }
    }
}

#[handler]
pub async fn delete_symbol(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let symbol = req.param::<String>("symbol").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    sqlx::query("DELETE FROM symbols WHERE symbol = $1")
        .bind(&symbol)
        .execute(&state.pool)
        .await
        .ok();
    res.render(Json(serde_json::json!({"status": "deleted"})));
}
