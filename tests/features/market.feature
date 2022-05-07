Feature: Public Market API

  Scenario: Current system time is successfully returned
    When server time is retrieved
    Then both unixtime and rfc1123 are returned in UTC format


