use crate::models::SymbolResponse;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolListResponse {
    pub symbols: Vec<SymbolResponse>,
    pub total: usize,
}

#[handler]
pub async fn list_symbols(res: &mut Response) {
    let symbols = vec![
        SymbolResponse {
            id: "1".to_string(),
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            tier: 1,
            enabled: true,
        },
        SymbolResponse {
            id: "2".to_string(),
            symbol: "ETHUSDT".to_string(),
            exchange: "binance".to_string(),
            tier: 1,
            enabled: true,
        },
        SymbolResponse {
            id: "3".to_string(),
            symbol: "BTCUSDT".to_string(),
            exchange: "okx".to_string(),
            tier: 1,
            enabled: true,
        },
        SymbolResponse {
            id: "4".to_string(),
            symbol: "ADAUSDT".to_string(),
            exchange: "binance".to_string(),
            tier: 2,
            enabled: true,
        },
        SymbolResponse {
            id: "5".to_string(),
            symbol: "DOGEUSDT".to_string(),
            exchange: "okx".to_string(),
            tier: 3,
            enabled: false,
        },
    ];

    let response = SymbolListResponse {
        total: symbols.len(),
        symbols,
    };

    res.render(Json(response));
}

#[handler]
pub async fn get_symbol(req: &mut Request, res: &mut Response) {
    let _symbol = req.param::<String>("symbol").unwrap_or_default();

    let symbol = SymbolResponse {
        id: "1".to_string(),
        symbol: "BTCUSDT".to_string(),
        exchange: "binance".to_string(),
        tier: 1,
        enabled: true,
    };

    res.render(Json(symbol));
}

#[handler]
pub async fn create_symbol(req: &mut Request, res: &mut Response) {
    let _body = req.parse_json::<serde_json::Value>().await.unwrap_or_default();

    let symbol = SymbolResponse {
        id: "new".to_string(),
        symbol: "NEWUSDT".to_string(),
        exchange: "binance".to_string(),
        tier: 3,
        enabled: true,
    };

    res.render(Json(symbol));
}

#[handler]
pub async fn update_symbol_tier(req: &mut Request, res: &mut Response) {
    let _symbol = req.param::<String>("symbol").unwrap_or_default();
    let _body = req.parse_json::<serde_json::Value>().await.unwrap_or_default();

    let symbol = SymbolResponse {
        id: "1".to_string(),
        symbol: "BTCUSDT".to_string(),
        exchange: "binance".to_string(),
        tier: 1,
        enabled: true,
    };

    res.render(Json(symbol));
}

#[handler]
pub async fn delete_symbol(req: &mut Request, res: &mut Response) {
    let _symbol = req.param::<String>("symbol").unwrap_or_default();

    res.render(Json(serde_json::json!({
        "status": "deleted"
    })));
}
