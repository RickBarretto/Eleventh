"""Card Trading service.

At A Glance
===========

Run this using:

    $ poetry run python -m trade Node1@127.0.0.1:8031
    $ poetry run python -m trade Node2@127.0.0.1:8032 --join 127.0.0.1:8031
    $ poetry run python -m trade Node3@127.0.0.1:8033 --join 127.0.0.1:8031


You can open the browser in the following URLs:
- http://127.0.0.1:8031: For user interface.
- http://127.0.0.1:8031/admin: For admin interface.
- http://127.0.0.1:8031/docs: For API documentation.

As Admin
~~~~~~~~

As Admin, you can attach/detach peers and check cluster health.
You can also see the log of the system.

As User
~~~~~~~

As User, you can trade cards with other users.
Basicaly, you can propose trades and accept/reject trades.

Running from Shell
==================

After running the above commands, you can also interact with the service:

Service
~~~~~~~

List Global Trades
------------------

    $ curl https://127.0.0.1:8031/api/trades
    { "status": "success", "trades": [ { "id": 1, "name": "Card 1", "power": 15 }, { "id": 2, "name": "Card 2", "power": 10 } ] }

List My Trades
--------------

    $ curl https://127.0.0.1:8031/api/rick/trades
    {
        "status": "success",
        "proposals": [
            {
                "id": 1,
                "to": "morty",
                "wants": { "id": 2, "name": "Card 2", "power": 10 },
                "card": { "id": 2, "name": "Card 2", "power": 12 },
                "status": "pending"
            },
            {
                "id": 2,
                "to": "summer",
                "wants": { "id": 3, "name": "Card 3", "power": 21 },
                "card": { "id": 3, "name": "Card 3", "power": 20 },
                "status": "accepted"
            }
        ],
        "received": [
            {
                "id": 3,
                "from": "beth",
                "wants": { "id": 4, "name": "Card 4", "power": 26 },
                "card": { "id": 4, "name": "Card 4", "power": 25 },
                "status": "pending"
            }
        ],
    }

Submit a Trade
----------------

    $ curl -X POST https://127.0.0.1:8031/api/rick/trade -d "{ 'id': 1, 'name': 'Card 1', 'power': 15 }"
    { "status": "success", "message": "Trade submitted" }

Propose a Trade
----------------

    $ curl -X POST https://127.0.0.1:8031/api/rick/trade/propose?to=morty&card_id=2
    { "status": "success", "message": "Trade proposed to morty" }

Accept a Trade
--------------

    $ curl -X POST https://127.0.0.1:8031/api/rick/trade/accept?trade_id=1
    { "status": "success", "message": "Trade accepted" }

Reject a Trade
--------------

    $ curl -X POST https://127.0.0.1:8031/api/rick/trade/reject?trade_id=1
    { "status": "success", "message": "Trade rejected" }

"""

from cyclopts import App as Cyclopts
from fastapi import FastAPI

from plugins.cluster import plug_cluster
from trade.service.api import service
from trade.service.web import pages

webapp = FastAPI(
    title="Card Trading Service",
    version="0.1.0",
    debug=True,
)
cli = Cyclopts("trade", help="Card Trading Service.")

webapp.include_router(service)
webapp.include_router(pages)
plug_cluster(cli, webapp)

if __name__ == "__main__":
    cli()
