from typing import TYPE_CHECKING, Annotated

from fastapi import APIRouter, Depends
from pydantic import UUID4

from plugins.leader.leader import Leader

api = APIRouter(prefix="/leader")


def get_leader() -> Leader:
    raise NotImplementedError("You must override this.")


type Username = str


@api.post("/{user}/trade/{card}")
async def trade_card(
    user: Username, card: UUID4, leader: Annotated[Leader, Depends(get_leader)]
):
    pass


@api.post("/{user}/propose")
async def propose_trade(
    user: Username,
    to: Username,
    proposed_card: UUID4,
    wish_card: UUID4,
    leader: Annotated[Leader, Depends(get_leader)],
):
    pass


@api.post("/{user}/proposals/{proposal_id}/reject")
async def reject_proposal(
    user: Username,
    proposal_id: UUID4,
    leader: Annotated[Leader, Depends(get_leader)],
):
    pass


@api.post("/{user}/proposals/{proposal_id}/accept")
async def accept_proposal(
    user: Username,
    proposal_id: UUID4,
    leader: Annotated[Leader, Depends(get_leader)],
):
    pass
