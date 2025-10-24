from __future__ import annotations

from collections.abc import Iterable

import attrs

from crdt.common import ActionSequence, Item, ItemID, NodeID


@attrs.define
class Stack[T]:
    """A simple CRDT stack implementation.

    Attributes
    ----------
    node: NodeID
        The identifier of the node (client) that owns this stack.
    clock: ActionSequence
        The logical clock for tracking causality.
    items: list[Item]
        The list of items in the stack.
    """

    node: NodeID
    items: list[Item[T]] = attrs.field(factory=list)
    clock: ActionSequence = attrs.field(init=False)

    def __attrs_post_init__(self):
        self.clock = ActionSequence(self.node)

    def push(self, *values: Iterable[T]) -> None:
        """Push a new item onto the stack.

        Parameters
        ----------
        value : Any
            The value to be pushed onto the stack.
        """
        for value in values:
            self.clock.next()
            item_id = ItemID(node=self.node, clock=self.clock.sequence)
            item = Item(id=item_id, value=value)
            self.items.append(item)

    def pop(self) -> T | None:
        """Pop an item from the stack.

        Returns
        -------
        Item | None
            The popped item, or None if the stack is empty.
        """
        for index in range(len(self.items) - 1, -1, -1):
            item = self.items[index]
            if item.exists():
                item.delete()
                return item.value
        return None

    def peek(self) -> T | None:
        """Peek at the top item of the stack without removing it.

        Returns
        -------
        Item | None
            The top item, or None if the stack is empty.
        """
        for item in reversed(self.items):
            if item:
                return item.value
        return None

    def values(self) -> list[T]:
        """Get a list of all non-deleted values in the stack.

        Returns
        -------
        list[T]
            The list of non-deleted values.
        """
        return [item.value for item in self.items if item.exists()]

    def is_empty(self) -> bool:
        """Check if the stack is empty.

        Returns
        -------
        bool
            True if the stack is empty, False otherwise.
        """
        return self.values() == []

    def __len__(self) -> int:
        """Get the number of non-deleted items in the stack.

        Returns
        -------
        int
            The number of non-deleted items.
        """
        return sum(1 for item in self.items if item)

    def merge(self, other: Stack[T]) -> None:
        """Merge another stack into this one.

        Parameters
        ----------
        other : Stack[T]
            The other stack to merge with.
        """
        if self.node != other.node:
            message = "Cannot merge stacks from different nodes."
            raise ValueError(message)

        self.clock.merge(other.clock)
        existing_ids = {item.id for item in self.items}

        for item in other.items:
            if item.id not in existing_ids:
                self.items.append(item)
            else:
                index = next(i for i, it in enumerate(self.items) if it.id == item.id)
                if item.deleted() and self.items[index].exists():
                    self.items[index].delete()

        self.items.sort(key=lambda item: (item.id.clock, item.id.node))
