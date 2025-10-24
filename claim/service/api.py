from __future__ import annotations

from typing import Annotated, Literal

from fastapi import APIRouter, Depends
from pydantic import BaseModel

from claim.cases.stock import CardStock, InsufficientStock, OwnerNotFound
from claim.model.card import Card


router = APIRouter()

_stock = CardStock()


def stock() -> CardStock:
    return _stock


@router.post("/admin/store")
async def store(
    request: StoreRequest, stock: Annotated[CardStock, Depends(stock)]
) -> Pending | Failed:
    stock.store(request.cards)
    return Pending(message="Cards stored.")


@router.post("/{owner}/claim")
async def claim(
    owner: str, amount: int, stock: Annotated[CardStock, Depends(stock)]
) -> Pending | Failed:
    try:
        stock.claim(by=owner, amount=amount)
        return Pending(message=f"{amount} Card(s) claimed for {owner}.")
    except InsufficientStock[Card] as error:
        return Failed(message=str(error))


@router.get("/{owner}/cards")
async def get_cards_of(
    owner: str, stock: Annotated[CardStock, Depends(stock)]
) -> Success[list[Card]] | Failed:
    try:
        cards = list(stock.of(owner=owner))
        return Success(message=f"User {owner} has {len(cards)} cards.", data=cards)
    except OwnerNotFound as error:
        return Failed(message=str(error))


# ================ Schemas ================

type StatusKind = Literal["success", "fail", "pending"]


class StoreRequest(BaseModel):
    cards: list[Card]


class OperationStatus(BaseModel):
    status: StatusKind
    message: str


class Success[T](OperationStatus[T]):
    data: T
    status: StatusKind = "success"


class Pending(OperationStatus[None]):
    status: StatusKind = "pending"


class Failed(OperationStatus[None]):
    status: StatusKind = "fail"
