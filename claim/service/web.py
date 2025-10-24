from fastapi import APIRouter


pages = APIRouter()


@pages.get("/")
async def homepage():
    return {"message": "Welcome to the Claim Service API"}


@pages.get("/admin")
async def admin_panel():
    return {"message": "Admin Panel - Manage your cluster here"}


@pages.get("/admin/logs")
async def admin_logs():
    return {"message": "Admin Logs - View your logs here"}


@pages.post("/admin/cards")
async def add_card(card: dict):
    return {"status": "success", "added_card": card}
