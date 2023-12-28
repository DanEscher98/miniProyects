import uvicorn
import socket
from fastapi import FastAPI


app = FastAPI()


@app.get("/")
async def main_route():
    ip_address = socket.gethostbyname(socket.gethostname())
    return {"message": f"Hi there from the Python server! IP: {ip_address}"}


def start():
    """Launched with `poetry run start` at root level"""
    uvicorn.run("mypy_server.main:app", host="0.0.0.0", port=8000, reload=True)
