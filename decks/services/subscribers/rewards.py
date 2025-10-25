from typing import Annotated

from fastapi import APIRouter, Depends
from pydantic import BaseModel

from decks.model.card import Card
from decks.model.deck import Decks
from decks.model.store import Store

api = APIRouter(prefix="subscriber")

# Placeholder for service providers


def get_store() -> Store:
    raise NotImplementedError("You must override this.")


def get_decks() -> Decks:
    raise NotImplementedError("You must override this.")


# Types

type Username = str


class Cards(BaseModel):
    cards: list[Card]


# Routes


@api.post("/store")
async def new_store(
    cards: Cards,
    store: Annotated[Store, Depends(get_store)],
):
    store.re_stock(cards.cards)
    return {"status": "updated"}


@api.post("/claim")
async def claim(
    store: Annotated[Store, Depends(get_store)],
):
    try:
        store.claim()
        return {"status": "updated"}
    except Exception as e:
        return {"status": "failed", "message": str(e)}


@api.post("/{user}/add")
async def add_to_user(
    user: Username, cards: Cards, decks: Annotated[Decks, Depends(get_decks)]
):
    decks.add_to(user, *cards.cards)
    return {"status": "updated"}
