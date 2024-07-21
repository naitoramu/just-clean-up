import requests
from behave import given, when, then


@given("request body")
def step_impl(context):
    data = context.table[0]
    context.request_body = {
        'username': data['username'],
        'email': data['email'],
        'password': data['password'],
    }


@when("make {method} request to {path}")
def step_impl(context, method, path):
    url = f"{context.api_url}/just-clean-up{path}"
    match method:
        case "GET":
            context.response = requests.get(url)
        case "POST":
            context.response = requests.post(url, json=context.request_body)
        case "POST":
            context.response = requests.put(url, json=context.request_body)
        case "DELETE":
            context.response = requests.delete(url)
        case _:
            raise Exception(f"Request with HTTP method '{method}' not implemented")


@then("the response status code should be {status_code:d}")
def step_impl(context, status_code):
    assert context.response.status_code == status_code


@then("the response body should be a list with {count:d} elements")
def step_impl(context, count):
    response_body = context.response.json()
    assert isinstance(response_body, list)
    assert len(response_body) == count


@then("the response should contain request body details")
def step_impl(context):
    request_body = context.request_body
    response_body = context.response.json()
    for key, value in request_body.items():
        assert response_body[key] == value


@then("the response should contain not null field '{field}'")
def step_impl(context, field):
    assert context.response.json()[field] is not None
