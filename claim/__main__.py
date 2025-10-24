"""Rewarding Claiming service.

At A Glance
===========

Run this using:

    $ poetry run python -m claim Node1@127.0.0.1:8011
    $ poetry run python -m claim Node2@127.0.0.1:8012 --join 127.0.0.1:8011
    $ poetry run python -m claim Node3@127.0.0.1:8013 --join 127.0.0.1:8011


You can open the browser in the following URLs:
- http://127.0.0.1:8011: For user interface.
- http://127.0.0.1:8011/admin: For admin interface.
- http://127.0.0.1:8011/docs: For API documentation.

As Admin
~~~~~~~~

As Admin, you can attach/detach peers and check cluster health.
You can also see the log of the system and add cards as well.

As User
~~~~~~~

As User, you can claim rewards.
And this is visible for everyone in the cluster and you can check this as an admin.


Running from Shell
==================

After running the above commands, you can also interact with the service:

Service
~~~~~~~

Claim Rewards
-------------

    $ curl -X POST https://127.0.0.1:8011/api/claim?n=5
    {
        "status": "success",
        "cards": [
            { "id": 1, "name": "Card 1", "power": 15 },
            { "id": 2, "name": "Card 2", "power": 10 },
            { "id": 3, "name": "Card 3", "power": 20 },
            { "id": 4, "name": "Card 4", "power": 25 },
            { "id": 5, "name": "Card 5", "power": 30 }
        ]
    }



Store Rewards
-------------

    $ curl -X POST https://127.0.0.1:8011/api/store -d '{"cards":[{"id":1,"name":"Card 1","power":15},{"id":2,"name":"Card 2","power":10},{"id":3,"name":"Card 3","power":20},{"id":4,"name":"Card 4","power":25},{"id":5,"name":"Card 5","power":30}]}'

"""

from cyclopts import App as Cyclopts
from fastapi import FastAPI

from plugins.cluster import plug_cluster
from claim.service.api import service
from claim.service.web import pages

webapp = FastAPI(
    title="Reward Claim Service",
    version="0.1.0",
    debug=True,
)
cli = Cyclopts("claim", help="Reward Claim Service.")

webapp.include_router(service)
webapp.include_router(pages)
plug_cluster(cli, webapp)

if __name__ == "__main__":
    cli()
