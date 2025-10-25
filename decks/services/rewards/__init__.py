from fastapi import APIRouter

from .follower import api as follower
from .leader import api as leader

api = APIRouter()
api.include_router(leader)
api.include_router(follower)
