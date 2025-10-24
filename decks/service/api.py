from fastapi import APIRouter


service = APIRouter(prefix="/api")


@service.get("/{user}/deck")
async def list_cards(user: str):
    return {"user": user, "deck": []}


@service.post("/{user}/deck")
async def add_card(user: str, card: dict):
    return {"status": "success", "message": f"Card {card} added"}


@service.delete("/{user}/deck")
async def delete_card(user: str, card: int):
    return {"status": "success", "message": f"Card {card} deleted"}
