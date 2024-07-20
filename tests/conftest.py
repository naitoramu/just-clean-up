import subprocess
import time
import pymongo
import pytest
import socket
import os

import requests


def find_free_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('', 0))
        return s.getsockname()[1]


@pytest.fixture(scope="session")
def mongo_url():
    mongo_port = find_free_port()
    mongo_user = "it-user"
    mongo_passwd = "it-passwd"

    mongo_process = subprocess.Popen([
        "docker", "run", "--name", "mongo-test", "-d",
        "-p", f"{mongo_port}:27017",
        "-e", f"MONGO_INITDB_ROOT_USERNAME={mongo_user}",
        "-e", f"MONGO_INITDB_ROOT_PASSWORD={mongo_passwd}",
        "mongo"
    ])

    yield f"mongodb://{mongo_user}:{mongo_passwd}@localhost:{mongo_port}"

    mongo_process.terminate()


@pytest.fixture(scope="session", autouse=True)
def api_server(mongo_url):
    time.sleep(5)  # Czekamy, aż MongoDB się uruchomi

    # Find an available port for Rust app
    rust_port = find_free_port()

    # Set environment variables for the Rust application
    env = os.environ.copy()
    env["DATABASE_URL"] = mongo_url
    env["PORT"] = str(rust_port)

    # Start the Rust application
    rust_app_process = subprocess.Popen(["./target/release/just-clean-up"], env=env)
    api_url = f"http://localhost:{rust_port}"
    for _ in range(20):
        try:
            requests.get(f"{api_url}/health")
            break
        except requests.ConnectionError:
            time.sleep(1)
    else:
        raise Exception("API failed to start in the expected time")

    yield api_url

    # Teardown
    rust_app_process.terminate()
    subprocess.run(["docker", "rm", "-f", "mongo-test"])


@pytest.fixture(autouse=True)
def clear_database(mongo_url):
    client = pymongo.MongoClient(mongo_url)
    db = client["just-clean-up"]

    for collection_name in db.list_collection_names():
        db[collection_name].delete_many({})
