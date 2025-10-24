from fastapi import APIRouter

service = APIRouter(prefix="/api")


@service.get("/trades")
async def list_trades():
    return {"message": "List of trades"}


@service.get("/{user}/trades")
async def list_user_trades(user: str):
    return {"message": f"Trades for user {user}"}


@service.post("/{user}/trade")
async def submit_trade(user: str, card: dict):
    return {"status": "success", "message": f"Trade submitted by {user}"}


@service.post("/{user}/trade/propose")
async def propose_trade(user: str, to: str, card_id: int):
    return {
        "status": "success",
        "message": f"Trade proposed by {user} to {to} for card {card_id}",
    }


@service.post("/{user}/trade/accept")
async def accept_trade(user: str, trade_id: int):
    return {
        "status": "success",
        "message": f"Trade accepted by {user} for trade {trade_id}",
    }


@service.post("/{user}/trade/reject")
async def reject_trade(user: str, trade_id: int):
    return {
        "status": "success",
        "message": f"Trade rejected by {user} for trade {trade_id}",
    }
