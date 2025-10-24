from collections import defaultdict

from pydantic import BaseModel

from decks.model.card import Card


__all__ = ["Decks"]


type Username = str
type Deck = list[Card]

class Database[K, T](defaultdict[K, T]):
    def __init__(self):
        super(list)


class Decks(BaseModel):
    map: Database[Username, Deck] = Database()
    
    def add_to(self, user: Username, cards: list[Card]):
        self.map[user].extend(cards)
