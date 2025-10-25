from __future__ import annotations

from typing import Protocol

import attrs

from plugins.leader.broadcast import Broadcast

__all__ = ["Leader"]


@attrs.define
class Leader:
    id: str
    url: str
    followers: list[Follower]
    prefix: str = "/follower"
    
    @property
    def broadcast(self) -> Broadcast:
        clients = [f.url for f in self.followers]
        return Broadcast(clients, self.prefix)


class Follower(Protocol):
    id: str
    url: str
