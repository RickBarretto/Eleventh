from typing import Final

import attrs

from crdt.common.types import LogicalClock, NodeID


@attrs.frozen
class ItemID:
    """A unique identifier for an item in a CRDT structure.

    Attributes
    ----------
    node: NodeID
        The identifier of the client that created the item.
    clock: LogicalClock
        A logical clock value representing the creation time of the item.
    """

    node: NodeID
    clock: LogicalClock


@attrs.define
class Item[T]:
    """An item in a CRDT structure.

    Attributes
    ----------
    id: ItemID
        The unique identifier for the item.
    value: T
        The value stored in the item.
    deleted: bool
        A flag indicating whether the item has been logically deleted.
    """

    id: Final[ItemID]
    value: Final[T]
    _deleted: bool = False

    def __bool__(self) -> bool:
        """Return True if the item is not deleted, False otherwise."""
        return not self._deleted

    def exists(self) -> bool:
        """Check if the item exists (is not deleted).

        Returns
        -------
        bool
            True if the item is not deleted, False otherwise.
        """
        return not self._deleted

    def deleted(self) -> bool:
        """Check if the item is deleted.

        Returns
        -------
        bool
            True if the item is deleted, False otherwise.
        """
        return self._deleted

    def delete(self) -> None:
        """Mark the item as deleted."""
        self._deleted = True

    def __del__(self):
        """Mark the item as deleted."""
        self._deleted = True
