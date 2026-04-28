"""Redis Streams Message Bus for the crypto arbitrage platform.

Replaces the old external message-bus package. This module provides
a unified pub/sub interface over Redis streams, used by both Rust
components (via FFI / direct imports) and Python components.
"""

from dataclasses import dataclass, field
from typing import Any, Optional
import json

import redis.asyncio as aioredis

DEFAULT_PREFIX = "crypto:"
DEFAULT_REDIS_URL = "redis://localhost:6379/0"


@dataclass
class BusMessage:
    """A message published on the bus."""
    stream: str
    key: str
    data: dict
    message_id: str = ""
    prefix: str = DEFAULT_PREFIX


class MessageBus:
    """Unified Redis Streams message bus.

    Provides publish/subscribe over Redis streams for heartbeats,
    commands, and data distribution between platform nodes.
    """

    def __init__(self, redis_url: str = DEFAULT_REDIS_URL, prefix: str = DEFAULT_PREFIX):
        self._redis_url = redis_url
        self._prefix = prefix
        self._redis: Optional[aioredis.Redis] = None

    async def __aenter__(self):
        await self.connect()
        return self

    async def __aexit__(self, *args):
        await self.close()

    async def connect(self) -> None:
        """Open the Redis connection."""
        if self._redis is None:
            self._redis = aioredis.from_url(self._redis_url, decode_responses=True)

    async def close(self) -> None:
        """Close the Redis connection."""
        if self._redis is not None:
            await self._redis.close()
            self._redis = None

    async def publish(
        self,
        topic: str,
        key: str,
        data: Any,
    ) -> str:
        """Publish a message to a Redis stream.

        Args:
            topic: Stream topic (e.g. "system.heartbeat").
            key: Partition key / message dedup key.
            data: JSON-serialisable payload.

        Returns:
            The message ID assigned by Redis.
        """
        await self.connect()
        stream_key = f"{self._prefix}{topic}"
        payload = {
            "key": key,
            "data": json.dumps(data, default=str),
        }
        msg_id = await self._redis.xadd(stream_key, payload)
        return msg_id

    async def subscribe(
        self,
        topics: list[str],
        block: int = 5000,
        count: int = 10,
    ) -> list[BusMessage]:
        """Read new messages from one or more streams (blocking).

        Args:
            topics: Stream topic names to read from.
            block: Max milliseconds to block (0 = forever).
            count: Max messages per stream.

        Returns:
            A list of BusMessage instances.
        """
        await self.connect()
        stream_keys = {f"{self._prefix}{t}": ">" for t in topics}
        results = await self._redis.xread(stream_keys, block=block, count=count)
        messages: list[BusMessage] = []
        for stream_key, entries in results:
            topic = stream_key[len(self._prefix):] if stream_key.startswith(self._prefix) else stream_key
            for msg_id, fields in entries:
                messages.append(BusMessage(
                    stream=topic,
                    key=fields.get("key", ""),
                    data=json.loads(fields.get("data", "{}")),
                    message_id=msg_id,
                    prefix=self._prefix,
                ))
        return messages


__all__ = ["MessageBus", "BusMessage", "DEFAULT_PREFIX"]
