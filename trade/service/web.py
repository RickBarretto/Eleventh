from fastapi import APIRouter

pages = APIRouter()


@pages.get("/")
async def homepage():
    return {"message": "Welcome to the Trade Service!"}


@pages.get("/{user}")
async def user_page(user: str):
    return {"message": f"Welcome, {user}, to your trade dashboard!"}


@pages.get("/admin")
async def admin_page():
    return {"message": "Admin Panel - Manage your trades here"}


@pages.get("/admin/logs")
async def admin_logs():
    return {"message": "Admin Logs - View your logs here"}
