

from typing import Annotated

from fastapi import Depends
from decks.model.deck import Decks
from decks.model.store import Store
from plugins.cluster.model import Cluster
from plugins.leader.leader import Leader

__all__ = [
    "ClusterService",
    "LeaderService",
    "StoreService",
    "DecksService",
]

def get_cluster() -> Cluster:
    raise NotImplementedError("You must override this.")

def get_leader() -> Leader:
    raise NotImplementedError("You must override this.")

def get_store() -> Store:
    raise NotImplementedError("You must override this.")

def get_decks() -> Decks:
    raise NotImplementedError("You must override this.")

type ClusterService = Annotated[Cluster, Depends(get_cluster)]
type LeaderService = Annotated[Leader, Depends(get_leader)]
type StoreService = Annotated[Store, Depends(get_store)]
type DecksService = Annotated[Decks, Depends(get_decks)]