from pydantic import BaseModel, UUID4


class Card(BaseModel):
    """Pydantic model Card."""

    id: UUID4
    name: str
    power: int
