#!/usr/bin/env bash
# scripts/test_subscribe.sh

BASE_URL=${BASE_URL:-http://localhost:3000}


# Automated edge-case tests for POST /subscribe
echo ""
echo "=== 1) Happy Path: 201 Created ==="
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: application/json" \
     -d '{"customer_number":"234774784"}'
echo -e "\n"

echo "=== 2) Empty customer_number: 400 Bad Request ==="
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: application/json" \
     -d '{"customer_number":""}'
echo -e "\n"

echo "=== 3) Missing field: 400 Bad Request ==="
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: application/json" \
     -d '{}'
echo -e "\n"

echo "=== 4) Malformed JSON: 400 Bad Request ==="
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: application/json" \
     -d 'not a valid JSON'
echo -e "\n"

echo "=== 5) No Content-Type header: likely 400 Bad Request ==="
curl -i -X POST "$BASE_URL/subscribe" \
     -d '{"customer_number":"234774784"}'
echo -e "\n"

echo "=== 6) Wrong Content-Type: 415 Unsupported Media Type (or 400) ==="
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: text/plain" \
     -d '{"customer_number":"234774784"}'
echo -e "\n"

echo "=== 7) Duplicate subscription: 409 Conflict ==="
echo "-- First call --"
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: application/json" \
     -d '{"customer_number":"318411216"}'
echo -e "\n"
echo "-- Second call --"
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: application/json" \
     -d '{"customer_number":"318411216"}'
echo -e "\n"

echo "=== 8) Wrong HTTP method: 404 Not Found / 405 Method Not Allowed ==="
curl -i -X GET "$BASE_URL/subscribe"
echo -e "\n"
