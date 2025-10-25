from typing import Annotated
from fastapi import APIRouter, Depends

from decks.model.store import Store
from decks.model.deck import Decks
from plugins.leader.leader import Leader

api = APIRouter(prefix="/leader")

type Username = str

def get_cluster():
    raise NotImplementedError("You must override this.")

def get_leader() -> Leader:
    raise NotImplementedError("You must override this.")

def get_store() -> Store:
    raise NotImplementedError("You must override this.")

def get_decks() -> Decks:
    raise NotImplementedError("You must override this.")


@api.post("/store/regenerate")
async def regenerate(
    store: Annotated[Store, Depends(get_store)],
    leader: Annotated[Leader, Depends(get_leader)],
):
    AMOUNT = 500
    cards = store.regenerate(AMOUNT)
    await leader.broadcast.post("/store", json={"cards": cards})
    return {"status": "ok"}


@api.post("/{user}/store/claim")
async def claim_card(
    user: Username,
    store: Annotated[Store, Depends(get_store)],
    decks: Annotated[Decks, Depends(get_decks)],
    leader: Annotated[Leader, Depends(get_leader)],
):
    if store.is_empty():
        await regenerate(store=store, leader=leader)

    claimed = store.claim()
    decks.add_to(user, claimed)
    await leader.broadcast.post("/store/claim")
    await leader.broadcast.post(f"{user}/add", json={"cards": claimed})
    return {"status": "ok"}
