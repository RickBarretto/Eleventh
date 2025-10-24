from random import random
from typing import Self
from uuid import uuid4

from pydantic import BaseModel, UUID4


class Card(BaseModel):
    id: UUID4
    name: str
    power: int

    @classmethod
    def random(cls) -> Self:
        return cls(id=uuid4(), name="Player", power=random.randint(55, 99))
