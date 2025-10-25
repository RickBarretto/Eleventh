from __future__ import annotations

from sys import prefix
from typing import Protocol

import attrs

from httpx import AsyncClient


__all__ = ["Follower"]


@attrs.define
class Follower:
    id: str
    url: str
    follows: Leader
    _prefix: str = "/leader"

    @property
    def prefix(self) -> str:
        return self._prefix.strip("/")

    @property
    def client(self) -> AsyncClient:
        return AsyncClient(base_url=self.follows.url + self.prefix)

    async def has_leader(self) -> bool:
        try:
            response = await self.client.get("/")
            return response.is_success
        except Exception:
            return False


class Leader(Protocol):
    id: str
    url: str
