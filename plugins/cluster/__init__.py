import asyncio
from cyclopts import App as CliApp
from fastapi import FastAPI
import uvicorn

from node.cluster.model import Cluster
from node.cluster.router import router as cluster_router

__all__ = ["plug_cluster"]


def plug_cluster(cli: CliApp, webserver: FastAPI):
    @cli.command()
    def start(node: str, join: str | None = None):
        """Start a node

        Usage
        -----
            start Node1@127.0.0.1:8000
            start Node2@127.0.0.1:8001 --join 127.0.0.1:8000
            start Node3@127.0.0.1:8002 --join 127.0.0.1:8000
        """
        loop = asyncio.get_event_loop()
        host, port = node.split("@")[1].split(":")

        cluster = Cluster({node})

        if join:
            loop.run_until_complete(cluster.join_cluster(join, node))

        print(f"[INFO] Node {node} started. Known nodes: {cluster.nodes}")

        webserver.include_router(cluster_router(cluster))
        uvicorn.run(webserver, host=host, port=int(port))
