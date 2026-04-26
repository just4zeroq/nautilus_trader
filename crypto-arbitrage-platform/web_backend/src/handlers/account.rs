use crate::models::{AccountBalanceResponse, OrderResponse, PositionResponse};
use salvo::prelude::*;

#[handler]
pub async fn get_balance(res: &mut Response) {
    let balances = vec![
        AccountBalanceResponse {
            exchange: "binance".to_string(),
            asset: "USDT".to_string(),
            balance: 50000.0,
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        AccountBalanceResponse {
            exchange: "binance".to_string(),
            asset: "BTC".to_string(),
            balance: 1.5,
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        AccountBalanceResponse {
            exchange: "binance".to_string(),
            asset: "ETH".to_string(),
            balance: 25.0,
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        AccountBalanceResponse {
            exchange: "okx".to_string(),
            asset: "USDT".to_string(),
            balance: 50000.0,
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        AccountBalanceResponse {
            exchange: "okx".to_string(),
            asset: "BTC".to_string(),
            balance: 1.2,
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
    ];

    res.render(Json(balances));
}

#[handler]
pub async fn get_positions(res: &mut Response) {
    let positions = vec![
        PositionResponse {
            id: "1".to_string(),
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            side: "LONG".to_string(),
            quantity: 0.5,
            avg_price: 65000.0,
        },
        PositionResponse {
            id: "2".to_string(),
            symbol: "ETHUSDT".to_string(),
            exchange: "okx".to_string(),
            side: "LONG".to_string(),
            quantity: 5.0,
            avg_price: 3500.0,
        },
    ];

    res.render(Json(positions));
}

#[handler]
pub async fn get_orders(res: &mut Response) {
    let orders = vec![
        OrderResponse {
            id: "order-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            side: "BUY".to_string(),
            order_type: "LIMIT".to_string(),
            price: 64000.0,
            quantity: 0.1,
            status: "FILLED".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        },
        OrderResponse {
            id: "order-2".to_string(),
            symbol: "ETHUSDT".to_string(),
            exchange: "okx".to_string(),
            side: "SELL".to_string(),
            order_type: "LIMIT".to_string(),
            price: 3600.0,
            quantity: 2.0,
            status: "FILLED".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    ];

    res.render(Json(orders));
}
