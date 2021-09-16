Feature: Test an Api feature
    
  Scenario: Get valid response from time server scenario
    Given We have the correct server time endpoint
    When We call the server for time
    Then We should get back a properly formed response with both rfc1123 and unixtime properties
    
  Scenario: Expect error response for invalid time endpoint scenario
    Given We have the incorrect server time endpoint
    When We call the server for time
    Then We expect errors in the time response
    
  Scenario: Expect valid response from ticker api scenario
    Given We have the correct ticker endpoint
    When We call the server ticker info with valid trading pair
    Then We should get back a properly formed ticker info response

  Scenario: Expect error response for invalid pair scenario
    Given We have the correct ticker endpoint
    When We call the ticker info with an invalid trading pair
    Then We expect errors in ticker response
   
  Scenario: Expect valid response from open trades api scenario
    Given We have api keys
    Then We should get back a properly formed response without errors

  Scenario: Expect invalid response from open trades when nonce reused
    Given We have api keys
    Then We should get error when request uses bad nonce
