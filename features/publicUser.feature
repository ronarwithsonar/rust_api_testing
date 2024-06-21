Feature: Public User endpoint

    Background:
        Given I am a public user
    
    Scenario: Request the server time
        When I request the server time
        Then I should receive the current server time

    Scenario Outline: Currency Enquiry
        When I request information on "<ccyPair>"
        Then I should receive information on "<ccyPair>"
        
        Examples:
            | ccyPair |
            | XBTUSD  |