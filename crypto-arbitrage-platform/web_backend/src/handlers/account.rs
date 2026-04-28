use crate::models::{AccountBalanceResponse, OrderResponse, PositionResponse, AccountResponse, AccountListResponse, ApiKeyConfigRequest};
use crate::AppState;
use salvo::prelude::*;
use sqlx::Row;

// ---------------------------------------------------------------------------
// Balance / Positions / Orders
// ---------------------------------------------------------------------------

#[handler]
pub async fn get_balance(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query("SELECT exchange, asset, balance, updated_at FROM accounts ORDER BY exchange, asset")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let balances: Vec<AccountBalanceResponse> = rows.iter().map(|r| AccountBalanceResponse {
        exchange: r.get("exchange"),
        asset: r.get("asset"),
        balance: r.get("balance"),
        updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
    }).collect();
    res.render(Json(balances));
}

#[handler]
pub async fn get_positions(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query("SELECT id, symbol, exchange, side, quantity, avg_price FROM positions ORDER BY symbol")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let positions: Vec<PositionResponse> = rows.iter().map(|r| PositionResponse {
        id: r.get("id"),
        symbol: r.get("symbol"),
        exchange: r.get("exchange"),
        side: r.get("side"),
        quantity: r.get("quantity"),
        avg_price: r.get("avg_price"),
    }).collect();
    res.render(Json(positions));
}

#[handler]
pub async fn get_orders(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query(
        "SELECT id, symbol, exchange, side, order_type, price, quantity, status, created_at FROM orders ORDER BY created_at DESC LIMIT 50"
    )
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let orders: Vec<OrderResponse> = rows.iter().map(|r| OrderResponse {
        id: r.get("id"),
        symbol: r.get("symbol"),
        exchange: r.get("exchange"),
        side: r.get("side"),
        order_type: r.get("order_type"),
        price: r.get("price"),
        quantity: r.get("quantity"),
        status: r.get("status"),
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
    }).collect();
    res.render(Json(orders));
}

// ---------------------------------------------------------------------------
// Exchange accounts (API key management)
// ---------------------------------------------------------------------------

#[handler]
pub async fn list_accounts(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query(
        "SELECT DISTINCT ON (exchange) exchange, label, api_key_encrypted FROM accounts ORDER BY exchange, updated_at DESC"
    )
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

    let mut accounts = Vec::new();
    for row in &rows {
        let exchange: String = row.get("exchange");
        let label: Option<String> = row.get("label");
        let api_key_encrypted: Option<String> = row.get("api_key_encrypted");
        let api_key_configured = api_key_encrypted.is_some();
        let api_key_mask = api_key_encrypted.as_ref().map(|k| {
            if k.len() > 4 { format!("****{}", &k[k.len()-4..]) } else { "****".into() }
        });

        let asset_rows = sqlx::query("SELECT exchange, asset, balance, updated_at FROM accounts WHERE exchange = $1")
            .bind(&exchange)
            .fetch_all(&state.pool)
            .await
            .unwrap_or_default();
        let assets: Vec<AccountBalanceResponse> = asset_rows.iter().map(|r| AccountBalanceResponse {
            exchange: r.get("exchange"),
            asset: r.get("asset"),
            balance: r.get("balance"),
            updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
        }).collect();

        let id = format!("acc-{}", exchange);
        accounts.push(AccountResponse { id, exchange, label, api_key_configured, api_key_mask, assets });
    }
    res.render(Json(AccountListResponse { total: accounts.len(), accounts }));
}

#[handler]
pub async fn configure_api_key(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let body = req.parse_json::<ApiKeyConfigRequest>().await.unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();

    // id format: "acc-{exchange}"
    let exchange = id.strip_prefix("acc-").unwrap_or(&id);

    let result = sqlx::query(
        "UPDATE accounts SET api_key_encrypted = $1, api_secret_encrypted = $2, api_passphrase_encrypted = $3 WHERE exchange = $4"
    )
        .bind(&body.api_key)
        .bind(&body.api_secret)
        .bind(&body.api_passphrase)
        .bind(exchange)
        .execute(&state.pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            let mask = if body.api_key.len() > 4 {
                format!("****{}", &body.api_key[body.api_key.len()-4..])
            } else {
                "****".into()
            };
            res.render(Json(serde_json::json!({"status": "configured", "api_key_mask": mask})));
        }
        _ => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({"error": "account not found"})));
        }
    }
}
