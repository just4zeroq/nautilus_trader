from src.trading_node.strategies.base import BaseStrategy, Signal
from typing import Optional


class TriangularStrategy(BaseStrategy):
    def __init__(self):
        super().__init__("triangular-1", "Triangular Arbitrage", "triangular")
        self.prices: dict[str, tuple[float, float]] = {}

    def update_price(self, pair: str, bid: float, ask: float):
        self.prices[pair] = (bid, ask)

    async def analyze(self) -> Optional[Signal]:
        # Placeholder - triangular arb logic
        return None
