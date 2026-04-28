use crate::models::{BlacklistListResponse, BlacklistItemResponse, BlacklistCreateRequest};
use crate::AppState;
use salvo::prelude::*;
use sqlx::Row;
use uuid::Uuid;

fn row_to_item(row: &sqlx::postgres::PgRow) -> BlacklistItemResponse {
    BlacklistItemResponse {
        id: row.get("id"),
        symbol: row.get("symbol"),
        exchange: row.get("exchange"),
        reason: row.get("reason"),
        created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
        created_by: row.get("created_by"),
    }
}

#[handler]
pub async fn list_blacklist(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query("SELECT id, symbol, exchange, reason, created_at, created_by FROM symbol_blacklist ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let items: Vec<BlacklistItemResponse> = rows.iter().map(row_to_item).collect();
    res.render(Json(BlacklistListResponse { total: items.len(), items }));
}

#[handler]
pub async fn create_blacklist_item(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let body = req.parse_json::<BlacklistCreateRequest>().await.unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO symbol_blacklist (id, symbol, exchange, reason) VALUES ($1, $2, $3, $4)"
    )
        .bind(&id)
        .bind(&body.symbol)
        .bind(&body.exchange)
        .bind(&body.reason)
        .execute(&state.pool)
        .await
        .ok();
    res.render(Json(serde_json::json!({"status": "created", "id": id})));
}

#[handler]
pub async fn delete_blacklist_item(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    sqlx::query("DELETE FROM symbol_blacklist WHERE id = $1")
        .bind(&id)
        .execute(&state.pool)
        .await
        .ok();
    res.render(Json(serde_json::json!({"status": "deleted"})));
}
