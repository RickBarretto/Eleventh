import re
import attrs
import httpx

from plugins.cluster.model import Cluster
from plugins.leader import follower
from plugins.leader.leader import Leader
from plugins.leader.follower import Follower


@attrs.frozen
class LeaderFollower:
    cluster: Cluster
    current_node: str
    
    @property
    async def leader(self) -> Leader:
        leader_node = await self._candidate()
        leader_id, leader_url = leader_node.split("@")
        leader = Leader(id=leader_id, url=leader_url, followers=[])
    
        followers: list[Follower] = list(
            map(
                lambda node: Follower(
                    id=node.split("@")[0],
                    url=node.split("@")[1],
                    follows=leader
                ),
                filter(lambda node: node != leader_node, self.cluster.nodes)
            )
        )
        
        leader.followers = followers
        return leader

    async def _candidate(self) -> str:
        for node in sorted(self.cluster.nodes):
            if await _is_alive(node):
                return node
        return node


async def _is_alive(node: str) -> bool:
    try:
        response = await httpx.AsyncClient(timeout=3).get(f"http://{node}/leader")
        return response.is_success
    except httpx.HTTPError:
        return False