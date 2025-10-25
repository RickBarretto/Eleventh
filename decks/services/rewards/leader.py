from typing import Annotated
from fastapi import APIRouter, Depends

from decks.model.store import Store
from decks.model.deck import Deck, Decks
from plugins.leader.leader import Leader

from .deps import DecksService, LeaderService, StoreService

api = APIRouter(prefix="/leader")

type Username = str


@api.post("/store/regenerate")
async def regenerate(store: StoreService, leader: LeaderService):
    AMOUNT = 500
    cards = store.regenerate(AMOUNT)
    await leader.broadcast.post("/store", json={"cards": cards})
    return {"status": "ok"}


@api.post("/{user}/store/claim")
async def claim_card(
    user: Username,
    store: StoreService,
    decks: DecksService,
    leader: LeaderService,
):
    if store.is_empty():
        await regenerate(store=store, leader=leader)

    claimed = store.claim()
    decks.add_to(user, claimed)
    await leader.broadcast.post("/store/claim")
    await leader.broadcast.post(f"{user}/add", json={"cards": claimed})
    return {"status": "ok"}
