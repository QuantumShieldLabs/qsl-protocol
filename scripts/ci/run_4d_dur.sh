#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

GIT_COMMIT="$(git rev-parse --short=12 HEAD 2>/dev/null || echo nogit)"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
RUN_ID="${TS}_${GIT_COMMIT}"

OUT="$ROOT/artifacts/$RUN_ID/4D"
EVID="$ROOT/evidence/4D_runs/$RUN_ID/4D"
mkdir -p "$OUT" "$EVID"

echo "[4D-DUR] Run ID: $RUN_ID"
echo "[4D-DUR] Artifacts: $OUT"
echo "[4D-DUR] Evidence:   $EVID"

# Build + install actor (CI checkouts won't have the prebuilt binary).
echo "[4D-DUR] building Rust refimpl_actor (release)"
(
  cd "$ROOT" && cargo build --release -p refimpl_actor
)

# In this repo, Cargo builds workspace binaries into $ROOT/target by default.
# Some local configurations may place targets under the crate directory; accept either.
BIN_SRC="$ROOT/target/release/refimpl_actor"
ALT_SRC="$ROOT/tools/actors/refimpl_actor_rs/target/release/refimpl_actor"
if [[ ! -x "$BIN_SRC" && -x "$ALT_SRC" ]]; then
  BIN_SRC="$ALT_SRC"
fi

BIN_DST="$ROOT/tools/actors/refimpl_actor"
if [[ ! -x "$BIN_SRC" ]]; then
  echo "[4D-DUR] ERROR: expected actor binary not found at $BIN_SRC" >&2
  echo "[4D-DUR] HINT: checked $ROOT/target/release/refimpl_actor and $ALT_SRC" >&2
  exit 1
fi

mkdir -p "$(dirname "$BIN_DST")"
cp -f "$BIN_SRC" "$BIN_DST"
chmod +x "$BIN_DST"

# Durability tests require explicit opt-in for debug hooks.
export QSL_TEST_HOOKS=1

python3 scripts/ci/durability_4d.py \
  --out "$OUT" \
  --evidence "$EVID" \
  --git-commit "$GIT_COMMIT" \
  2>&1 | tee "$EVID/D2_durability.log"

python3 scripts/ci/assert_4d_dur_ok.py

echo "[4D-DUR] Complete."
