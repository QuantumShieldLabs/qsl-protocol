#!/usr/bin/env bash
set -euo pipefail

# DEMO-PUBLIC-001 helper: build + run metadata visibility demo
# Output artifacts go to /home/victor/work/qsl/_forensics

REPO_DIR="/home/victor/work/qsl/qsl-protocol"
cd "$REPO_DIR"

RUN_ID="$(date -u +%Y%m%dT%H%M%SZ)"
OUT="/home/victor/work/qsl/_forensics/demo_public_meta_${RUN_ID}"
mkdir -p "$OUT"

{
  echo "RUN_ID=$RUN_ID"
  echo "HEAD=$(git rev-parse HEAD)"
  echo "START_UTC=$(date -u)"
} | tee "$OUT/00_header.txt"

# Local mode (basic)
set +e
cargo run -p qsl-tui --release -- \
  demo --privacy basic --mode local --message "hello" \
  >"$OUT/10_local_basic.txt" 2>&1
RC_LOCAL=$?
set -e

echo "RC_LOCAL=$RC_LOCAL" | tee -a "$OUT/10_local_basic.txt"

# Relay mode (padded) requires explicit opt-in
BASE="http://qsl.ddnsfree.com:8080"
CH="demo-${RUN_ID}"
set +e
QSL_ALLOW_REMOTE=1 cargo run -p qsl-tui --release -- \
  demo --privacy padded --mode relay \
  --relay-base-url "$BASE" --relay-channel "$CH" \
  >"$OUT/20_relay_padded.txt" 2>&1
RC_RELAY=$?
set -e

echo "RC_RELAY=$RC_RELAY" | tee -a "$OUT/20_relay_padded.txt"

# Quick relay sanity
{
  echo "BASE=$BASE"
  echo "CH=$CH"
  echo "--- GET pull (expect 204) ---"
  curl -sS -D- -o /dev/null "$BASE/v1/pull/$CH" | sed -n '1,25p'
} >"$OUT/30_relay_sanity.txt" 2>&1

echo "OUT=$OUT"
ls -lah "$OUT" | sed -n '1,200p'
