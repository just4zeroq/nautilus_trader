from fastapi import FastAPI
from pydantic import BaseModel
import uvicorn
import logging
from typing import Optional

logger = logging.getLogger(__name__)

app = FastAPI(title="Python Trading Node")


class StrategyConfig(BaseModel):
    id: str
    name: str
    strategy_type: str
    enabled: bool = False
    params: dict = {}


strategies: list[StrategyConfig] = [
    StrategyConfig(id="cross-exchange-1", name="Cross Exchange Arbitrage", strategy_type="cross_exchange", enabled=False, params={"spread_threshold": 10.0}),
    StrategyConfig(id="triangular-1", name="Triangular Arbitrage", strategy_type="triangular", enabled=False, params={"spread_threshold": 0.001}),
]


@app.get("/health")
async def health():
    return {"status": "healthy", "service": "python_trading_node"}


@app.get("/api/v1/strategies")
async def list_strategies():
    return {"strategies": [s.model_dump() for s in strategies], "total": len(strategies)}


@app.put("/api/v1/strategies")
async def update_strategies(new_strategies: list[StrategyConfig]):
    strategies.clear()
    strategies.extend(new_strategies)
    return {"status": "updated", "total": len(strategies)}


async def run_api_server(host: str = "0.0.0.0", port: int = 8082):
    config = uvicorn.Config(app, host=host, port=port, log_level="info")
    server = uvicorn.Server(config)
    await server.serve()
