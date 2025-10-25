from fastapi import FastAPI
from loguru import logger

app = FastAPI()

@app.get("/")
async def home():
    logger.info("Hello")
    pass
