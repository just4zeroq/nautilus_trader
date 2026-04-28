import asyncio
import logging

from src.common.config import NodeConfig
from src.common.registry import RegisterRequest, call_register
from src.common.ws_client import NodeWebSocketClient
from src.trading_node.api_server import run_api_server

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


async def main():
    config = NodeConfig.load()
    req = RegisterRequest(
        node_id=config.node_id,
        node_type=config.node_type,
        exchange=config.exchange,
        ip=config.ip,
        api_port=config.api_port,
    )
    resp = await call_register(config.register_url, req)
    config.node_id = resp.node_id
    config.save()
    logger.info(f"Registered as {resp.node_id}")

    ws = NodeWebSocketClient(config.ws_url, resp.token, on_command=handle_command)
    await ws.connect()

    api_task = asyncio.create_task(run_api_server(config.api_host, config.api_port))
    ws_task = asyncio.create_task(ws.run_forever())
    await asyncio.gather(api_task, ws_task)


def handle_command(data: dict):
    logger.info(f"Command received: {data}")


if __name__ == "__main__":
    asyncio.run(main())
