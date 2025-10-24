from __future__ import annotations

import attrs
import httpx


__all__ = ["Cluster"]


@attrs.define
class Cluster:
    nodes: set[str] = attrs.field(factory=set)

    async def join_cluster(self, cluster_node: str, node: str):
        base_url = f"http://{cluster_node}"
        async with httpx.AsyncClient(timeout=3, base_url=base_url) as client:
            response = await client.post("/cluster/join", json={"node": node})
            for node in response.json()["nodes"]:
                self.nodes.add(node)

    def on_add(self, peer: str):
        """On new requested to be added to the current Node."""
        self.nodes.add(peer)

    async def on_join_cluster(self, node: str):
        """On requested to be added to the cluster."""
        for other in tuple(self.nodes):
            base_url = f"http://{other}"
            async with httpx.AsyncClient(timeout=3, base_url=base_url) as client:
                _ = await client.post("/cluster/node/add", json={"node": node})

        self.nodes.add(node)
