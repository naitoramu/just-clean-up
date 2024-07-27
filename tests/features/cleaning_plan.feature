Feature: Cleaning Plan

  Scenario: Unauthorized requests produces 401
    When make requests
      | method | path                       |
      | POST   | /v1/cleaning-plans         |
      | GET    | /v1/cleaning-plans/plan-id |
      | PUT    | /v1/cleaning-plans/plan-id |
      | DELETE | /v1/cleaning-plans/plan-id |
    Then each response status code should be 401

  Scenario: Successful cleaning plan creation
    Given user is registered and logged in
    And cleaning plan
      | title               | address          | startDate |
      | Cleaning plan title | Location address | 23149834  |
    And cleaning plan duties
      | title     | description      | repetition | offset | penalty                         |
      | Duty tile | Duty description | 1W         | 2D     | Penalty for not completing duty |
    And cleaning plan cleaner identifiers with existing users
    And request body with cleaning plan
    When make POST request to '/v1/cleaning-plans'
    Then the response status code should be 201
    And the response should contain not null field 'id'
    And the response should contain request body details

#  Scenario: Invalid password
#    Given users exists
#      | username      | email             | password      |
#      | test-username | email-1@gmail.com | test-password |
#    And request body
#      | email             | password         |
#      | email-1@gmail.com | invalid-password |
#    When make POST request to '/login'
#    Then the response status code should be 401
#    And the response should contain field 'title' with value 'Unauthorized'
#    And the error detail should be 'Invalid credentials'
#
#  Scenario: Missing email
#    Given users exists
#      | username      | email             | password      |
#      | test-username | email-1@gmail.com | test-password |
#    And request body
#      | password         |
#      | invalid-password |
#    When make POST request to '/login'
#    Then the response status code should be 400
#    And the response should contain field 'title' with value 'Bad request'
#    And the error detail should contain 'missing field `email`'
#
#  Scenario: Missing password
#    Given users exists
#      | username      | email             | password      |
#      | test-username | email-1@gmail.com | test-password |
#    And request body
#      | email             |
#      | email-1@gmail.com |
#    When make POST request to '/login'
#    Then the response status code should be 400
#    And the response should contain field 'title' with value 'Bad request'
#    And the error detail should contain 'missing field `password`'
