from __future__ import annotations

from typing import Final

import attrs

from crdt.common.types import NodeID


@attrs.define
class ActionSequence:
    """A logical clock for tracking causality in a CRDT structure.

    Attributes
    ----------
    node: str
        The identifier of the node (client) that owns this clock.
    counter: int
        A counter representing the logical time.
    """

    node: Final[NodeID]
    _sequence: int = 0

    @property
    def sequence(self) -> int:
        """Get the current value of the logical clock."""
        return self._sequence

    def next(self) -> None:
        """Increment the logical clock."""
        self._sequence += 1

    def merge(self, other: ActionSequence) -> None:
        """Merge another counter into this one by taking the maximum counter value.

        Parameters
        ----------
        other : Counter
            The other clock to merge with.
        """
        if self.node != other.node:
            raise ValueError("Cannot merge clocks from different nodes.")
        self._sequence = max(self._sequence, other._sequence)