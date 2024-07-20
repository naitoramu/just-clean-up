import requests


def test_get_all_users(api_server):
    response = requests.get(f"{api_server}/just-clean-up/v1/users")
    assert response.status_code == 200
    users = response.json()
    assert len(users) == 0


def test_create_user(api_server):
    request_body = {
        'username': "test-username",
        'email': "test-email@gmail.com",
        'password': "test-password",
    }

    response = requests.post(f"{api_server}/just-clean-up/v1/users", json=request_body)

    assert response.status_code == 201
    created_user = response.json()
    assert created_user['id'] is not None
    assert created_user['username'] == request_body['username']
    assert created_user['email'] == request_body['email']
    assert created_user['password'] == request_body['password']
