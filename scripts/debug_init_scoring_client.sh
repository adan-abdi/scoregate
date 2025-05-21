#!/bin/bash

echo "👉 Registering scoring client…"

curl -k -X POST https://scoringtest.credable.io/api/v1/client/createClient \
  -H "Content-Type: application/json" \
  -d '{
    "url": "http://localhost:3000/client/transaction-data",
    "name": "scoregate",
    "username": "admin",
    "password": "pwd123"
  }' \
  -i \
  --silent --show-error > register_response.txt

echo "✅ Response saved to register_response.txt"
