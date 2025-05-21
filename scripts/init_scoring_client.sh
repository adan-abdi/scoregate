#!/usr/bin/env bash
set -euo pipefail

CLIENT_REG="https://scoringtest.credable.io/api/v1/client/createClient"
SCORING_BASE="https://scoringtest.credable.io/api/v1/scoring"
LOCAL_TXN_URL="http://localhost:3000/client/transaction-data"
USER="admin"
PASS="pwd123"
CUSTOMER="234774784"

echo "👉 Registering scoring client…"
RESP=$(curl -ks -X POST "$CLIENT_REG" \
  -H "Content-Type: application/json" \
  -d "{
    \"url\": \"$LOCAL_TXN_URL\",
    \"name\": \"scoregate\",
    \"username\": \"$USER\",
    \"password\": \"$PASS\"
  }")

TOKEN=$(echo "$RESP" | jq -r .token)
if [[ -z "$TOKEN" || "$TOKEN" == "null" ]]; then
  echo "❌ No token in response. Response was:"
  echo "$RESP"
  exit 1
fi
echo "🎉 Your client-token is: $TOKEN"

echo
echo "👉 Initiating score for customer $CUSTOMER…"
curl -k -i -X GET "$SCORING_BASE/initiateQueryScore/$CUSTOMER" \
     -H "client-token: $TOKEN"

echo
echo "👉 Querying the score…"
curl -k -i -X GET "$SCORING_BASE/queryScore/$TOKEN" \
     -H "client-token: $TOKEN"
