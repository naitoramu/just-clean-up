from behave import then

@then("the error detail should contain '{message}'")
def step_impl(context, message):
    actual_detail: str = context.response.json()["detail"]
    assert message in actual_detail, \
        f"Expected actual '{actual_detail}' to contain '{message}'"


@then("the error detail should be '{exp_detail}'")
def step_impl(context, exp_detail):
    actual_detail: str = context.response.json()["detail"]
    assert actual_detail == exp_detail, \
        f"Expected actual '{actual_detail}' to be '{exp_detail}'"
