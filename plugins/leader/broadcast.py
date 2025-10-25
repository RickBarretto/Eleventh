from __future__ import annotations

import asyncio
from typing import Self

import attrs
from httpx import AsyncClient

@attrs.frozen
class Broadcast:
    _followers: list[str]
    _prefix: str = "/"
    
    @property
    def followers(self) -> list[str]:
        return [
            follower.strip("/") 
            for follower in self._followers
        ]
    
    @property
    def prefix(self) -> str:
        return self._prefix.strip("/")

    @property
    def clients(self) -> list[AsyncClient]:
        return [
            AsyncClient(base_url=f"{follower}/{self.prefix}")
            for follower in self.followers
        ]

    async def request(self, method: str, path: str, **kwargs) -> list:
        tasks = []
        for client in self.clients:
            url = path.lstrip("/")
            tasks.append(
                client.request(method, url, **kwargs)
            )
        responses = await asyncio.gather(*tasks, return_exceptions=True)
        return responses

    async def __aenter__(self) -> Self:
        return self

    async def __aexit__(self, exc_type, exc_value, traceback) -> None:
        tasks = [client.aclose() for client in self.clients]
        await asyncio.gather(*tasks)
    
    async def get(self, path: str, **kwargs) -> list:
        return await self.request("GET", path, **kwargs)
    
    async def post(self, path: str, **kwargs) -> list:
        return await self.request("POST", path, **kwargs)
