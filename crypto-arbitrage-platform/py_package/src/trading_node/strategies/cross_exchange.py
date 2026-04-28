from src.trading_node.strategies.base import BaseStrategy, Signal
from typing import Optional


class CrossExchangeStrategy(BaseStrategy):
    def __init__(self):
        super().__init__("cross-exchange-1", "Cross Exchange Arbitrage", "cross_exchange")
        self.prices: dict[str, dict[str, float]] = {}

    def update_price(self, symbol: str, exchange: str, bid: float, ask: float):
        self.prices[f"{symbol}:{exchange}"] = {"bid": bid, "ask": ask}

    async def analyze(self) -> Optional[Signal]:
        # Simplified: check spread between binance and okx
        threshold = self.params.get("spread_threshold", 10.0)
        for key_binance, _ in list(self.prices.items()):
            if "binance" not in key_binance:
                continue
            symbol = key_binance.split(":")[0]
            okx_key = f"{symbol}:okx"
            if okx_key not in self.prices:
                continue
            p1 = self.prices[key_binance]
            p2 = self.prices[okx_key]
            spread = abs(p1["bid"] - p2["ask"])
            if spread > threshold:
                return Signal(symbol=symbol, action="BUY", price=min(p1["ask"], p2["ask"]),
                              confidence=min(spread / threshold, 1.0), source="cross_exchange")
        return None
