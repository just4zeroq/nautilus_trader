from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Optional


@dataclass
class Signal:
    symbol: str
    action: str  # BUY, SELL, CLOSE
    price: float
    confidence: float
    source: str


class BaseStrategy(ABC):
    def __init__(self, strategy_id: str, name: str, strategy_type: str):
        self.id = strategy_id
        self.name = name
        self.strategy_type = strategy_type
        self.enabled = False
        self.params: dict = {}

    @abstractmethod
    async def analyze(self) -> Optional[Signal]:
        pass
