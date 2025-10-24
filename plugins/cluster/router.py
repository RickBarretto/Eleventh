from fastapi import APIRouter
from pydantic import BaseModel

from node.cluster.model import Cluster


__all__ = ["router"]


def router(cluster: Cluster) -> APIRouter:
    router = APIRouter(prefix="/cluster")

    class Join(BaseModel):
        node: str

    @router.post("/join")
    async def join_cluster(join: Join):
        """Add a new node dynamically"""
        await cluster.on_join_cluster(join.node)
        return {"nodes": sorted(list(cluster.nodes))}

    @router.post("/node/add")
    async def add_node_individually(join: Join):
        """Add a new node to an individual node"""
        cluster.on_add(join.node)
        return {"status": f"node {join.node} added."}

    @router.get("/nodes")
    async def list_all_nodes():
        return {"nodes": sorted(list(cluster.nodes))}

    return router
