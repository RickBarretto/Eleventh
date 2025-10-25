from decks.model.deck import Decks
from decks.model.store import Store
from plugins.cluster.model import Cluster
from plugins.leader.leader import Leader

async def get_cluster() -> Cluster:
    """Get cluster instance. Overridden by plugin."""
    raise NotImplementedError("You must override this by calling plug_decks.")


async def get_leader() -> Leader:
    """Get leader instance. Overridden by plugin."""
    raise NotImplementedError("You must override this by calling plug_decks.")

async def get_store() -> Store:
    """Get store instance. Overridden by plugin."""
    raise NotImplementedError("You must override this by calling plug_decks.")


async def get_decks() -> Decks:
    """Get decks instance. Overridden by plugin."""
    raise NotImplementedError("You must override this by calling plug_decks.")
