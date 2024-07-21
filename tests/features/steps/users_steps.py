import requests
from behave import given


class TestUsersClient:
    def __init__(self, url):
        self.base_path = f"{url}/just-clean-up/v1"

    def create_user(self, user):
        requests.post(f"{self.base_path}/users", json=user)


@given("Users exists")
def step_impl(context):
    user_client = TestUsersClient(context.api_url)
    for row in context.table:
        request_body = {}
        for key, value in row.items():
            request_body[key] = value
        user_client.create_user(request_body)