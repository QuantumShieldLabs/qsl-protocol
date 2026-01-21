#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_DIR"

TS="$(date -u +%Y%m%dT%H%M%SZ)"
OUT="${OUT_DIR:-${REPO_DIR}/_forensics/demo_two_party_local_${TS}}"
mkdir -p "$OUT"

CHANNEL="demo-local-${TS}"
RECEIVER_LOG="${OUT}/receiver.log"
SENDER_LOG="${OUT}/sender.log"

echo "OUT=${OUT}"
echo "CHANNEL=${CHANNEL}"

set +e
timeout 60s cargo run -p qsl-tui --release -- \
  --headless --role receiver --mode local \
  --relay-channel "${CHANNEL}" \
  --privacy-mode padded \
  >"${RECEIVER_LOG}" 2>&1 &
RECV_PID=$!
set -e

sleep 1

timeout 60s cargo run -p qsl-tui --release -- \
  --headless --role sender --mode local \
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
