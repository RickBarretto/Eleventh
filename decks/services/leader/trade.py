from typing import TYPE_CHECKING, Annotated

from fastapi import APIRouter, Depends
from pydantic import UUID4

if TYPE_CHECKING:
    from plugins.cluster.models import Cluster

api = APIRouter(prefix="/leader")


def subscribers() -> Cluster:
    raise NotImplementedError("You must override this.")


type Username = str


@api.post("/{user}/trade/{card}")
async def trade_card(
    user: Username, card: UUID4, subscribers: Annotated[Cluster, Depends(subscribers)]
):
    pass


@api.post("/{user}/propose")
async def propose_trade(
    user: Username,
    to: Username,
    proposed_card: UUID4,
    wish_card: UUID4,
    subscribers: Annotated[Cluster, Depends(subscribers)],
):
    pass


@api.post("/{user}/proposals/{proposal_id}/reject")
async def reject_proposal(
    user: Username,
    proposal_id: UUID4,
    subscribers: Annotated[Cluster, Depends(subscribers)],
):
    pass


@api.post("/{user}/proposals/{proposal_id}/accept")
async def accept_proposal(
    user: Username,
    proposal_id: UUID4,
    subscribers: Annotated[Cluster, Depends(subscribers)],
):
    pass
