from dataclasses import dataclass, field, asdict
from pathlib import Path
import yaml

DEFAULT_CONFIG_PATH = "node_config.yaml"


@dataclass
class NodeConfig:
    node_id: str = ""
    node_type: str = "trader"  # collector | trader | alert
    exchange: str = ""  # only for collector
    ip: str = "0.0.0.0"
    api_port: int = 8081
    api_host: str = "0.0.0.0"
    ws_url: str = "ws://localhost:8080/ws/node"
    register_url: str = "http://localhost:8080/api/v1/nodes/register"
    log_level: str = "INFO"
    strategies: list = field(default_factory=lambda: [
        {"id": "cross-exchange-1", "name": "Cross Exchange Arbitrage", "type": "cross_exchange", "enabled": False},
        {"id": "triangular-1", "name": "Triangular Arbitrage", "type": "triangular", "enabled": False},
        {"id": "statistical-1", "name": "Statistical Arbitrage", "type": "statistical", "enabled": False},
    ])

    @classmethod
    def load(cls, path: str = DEFAULT_CONFIG_PATH) -> "NodeConfig":
        p = Path(path)
        if p.exists():
            data = yaml.safe_load(p.read_text())
            return cls(**data)
        return cls()

    def save(self, path: str = DEFAULT_CONFIG_PATH):
        p = Path(path)
        p.write_text(yaml.dump(asdict(self), default_flow_style=False, allow_unicode=True))
