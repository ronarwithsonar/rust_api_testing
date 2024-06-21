Feature: Authenicated User Enquiries

    Scenario: I see no open orders when I have no orders
        Given I am authenticated
        When I have no open orders
        Then I should see no orders

    Scenario: I see my open orders when I have orders
        Given I am authenticated
        When I have open orders
        Then I should see my open orders