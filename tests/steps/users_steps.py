import requests
from behave import given


class TestUsersClient:
    def __init__(self, url):
        self.base_path = f"{url}/just-clean-up"

    def register_user(self, user) -> dict:
        return requests.post(f"{self.base_path}/register", json=user).json()

    def login_user(self, user) -> dict:
        return requests.post(f"{self.base_path}/login", json=user).json()


@given("user is registered and logged in")
def step_impl(context):
    context.existing_users = []
    user = {
        'username': 'username',
        'email': 'email@gmail.com',
        'password': 'passwd123',
    }
    user_client = TestUsersClient(context.api_url)
    context.existing_users.append(user_client.register_user(user))
    context.auth_token = user_client.login_user(user)["accessToken"]


@given("users exists")
def step_impl(context):
    created_users = []
    user_client = TestUsersClient(context.api_url)
    for row in context.table:
        request_body = {}
        for key, value in row.items():
            request_body[key] = value
        created_users.append(user_client.register_user(request_body))

    context.existing_users = created_users
