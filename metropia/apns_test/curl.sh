#!/bin/sh
base_url="https://api.sandbox.push.apple.com"
device="39065c1da26c4c33277e3a06ede8370b1d320205f67f26eaa7ae158b8965fa8a"
bearer="eyJhbGciOiJFUzI1NiIsImtpZCI6IlA3REFCOU5DOU0ifQ.eyJpc3MiOiI2NUJKMjQzQ0tBIiwiaWF0IjoxNjU2MjQ4NjE0fQ.yHn9m21RO9C_EPepOXNbOC9YvRTlVAtpIw4QBcJE6ruYwL0mI6jHct2eDc9hLggq4jkMd7pish2jMjkNKgHC_g"
topic="com.connectsmart.carpool.transit.gps.directions.rideshare.navigate"
content_type="Content-Type: application/json"
authorization="Authorization: Bearer ${bearer}"
apns_topic="apns-topic: ${topic}"
url="${base_url}/3/device/${device}"

# curl -s -v -i --http2 -H "${apns_topic}" -H "${authorization}" -H "${content_type}" -d "{ \"aps\" : { \"alert\" : \"Hello\" } }" $url

curl -s -v -i -H "${apns_topic}" -H "${authorization}" -H "${content_type}" -d "{ \"aps\" : { \"alert\" : \"Hello\" } }" $url

