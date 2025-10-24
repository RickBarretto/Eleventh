from typing import TYPE_CHECKING
from fastapi import APIRouter, Depends
from fastapi.routing import Annotated
from pydantic import UUID4

from decks.model.card import Card
from decks.model.store import Store
from decks.model.deck import Decks

if TYPE_CHECKING:
    from plugins.cluster.model import Cluster

api = APIRouter(
    prefix="/leader"
)

type Username = str

def subscribers() -> Cluster:
    raise NotImplementedError("You must override this.")

def get_store() -> Store:
    raise NotImplementedError("You must override this.")
    
def get_decks() -> Decks:
    raise NotImplementedError("You must override this.")


@api.post("/global/deck/regenerate")
async def regenerate(
    store: Annotated[Store, Depends(get_store)],
    subscribers: Annotated[Cluster, Depends(subscribers)]
) -> list[Card]:
    AMOUNT = 500
    cards = store.regenerate(AMOUNT)
    await subscribers.broadcast.post("/subscriber/global/", json={
        "cards": cards
    })
    return { "status": "ok" }


@api.post("/{user}/claim")
async def claim_card(
    user: Username,
    
    store: Annotated[Store, Depends(get_store)],
    decks: Annotated[Store, Depends(get_decks)],
    subscribers: Annotated[Cluster, Depends(subscribers)]
):
    if store.is_empty():
        await regenerate(subscribers=subscribers)
    
    claimed = store.claim()
    decks.add_to(user, claimed)
    await subscribers.broadcast.postra("/subscriber/{user}/add", json={
        "cards": claimed
    })
    return { "status": "ok" }
