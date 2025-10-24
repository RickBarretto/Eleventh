from fastapi import APIRouter


pages = APIRouter()


@pages.get("/")
async def homepage():
    return {"message": "Welcome to the Reward Generation Service!"}


@pages.get("/admin")
async def admin_page():
    return {"message": "Admin Panel - Manage your rewards here"}


@pages.get("/admin/logs")
async def admin_logs():
    return {"message": "Admin Logs - View your logs here"}
