import httpx
from dataclasses import dataclass, asdict


@dataclass
class RegisterRequest:
    node_id: str
    node_type: str
    exchange: str = ""
    ip: str = "0.0.0.0"
    api_port: int = 8081
    version: str = "0.1.0"


@dataclass
class RegisterResponse:
    node_id: str
    ip: str
    api_port: int
    token: str
    node_type: str
    status: str


async def call_register(url: str, req: RegisterRequest) -> RegisterResponse:
    async with httpx.AsyncClient() as client:
        resp = await client.post(url, json=asdict(req), timeout=10)
        resp.raise_for_status()
        data = resp.json()
        return RegisterResponse(**data)
