#!/usr/bin/env bash

BASE_URL=${BASE_URL:-http://localhost:3000}
CUSTOMER_VALID=234774784
AMOUNT_VALID=100.0

# Automated edge‐case tests for POST /loan-request
echo ""
echo "=== 1) No subscription: expect 404 Not Found ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: application/json" \
     -d "{\"customer_number\":\"$CUSTOMER_VALID\",\"amount\":$AMOUNT_VALID}"
echo -e "\n"

echo "=== 2) Invalid input: empty customer_number → 400 Bad Request ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: application/json" \
     -d "{\"customer_number\":\"\",\"amount\":$AMOUNT_VALID}"
echo -e "\n"

echo "=== 3) Invalid input: non‐positive amount → 400 Bad Request ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: application/json" \
     -d "{\"customer_number\":\"$CUSTOMER_VALID\",\"amount\":0}"
echo -e "\n"

echo "=== 4) Missing field: missing amount → 400 Bad Request ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: application/json" \
     -d "{\"customer_number\":\"$CUSTOMER_VALID\"}"
echo -e "\n"

echo "=== 5) Missing field: missing customer_number → 400 Bad Request ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: application/json" \
     -d "{\"amount\":$AMOUNT_VALID}"
echo -e "\n"

echo "=== 6) Malformed JSON → 400 Bad Request ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: application/json" \
     -d "not a valid JSON"
echo -e "\n"

echo "=== 7) No Content-Type header → 415 Unsupported Media Type ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -d "{\"customer_number\":\"$CUSTOMER_VALID\",\"amount\":$AMOUNT_VALID}"
echo -e "\n"

echo "=== 8) Wrong Content-Type → 415 Unsupported Media Type ==="
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: text/plain" \
     -d "{\"customer_number\":\"$CUSTOMER_VALID\",\"amount\":$AMOUNT_VALID}"
echo -e "\n"

echo "=== 9) Happy Path: 200 OK (after subscription) ==="
# ensure subscription
curl -i -X POST "$BASE_URL/subscribe" \
     -H "Content-Type: application/json" \
     -d "{\"customer_number\":\"$CUSTOMER_VALID\"}"
echo ""
# now request loan
curl -i -X POST "$BASE_URL/loan-request" \
     -H "Content-Type: application/json" \
     -d "{\"customer_number\":\"$CUSTOMER_VALID\",\"amount\":$AMOUNT_VALID}"
echo -e "\n"

echo "=== 10) Wrong HTTP method → 405 Method Not Allowed ==="
curl -i -X GET "$BASE_URL/loan-request"
echo -e "\n"
