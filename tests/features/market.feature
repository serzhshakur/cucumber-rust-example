Feature: Public Market API

  Scenario: Current system time is successfully returned
    When server time is retrieved
    Then both unixtime and rfc1123 are returned in UTC format

  Scenario: An asset pair is successfully returned
    Given client chooses asset pair "XXBT/ZUSD"
    When an asset pair is queried
    Then a valid info about asset pair is returned
