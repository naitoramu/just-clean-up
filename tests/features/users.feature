Feature: User management

  Scenario: Get all users when no users exist
    When make GET request to /v1/users
    Then the response status code should be 200
    And the response body should be a list with 0 elements

  Scenario: Get all users
    Given users exists
      | username  | email             | password  |
      | username1 | email-1@gmail.com | password1 |
      | username2 | email-2@gmail.com | password2 |
    When make GET request to /v1/users
    Then the response status code should be 200
    And the response body should be a list with 2 elements

  Scenario: Create a new user
    Given request body
      | username  | email             | password  |
      | username1 | email-1@gmail.com | password1 |
    When make POST request to /v1/users
    Then the response status code should be 201
    And the response should contain not null field 'id'
    And the response should contain request body details
