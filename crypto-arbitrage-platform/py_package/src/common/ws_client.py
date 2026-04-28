import asyncio
import json
import logging
import time
from typing import Callable, Optional
from datetime import datetime, timezone

import websockets

logger = logging.getLogger(__name__)


class NodeWebSocketClient:
    def __init__(self, ws_url: str, token: str, on_command: Optional[Callable] = None):
        self.ws_url = f"{ws_url}?token={token}"
        self.on_command = on_command
        self._ws = None
        self._running = False

    async def connect(self):
        self._ws = await websockets.connect(self.ws_url)
        self._running = True
        logger.info(f"WebSocket connected: {self.ws_url}")

    async def send_heartbeat(self, data: dict = None):
        await self.send({"type": "heartbeat", "ts": datetime.now(timezone.utc).isoformat(), "data": data or {}})

    async def send(self, msg: dict):
        if self._ws:
            await self._ws.send(json.dumps(msg))

    async def run_forever(self, heartbeat_interval: int = 30):
        """Main event loop: send heartbeat, receive commands."""
        while self._running:
            try:
                recv_task = asyncio.create_task(self._ws.recv())
                done, pending = await asyncio.wait(
                    [recv_task],
                    timeout=heartbeat_interval,
                )
                if recv_task in done:
                    raw = recv_task.result()
                    msg = json.loads(raw)
                    await self._handle_message(msg)
                else:
                    recv_task.cancel()
                await self.send_heartbeat()
            except websockets.ConnectionClosed:
                logger.warning("WebSocket disconnected, reconnecting...")
                await self._reconnect()
            except Exception as e:
                logger.error(f"WS loop error: {e}")
                await self._reconnect()

    async def _handle_message(self, msg: dict):
        msg_type = msg.get("type")
        if msg_type == "command" and self.on_command:
            await self.on_command(msg.get("data", {}))
        elif msg_type == "config_update":
            logger.info(f"Config update: {msg.get('data')}")
        elif msg_type == "ack":
            pass

    async def _reconnect(self):
        await asyncio.sleep(1)
        try:
            await self.connect()
        except Exception as e:
            logger.error(f"Reconnect failed: {e}")

    async def close(self):
        self._running = False
        if self._ws:
            await self._ws.close()
