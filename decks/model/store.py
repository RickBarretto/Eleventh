from pydantic import BaseModel, Field

from decks.model.card import Card

class Store(BaseModel):
    _stored: list[Card] = Field(default_factory=list)

    def regenerate(self, amount: int) -> list[Card]:
        cards = [Card.random() for _ in range(amount)]
        self._stored = cards
        return cards
        
    def is_empty(self) -> bool:
        return self._stored == []
        
    def claim(self):
        return self._stored.pop()
