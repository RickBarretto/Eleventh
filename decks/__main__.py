"""Deck Management service.

At A Glance
===========

Run this using:

    $ poetry run python -m decks Node1@127.0.0.1:8021
    $ poetry run python -m decks Node2@127.0.0.1:8022 --join 127.0.0.1:8021
    $ poetry run python -m decks Node3@127.0.0.1:8023 --join 127.0.0.1:8021


You can open the browser in the following URLs:
- http://127.0.0.1:8021: For user interface.
- http://127.0.0.1:8021/admin: For admin interface.
- http://127.0.0.1:8021/docs: For API documentation.

As Admin
~~~~~~~~

As Admin, you can attach/detach peers and check cluster health.
You can also see the log of the system and add cards as well.

As User
~~~~~~~

As User, you can manage your cards.
And this is visible for everyone in the cluster and you can check this as an admin.


Running from Shell
==================

After running the above commands, you can also interact with the service:

Service
~~~~~~~

Add Card to Deck
----------------

    $ curl -X POST https://127.0.0.1:8021/api/rick/deck -d '{"id":6,"name":"Card 6","power":35}' -H "Content-Type: application/json"
    { "status": "success", "message": "Card 6 added" }

Remove Card from Deck
---------------------

    $ curl -X DELETE https://127.0.0.1:8021/api/rick/deck?card=6
    { "status": "success", "message": "Card 6 removed" }

Get Deck
--------

    $ curl https://127.0.0.1:8021/api/rick/deck
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

"""

from cyclopts import App as Cyclopts
from fastapi import FastAPI

from plugins.cluster import plug_cluster
from decks.services.rewards import api as rewards

webapp = FastAPI(
    title="Deck Management Service",
    version="0.1.0",
    debug=True,
)
cli = Cyclopts("decks", help="Deck Management Service.")

webapp.get("/leader")(lambda: {"status": "ok"})
webapp.include_router(rewards)
plug_cluster(cli, webapp)

if __name__ == "__main__":
    cli()
