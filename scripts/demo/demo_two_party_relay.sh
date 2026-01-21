#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_DIR"

TS="$(date -u +%Y%m%dT%H%M%SZ)"
OUT="${OUT_DIR:-${REPO_DIR}/_forensics/demo_two_party_relay_${TS}}"
mkdir -p "$OUT"

BASE_URL="${QSL_RELAY_BASE_URL:-http://qsl.ddnsfree.com:8080}"
CHANNEL="demo-relay-${TS}"
RECEIVER_LOG="${OUT}/receiver.log"
SENDER_LOG="${OUT}/sender.log"

export QSL_ALLOW_REMOTE=1

echo "OUT=${OUT}"
echo "BASE_URL=${BASE_URL}"
echo "CHANNEL=${CHANNEL}"

set +e
timeout 60s cargo run -p qsl-tui --release -- \
  --headless --role receiver --mode relay \
  --relay-base-url "${BASE_URL}" \
  --relay-channel "${CHANNEL}" \
  --privacy-mode padded \
  >"${RECEIVER_LOG}" 2>&1 &
RECV_PID=$!
set -e

sleep 1

timeout 60s cargo run -p qsl-tui --release -- \
  --headless --role sender --mode relay \
  --relay-base-url "${BASE_URL}" \
  --relay-channel "${CHANNEL}" \
  --privacy-mode padded \
  --message "hello" \
  >"${SENDER_LOG}" 2>&1

wait "${RECV_PID}"

echo "DONE. Logs:"
echo "  ${RECEIVER_LOG}"
echo "  ${SENDER_LOG}"
echo "COPY/PASTE:"
echo "  sed -n '1,200p' ${RECEIVER_LOG}"
echo "  sed -n '1,200p' ${SENDER_LOG}"
