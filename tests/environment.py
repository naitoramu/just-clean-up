import subprocess
import time
import socket
import os
import requests
import pymongo


def find_free_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('', 0))
        return s.getsockname()[1]


def before_all(context):
    # Set up MongoDB
    mongo_port = find_free_port()
    mongo_user = "it-user"
    mongo_passwd = "it-passwd"

    context.mongo_process = subprocess.Popen([
        "docker", "run", "--name", "mongo-test", "-d",
        "-p", f"{mongo_port}:27017",
        "-e", f"MONGO_INITDB_ROOT_USERNAME={mongo_user}",
        "-e", f"MONGO_INITDB_ROOT_PASSWORD={mongo_passwd}",
        "mongo"
    ])

    context.mongo_url = f"mongodb://{mongo_user}:{mongo_passwd}@localhost:{mongo_port}"

    # Wait for MongoDB to start
    time.sleep(5)

    # Set up Rust API server
    rust_port = find_free_port()
    env = os.environ.copy()
    env["DATABASE_URL"] = context.mongo_url
    env["PORT"] = str(rust_port)

    context.rust_app_process = subprocess.Popen(["./target/release/just-clean-up"], env=env)
    context.api_url = f"http://localhost:{rust_port}"

    # Wait for the API server to be ready
    for _ in range(20):
        try:
            requests.get(f"{context.api_url}/health")
            break
        except requests.ConnectionError:
            time.sleep(1)
    else:
        raise Exception("API failed to start in the expected time")


def after_all(context):
    context.rust_app_process.terminate()
    subprocess.run(["docker", "rm", "-f", "mongo-test"])


def before_scenario(context, scenario):
    client = pymongo.MongoClient(context.mongo_url)
    db = client["just-clean-up"]

    for collection_name in db.list_collection_names():
        db[collection_name].delete_many({})
