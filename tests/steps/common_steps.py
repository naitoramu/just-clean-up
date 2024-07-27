import requests
from behave import given, when, then
from requests import Response


@given("request body")
def step_impl(context):
    data = {}
    for key, value in context.table[0].items():
        data[key] = value

    context.request_body = data


@when("make {method} request to '{path}'")
def step_impl(context, method, path):
    url = f"{context.api_url}/just-clean-up{path}"
    auth_token = context.auth_token
    context.response = make_request(method, url, auth_token, context.request_body)


@when("make requests")
def step_impl(context):
    context.responses = []
    auth_token = context.auth_token
    for row in context.table:
        url = f"{context.api_url}/just-clean-up{row['path']}"
        context.responses.append(make_request(row['method'], url, auth_token))


def make_request(method: str, url: str, auth_token: str, body: dict = None) -> Response:
    print("Auth token:", auth_token)
    headers = {
        "Authorization": auth_token
    }
    match method:
        case "GET":
            return requests.get(url, headers=headers)
        case "POST":
            return requests.post(url, json=body, headers=headers)
        case "PUT":
            return requests.put(url, json=body, headers=headers)
        case "DELETE":
            return requests.delete(url, headers=headers)
        case _:
            raise Exception(f"Request with HTTP method '{method}' not implemented")


@then("the response status code should be {status_code:d}")
def step_impl(context, status_code):
    asser_actual_eq_expected(context.response.status_code, status_code)


@then("each response status code should be {status_code:d}")
def step_impl(context, status_code):
    for response in context.responses:
        asser_actual_eq_expected(response.status_code, status_code)


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


@then("the response should contain field '{field}' with value '{value}'")
def step_impl(context, field, value):
    asser_actual_eq_expected(context.response.json()[field], value)


def asser_actual_eq_expected(actual, expected) -> None :
    assert actual == expected, \
        f"Expected actual '{actual}' to be '{expected}'"
