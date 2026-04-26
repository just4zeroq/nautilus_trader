use salvo::prelude::*;

#[handler]
pub async fn ws_handler(res: &mut Response) {
    res.render(Text::Plain("WebSocket endpoint - use ws://host/ws/{channel}"));
}
