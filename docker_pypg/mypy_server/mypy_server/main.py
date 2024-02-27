import uvicorn
import socket
import yaml
import os
import psycopg2
from fastapi import FastAPI


app = FastAPI()

config_path = "/app/config.yaml"
if os.path.exists(config_path):
    with open(config_path, "r") as config_file:
        config = yaml.safe_load(config_file)
else:
    raise FileNotFoundError(f"Config file not found at path: {config_path}")

db_config = {
    'user': config['database']['username'],
    'password': config['database']['password'],
    'host': config['database']['host'],
    'port': config['database']['port'],
    'database': config['database']['db_name'],
}


def get_connection():
    return psycopg2.connect(**db_config)


@app.get("/test_db")
def test_db():
    try:
        with get_connection() as connection, connection.cursor() as db:
            # Retrieve the list of tables from information_schema
            db.execute("SELECT table_name FROM information_schema.tables WHERE table_schema='public';")
            tables = [row[0] for row in db.fetchall()[1:]]

            return {
                    "message": "Database connection successful",
                    "tables": tables
                    }

    except psycopg2.Error as e:
        return {"error": str(e)}


@app.get("/hello_world")
def hello_world():
    return {"message": "Hello World!"}


@app.get("/")
async def main_route():
    ip_address = socket.gethostbyname(socket.gethostname())
    return {"message": f"Hi there from the Python server! IP: {ip_address}"}


def start():
    """Launched with `poetry run start` at root level"""
    uvicorn.run("mypy_server.main:app", host="0.0.0.0", port=8000, reload=True)
