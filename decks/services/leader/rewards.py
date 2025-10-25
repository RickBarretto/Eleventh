from typing import TYPE_CHECKING, Annotated
from fastapi import APIRouter, Depends

from decks.model.card import Card
from decks.model.store import Store
from decks.model.deck import Decks

if TYPE_CHECKING:
    from plugins.cluster.model import Cluster

api = APIRouter(prefix="/leader")

type Username = str


def subscribers() -> Cluster:
    raise NotImplementedError("You must override this.")


def get_store() -> Store:
    raise NotImplementedError("You must override this.")


def get_decks() -> Decks:
    raise NotImplementedError("You must override this.")


@api.post("/store/regenerate")
async def regenerate(
    store: Annotated[Store, Depends(get_store)],
    subscribers: Annotated[Cluster, Depends(subscribers)],
):
    AMOUNT = 500
    cards = store.regenerate(AMOUNT)
    await subscribers.broadcast.post("/store", json={"cards": cards})
    return {"status": "ok"}


@api.post("/{user}/store/claim")
async def claim_card(
    user: Username,
    store: Annotated[Store, Depends(get_store)],
    decks: Annotated[Store, Depends(get_decks)],
    subscribers: Annotated[Cluster, Depends(subscribers)],
):
    if store.is_empty():
        await regenerate(subscribers=subscribers)

    claimed = store.claim()
    decks.add_to(user, claimed)
    await subscribers.broadcast.post("/store/claim")
    await subscribers.broadcast.post(f"{user}/add", json={"cards": claimed})
    return {"status": "ok"}
