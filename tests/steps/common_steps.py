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
        "Authorization": f"Bearer {auth_token}"
    }
    print("Headers:", headers)
    match method:
        case "GET":
            response = requests.get(url, headers=headers)
        case "POST":
            response = requests.post(url, json=body, headers=headers)
        case "PUT":
            response = requests.put(url, json=body, headers=headers)
        case "DELETE":
            response = requests.delete(url, headers=headers)
        case _:
            raise Exception(f"Request with HTTP method '{method}' not implemented")
    print("Response:", response.json())
    return response


@then("the response status code should be {status_code:d}")
def step_impl(context, status_code):
    assert_actual_eq_expected(context.response.status_code, status_code)


@then("each response status code should be {status_code:d}")
def step_impl(context, status_code):
    for response in context.responses:
        assert_actual_eq_expected(response.status_code, status_code)


@then("the response body should be a list with {count:d} elements")
def step_impl(context, count):
    response_body = context.response.json()
    assert isinstance(response_body, list)
    assert len(response_body) == count


@then("the response should contain request body properties")
def step_impl(context):
    request_body = context.request_body
    response_body = context.response.json()
    assert_actual_eq_expected(response_body, request_body)


@then("the response should contain not null field '{field}'")
def step_impl(context, field):
    assert context.response.json()[field] is not None


@then("the response should contain field '{field}' with value '{value}'")
def step_impl(context, field, value):
    assert_actual_eq_expected(context.response.json()[field], value)


def assert_actual_list_contains_expected_list_elements(actual, expected):
    for exp_element, act_element in zip(expected, actual):
        if isinstance(exp_element, dict):
            assert_actual_dict_contains_expected_dict_fields(act_element, exp_element)
        elif isinstance(exp_element, list):
            assert_actual_list_contains_expected_list_elements(act_element, exp_element)
        else:
            assert exp_element in actual, f"Expected actual list: {actual} to contain '{exp_element}'"


def assert_actual_eq_expected(actual, expected) -> None:
    if isinstance(expected, dict):
        assert_actual_dict_contains_expected_dict_fields(actual, expected)
    elif isinstance(expected, list):
        assert_actual_list_contains_expected_list_elements(actual, expected)
    else:
        assert_actual_var_eq_expected_var(actual, expected)


def assert_actual_dict_contains_expected_dict_fields(actual, expected):
    for key, value in expected.items():
        print(type(value))
        if isinstance(value, dict):
            assert_actual_dict_contains_expected_dict_fields(actual[key], value)
        else:
            assert_actual_eq_expected(actual[key], value)


def assert_actual_var_eq_expected_var(actual, expected):
    assert actual == expected, \
        f"Expected actual '{actual}' to be '{expected}'"
