from collections.abc import Iterable
from typing import override

import attrs

from claim.model.card import Card

type UserID = str
type Owner = UserID
"""Owner of a Card. The same as `UserID`, that is a simple `str`."""


@attrs.frozen
class InsufficientStock[T](LookupError):
    available: list[T]
    requested: int

    @override
    def __str__(self) -> str:
        return (
            f"Insufficient stock. Tried to request {self.requested} items "
            f"but only {len(self.available)} available."
        )


@attrs.frozen
class OwnerNotFound(KeyError):
    owner: str

    @override
    def __str__(self) -> str:
        return f"Owner {self.owner} not found."


@attrs.define
class CardStock:
    available: list[Card] = attrs.field(factory=list)
    owned: dict[Owner, list[Card]] = attrs.field(factory=dict)
    history: list[str] = attrs.field(factory=list, init=False)

    def store(self, cards: list[Card]):
        self.available.extend(cards)
        self.history.append(f"[STORE] {cards}")

    def claim(self, by: Owner, amount: int = 1):
        owner = by
        if len(self.available) < amount:
            raise InsufficientStock[Card](available=self.available, requested=amount)

        claimed = self.available[:amount]
        del self.available[:amount]
        self.owned[owner].extend([card for card in claimed])
        self.history.append(f"[CLAIM] {claimed} by {owner}")

    def of(self, owner: Owner) -> Iterable[Card]:
        try:
            return (card for card in self.owned[owner])
        except KeyError as error:
            raise OwnerNotFound(owner=owner) from error
