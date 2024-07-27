Feature: Authentication

  Scenario: Successful login
    Given users exists
      | username      | email             | password      |
      | test-username | email-1@gmail.com | test-password |
    And request body
      | email             | password      |
      | email-1@gmail.com | test-password |
    When make POST request to '/login'
    Then the response status code should be 200
    And the response should contain not null field 'accessToken'

  Scenario: Invalid email
    Given users exists
      | username      | email             | password      |
      | test-username | email-1@gmail.com | test-password |
    And request body
      | email                   | password      |
      | invalid-email@gmail.com | test-password |
    When make POST request to '/login'
    Then the response status code should be 401
    And the response should contain field 'title' with value 'Unauthorized'
    And the error detail should be 'Invalid credentials'

  Scenario: Invalid password
    Given users exists
      | username      | email             | password      |
      | test-username | email-1@gmail.com | test-password |
    And request body
      | email             | password         |
      | email-1@gmail.com | invalid-password |
    When make POST request to '/login'
    Then the response status code should be 401
    And the response should contain field 'title' with value 'Unauthorized'
    And the error detail should be 'Invalid credentials'

  Scenario: Missing email
    Given users exists
      | username      | email             | password      |
      | test-username | email-1@gmail.com | test-password |
    And request body
      | password         |
      | invalid-password |
    When make POST request to '/login'
    Then the response status code should be 400
    And the response should contain field 'title' with value 'Bad request'
    And the error detail should contain 'missing field `email`'

  Scenario: Missing password
    Given users exists
      | username      | email             | password      |
      | test-username | email-1@gmail.com | test-password |
    And request body
      | email             |
      | email-1@gmail.com |
    When make POST request to '/login'
    Then the response status code should be 400
    And the response should contain field 'title' with value 'Bad request'
    And the error detail should contain 'missing field `password`'
