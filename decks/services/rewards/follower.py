from fastapi import APIRouter
from pydantic import BaseModel

from decks.model.card import Card
from .deps import DecksService, StoreService


api = APIRouter(prefix="subscriber")


type Username = str

class Cards(BaseModel):
    cards: list[Card]


@api.post("/store")
async def new_store(cards: Cards, store: StoreService):
    store.re_stock(cards.cards)
    return {"status": "updated"}


@api.post("/claim")
async def claim(store: StoreService):
    try:
        store.claim()
        return {"status": "updated"}
    except Exception as e:
        return {"status": "failed", "message": str(e)}


@api.post("/{user}/add")
async def add_to_user(user: Username, cards: Cards, decks: DecksService):
    decks.add_to(user, *cards.cards)
    return {"status": "updated"}
