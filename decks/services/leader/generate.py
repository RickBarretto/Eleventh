from typing import TYPE_CHECKING
from fastapi import APIRouter, Depends
from fastapi.routing import Annotated
from pydantic import UUID4

from decks.model.card import Card
from decks.model.store import Store

if TYPE_CHECKING:
    from plugins.cluster.model import Cluster

api = APIRouter(
    prefix="/leader"
)

type Username = str

def subscribers() -> Cluster:
    raise NotImplementedError("You must override this.")

_store = Store()
def get_store() -> Store:
    return _store
    
_decks = Decks()
def get_decks() -> Decks:
    return _decks


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
    decks.add(claimed)
    await subscribers.broadcast.post("/subscriber/{user}/add", json={
        "cards": claimed
    })
    return { "status": "ok" }
    
    
@api.post("/{user}/trade/{card}")
async def trade_card(
    user: Username,
    card: UUID4,

    subscribers: Annotated[Cluster, Depends(subscribers)]
):
    pass
    

@api.post("/{user}/propose")
async def propose_trade(
    user: Username,
    to: Username,
    proposed_card: UUID4,
    wish_card: UUID4,

    subscribers: Annotated[Cluster, Depends(subscribers)]
):
    pass
    

@api.post("/{user}/proposals/{proposal_id}/reject")
async def reject_proposal(
    user: Username,
    proposal_id: UUID4,
    
    subscribers: Annotated[Cluster, Depends(subscribers)]
):
    pass


@api.post("/{user}/proposals/{proposal_id}/accept")
async def accept_proposal(
    user: Username,
    proposal_id: UUID4,
    
    subscribers: Annotated[Cluster, Depends(subscribers)]
):
    pass