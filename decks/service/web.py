from fastapi import APIRouter


pages = APIRouter()


@pages.get("/")
async def homepage():
    return {"message": "Welcome to the Deck Management Service!"}


@pages.get("/{user}/deck")
async def user_page(user: str):
    return {"user": user, "deck": []}


@pages.get("/admin")
async def admin_page():
    return {"message": "Admin Panel - Manage your deck here"}


@pages.get("/admin/logs")
async def admin_logs():
    return {"message": "Admin Logs - View your logs here"}
