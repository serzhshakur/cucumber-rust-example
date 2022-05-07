Feature: User API

  Scenario: quering for a user open orders
    When I query open oreders
    Then number of open orders is 0

