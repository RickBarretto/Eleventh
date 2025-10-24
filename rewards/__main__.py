"""Rewarding Generation service.

At A Glance
===========

Run this using:

    $ poetry run python -m rewards Node1@127.0.0.1:8001
    $ poetry run python -m rewards Node2@127.0.0.1:8002 --join 127.0.0.1:8001
    $ poetry run python -m rewards Node3@127.0.0.1:8003 --join 127.0.0.1:8001


Open https://127.0.0.1:8001/admin on Browser and connect to other peers.
Then, open https://127.0.0.1:8001 and click "Generate" button to generate text and get rewards from other peers.


Running from Shell
==================

After running the above commands, you can also interact with the service:

Service
~~~~~~~~~~

Generate 500 Cards
------------------

    $ curl -X POST https://127.0.0.1:8001/api/create?n=500

"""

from cyclopts import App as Cyclopts
from fastapi import FastAPI

from plugins.cluster import plug_cluster
from rewards.service.api import service
from rewards.service.web import pages

webapp = FastAPI(
    title="Rewarding Generation Service",
    version="0.1.0",
    debug=True,
)
cli = Cyclopts("rewards", help="Rewarding Generation Service.")

webapp.include_router(service)
webapp.include_router(pages)
plug_cluster(cli, webapp)

if __name__ == "__main__":
    cli()
