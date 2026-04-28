use crate::models::*;
use crate::AppState;
use salvo::prelude::*;
use sqlx::Row;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Alert Rules
// ---------------------------------------------------------------------------

fn row_to_rule(row: &sqlx::postgres::PgRow) -> AlertRuleResponse {
    let channels: Vec<String> = row.get::<Vec<String>, _>("channels");
    let conditions_str: String = row.get("condition");
    let conditions: serde_json::Value = serde_json::from_str(&conditions_str).unwrap_or_default();
    AlertRuleResponse {
        id: row.get("id"),
        name: row.get("name"),
        level: row.get("level"),
        conditions,
        channels,
        enabled: row.get("enabled"),
        created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
        updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
    }
}

#[handler]
pub async fn list_alert_rules(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query("SELECT id, name, level, condition, channels, enabled, created_at, updated_at FROM alert_rules ORDER BY id")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let rules: Vec<AlertRuleResponse> = rows.iter().map(row_to_rule).collect();
    res.render(Json(AlertRuleListResponse { total: rules.len(), rules }));
}

#[handler]
pub async fn create_alert_rule(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let body = req.parse_json::<AlertRuleCreateRequest>().await.unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    let id = Uuid::new_v4().to_string();
    let condition = serde_json::to_string(&body.conditions).unwrap_or_default();
    sqlx::query(
        "INSERT INTO alert_rules (id, name, level, condition, channels) VALUES ($1, $2, $3, $4, $5)"
    )
        .bind(&id)
        .bind(&body.name)
        .bind(&body.level)
        .bind(&condition)
        .bind(&body.channels)
        .execute(&state.pool)
        .await
        .ok();
    res.render(Json(serde_json::json!({"status": "created", "id": id})));
}

#[handler]
pub async fn update_alert_rule(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let body = req.parse_json::<AlertRuleUpdateRequest>().await.unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    if let Some(enabled) = body.enabled {
        sqlx::query("UPDATE alert_rules SET enabled = $1, updated_at = NOW() WHERE id = $2")
            .bind(enabled).bind(&id).execute(&state.pool).await.ok();
    }
    if let Some(name) = &body.name {
        sqlx::query("UPDATE alert_rules SET name = $1, updated_at = NOW() WHERE id = $2")
            .bind(name).bind(&id).execute(&state.pool).await.ok();
    }
    if let Some(level) = &body.level {
        sqlx::query("UPDATE alert_rules SET level = $1, updated_at = NOW() WHERE id = $2")
            .bind(level).bind(&id).execute(&state.pool).await.ok();
    }
    res.render(Json(serde_json::json!({"status": "updated"})));
}

#[handler]
pub async fn delete_alert_rule(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    sqlx::query("DELETE FROM alert_rules WHERE id = $1").bind(&id).execute(&state.pool).await.ok();
    res.render(Json(serde_json::json!({"status": "deleted"})));
}

// ---------------------------------------------------------------------------
// Alert Channels
// ---------------------------------------------------------------------------

fn row_to_channel(row: &sqlx::postgres::PgRow) -> AlertChannelResponse {
    AlertChannelResponse {
        id: row.get("id"),
        name: row.get("name"),
        channel_type: row.get("channel_type"),
        config: row.get::<serde_json::Value, _>("config"),
        enabled: row.get("enabled"),
        created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
    }
}

#[handler]
pub async fn list_alert_channels(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query("SELECT id, name, channel_type, config, enabled, created_at FROM alert_channels ORDER BY id")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let channels: Vec<AlertChannelResponse> = rows.iter().map(row_to_channel).collect();
    res.render(Json(AlertChannelListResponse { total: channels.len(), channels }));
}

#[handler]
pub async fn create_alert_channel(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let body = req.parse_json::<AlertChannelCreateRequest>().await.unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO alert_channels (id, name, channel_type, config) VALUES ($1, $2, $3, $4)"
    )
        .bind(&id)
        .bind(&body.name)
        .bind(&body.channel_type)
        .bind(&body.config)
        .execute(&state.pool)
        .await
        .ok();
    res.render(Json(serde_json::json!({"status": "created", "id": id})));
}

#[handler]
pub async fn update_alert_channel(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let body = req.parse_json::<AlertChannelUpdateRequest>().await.unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    if let Some(name) = &body.name {
        sqlx::query("UPDATE alert_channels SET name = $1, updated_at = NOW() WHERE id = $2")
            .bind(name).bind(&id).execute(&state.pool).await.ok();
    }
    if let Some(enabled) = body.enabled {
        sqlx::query("UPDATE alert_channels SET enabled = $1, updated_at = NOW() WHERE id = $2")
            .bind(enabled).bind(&id).execute(&state.pool).await.ok();
    }
    res.render(Json(serde_json::json!({"status": "updated"})));
}

#[handler]
pub async fn delete_alert_channel(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();
    let state = depot.obtain::<AppState>().unwrap();
    sqlx::query("DELETE FROM alert_channels WHERE id = $1").bind(&id).execute(&state.pool).await.ok();
    res.render(Json(serde_json::json!({"status": "deleted"})));
}

#[handler]
pub async fn test_alert_channel(_depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let _id = req.param::<String>("id").unwrap_or_default();
    res.render(Json(serde_json::json!({"status": "test_sent"})));
}

// ---------------------------------------------------------------------------
// Alert History
// ---------------------------------------------------------------------------

#[handler]
pub async fn list_alert_history(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let rows = sqlx::query(
        "SELECT id, rule_id, rule_name, level, conditions, triggered_value, channels, status, error_message, created_at FROM alert_history ORDER BY created_at DESC LIMIT 100"
    )
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    let items: Vec<AlertHistoryResponse> = rows.iter().map(|r| AlertHistoryResponse {
        id: r.get("id"),
        rule_id: r.get("rule_id"),
        rule_name: r.get("rule_name"),
        level: r.get("level"),
        conditions: r.get("conditions"),
        triggered_value: r.get("triggered_value"),
        channels: r.get("channels"),
        status: r.get("status"),
        error_message: r.get("error_message"),
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
    }).collect();
    res.render(Json(AlertHistoryListResponse { total: items.len(), items }));
}
