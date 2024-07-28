import requests
from behave import given


class TestUsersClient:
    def __init__(self, url):
        self.base_path = f"{url}/just-clean-up/v1"

    def create_user(self, user) -> dict:
        return requests.post(f"{self.base_path}/users", json=user).json()


@given("cleaning plan")
def step_impl(context):
    context.cleaning_plan = {}
    for key, value in context.table[0].items():
        if key == 'startDate':
            context.cleaning_plan[key] = int(value)
        else:
            context.cleaning_plan[key] = value


@given("cleaning plan duties")
def step_impl(context):
    context.cleaning_plan['duties'] = []
    for row in context.table:
        duty = {}
        for key, value in row.items():
            duty[key] = value

        context.cleaning_plan['duties'].append(duty)


@given("cleaning plan cleaner identifiers with existing users")
def step_impl(context):
    context.cleaning_plan['cleanerIds'] = []
    for user in context.existing_users:
        context.cleaning_plan['cleanerIds'].append(user['id'])


@given("request body with cleaning plan")
def step_impl(context):
    context.request_body = context.cleaning_plan
