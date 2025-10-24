import attrs


@attrs.frozen
class Card:
    id: str
    name: str
    value: int


@attrs.frozen
class RewardingGeneration:
    def generate(self, /, amount: int) -> list[Card]:
        return [
            Card(id=str(i), name=f"Reward {i}", value=i * 10)
            for i in range(1, amount + 1)
        ]
