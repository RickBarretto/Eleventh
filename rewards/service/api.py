from __future__ import annotations

from typing import Literal

from fastapi import APIRouter
from pydantic import BaseModel

from rewards.usecase.generate import Card, RewardingGeneration

__all__ = ["service"]

service = APIRouter(prefix="/api")


# ========== ========== HTTP Endpoints ========== ==========


@service.get("/reward")
async def generate(n: int):
    use_case = RewardingGeneration()

    cards = use_case.generate(n)
    return RewardsOut.from_model(cards)


# ========== ========== Json Schema ========== ==========


class CardOut(BaseModel):
    id: str
    name: str
    value: int

    @classmethod
    def from_model(cls, model: Card) -> CardOut:
        return cls(id=model.id, name=model.name, value=model.value)


class RewardsOut(BaseModel):
    status: Literal["success", "failure"]
    rewards: list[CardOut]

    @classmethod
    def from_model(cls, models: list[Card]) -> RewardsOut:
        if not models:
            return cls(status="failure", rewards=[])

        return cls(status="success", rewards=[CardOut.from_model(m) for m in models])
