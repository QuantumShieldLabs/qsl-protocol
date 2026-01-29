#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
qsc_demo_local.sh --scenario <name> [--seed <u64>] [--out <dir>] [--dry-run]

Scenarios: happy-path, drop, reorder, drop+reorder, seeded-replay

Notes:
- Dry-run prints the exact commands that would run (no network, no execution).
- Marker logs are safe to share (QSC_MARK lines only).
USAGE
}

seed=1
scenario="happy-path"
out=""
dry_run=0

while [ $# -gt 0 ]; do
  case "$1" in
    --help|-h)
      usage; exit 0;;
    --seed)
      seed="$2"; shift 2;;
    --scenario)
      scenario="$2"; shift 2;;
    --out)
      out="$2"; shift 2;;
    --dry-run)
      dry_run=1; shift;;
    *)
      echo "Unknown arg: $1" >&2
      usage; exit 2;;
  esac
done

case "$scenario" in
  happy-path|drop|reorder|drop+reorder|drop-reorder|seeded-replay) ;;
  *) echo "Invalid scenario: $scenario" >&2; exit 2;;
esac

if [ -z "$out" ]; then
  ts=$(date -u +%Y%m%dT%H%M%SZ)
  out="./_demo_out/${ts}"
fi

mkdir -p "$out"

# Map scenario to relay knobs (deterministic; seed controls behavior)
# No secrets are printed; marker logs are filtered to QSC_MARK only.
case "$scenario" in
  happy-path)
    drop=0; dup=0; reorder=0; latency=0; jitter=0;;
  drop)
    drop=25; dup=0; reorder=0; latency=0; jitter=0;;
  reorder)
    drop=0; dup=0; reorder=3; latency=0; jitter=0;;
  drop+reorder|drop-reorder)
    drop=25; dup=0; reorder=3; latency=0; jitter=0;;
  seeded-replay)
    drop=10; dup=10; reorder=2; latency=0; jitter=0;;
esac

relay_cmd=(cargo run -p qsc -- relay serve \
  --seed "$seed" \
  --drop-pct "$drop" \
  --dup-pct "$dup" \
  --reorder-window "$reorder" \
  --fixed-latency-ms "$latency" \
  --jitter-ms "$jitter")

alice_cmd=(cargo run -p qsc -- relay send --to bob --file ./_demo_payloads/alice_to_bob.txt --relay http://127.0.0.1:9123)

bob_cmd=(cargo run -p qsc -- relay send --to alice --file ./_demo_payloads/bob_to_alice.txt --relay http://127.0.0.1:9123)

if [ "$dry_run" -eq 1 ]; then
  echo "DRY-RUN: ${relay_cmd[*]}"
  echo "DRY-RUN: ${alice_cmd[*]}"
  echo "DRY-RUN: ${bob_cmd[*]}"
  echo "OUT_DIR=$out"
  exit 0
fi

# Safety: marker logs are filtered to QSC_MARK lines only.
# This script does not print secrets; outputs are safe to share.

relay_log="$out/relay.log"
alice_log="$out/alice.log"
bob_log="$out/bob.log"

mkdir -p ./_demo_payloads
printf 'hello from alice\n' > ./_demo_payloads/alice_to_bob.txt
printf 'hello from bob\n' > ./_demo_payloads/bob_to_alice.txt

("${relay_cmd[@]}") >"$relay_log" 2>&1 &
relay_pid=$!
trap 'kill "$relay_pid" >/dev/null 2>&1 || true' EXIT

sleep 1
("${alice_cmd[@]}") >"$alice_log" 2>&1 || true
("${bob_cmd[@]}") >"$bob_log" 2>&1 || true

# Extract markers only
rg -n '^QSC_MARK' "$relay_log" > "$out/relay.markers" || true
rg -n '^QSC_MARK' "$alice_log" > "$out/alice.markers" || true
rg -n '^QSC_MARK' "$bob_log" > "$out/bob.markers" || true

echo "DEMO DONE"
echo "scenario=$scenario seed=$seed"
echo "out=$out"
