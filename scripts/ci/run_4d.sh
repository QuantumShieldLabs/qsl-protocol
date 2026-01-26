#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

PHASE2_ZIP_DEFAULT="$ROOT/inputs/phase2/QuantumShield_Phase2_CANONICAL_FROZEN_QSP4.3.2_QSE1.8.2.zip"
PHASE3_ZIP_DEFAULT="$ROOT/inputs/phase3/QuantumShield_Phase3_SUPPORTING_COMPLETE_P3-02_to_P3-30.zip"

PHASE2_ZIP="${PHASE2_ZIP:-$PHASE2_ZIP_DEFAULT}"
PHASE3_ZIP="${PHASE3_ZIP:-$PHASE3_ZIP_DEFAULT}"

# Actor configuration: default to CI config if present, else local config.
ACTORS_CONFIG_DEFAULT="$ROOT/tests/harness/4b/actors.ci.json"
if [[ ! -f "$ACTORS_CONFIG_DEFAULT" ]]; then
  ACTORS_CONFIG_DEFAULT="$ROOT/tests/harness/4b/actors.local.json"
fi
ACTORS_CONFIG="${ACTORS_CONFIG:-$ACTORS_CONFIG_DEFAULT}"

GIT_COMMIT="$(git -C "$ROOT" rev-parse --short=12 HEAD 2>/dev/null || echo "nogit")"
RUN_ID="${RUN_ID:-$(date -u +%Y%m%dT%H%M%SZ)_${GIT_COMMIT}}"

ART_DIR="$ROOT/artifacts/$RUN_ID/4D"
EVI_DIR="$ROOT/evidence/4D_runs/$RUN_ID/4D"

mkdir -p "$ART_DIR" "$EVI_DIR"

install_refimpl_actor() {
  # The harness expects ./tools/actors/refimpl_actor (gitignored). CI starts from a clean checkout,
  # so we must install the freshly built binary there.
  local src="$ROOT/target/release/refimpl_actor"
  local dst="$ROOT/tools/actors/refimpl_actor"

  if [[ ! -f "$src" ]]; then
    echo "[4D] ERROR: expected actor binary not found at $src" >&2
    return 2
  fi

  mkdir -p "$ROOT/tools/actors"
  cp "$src" "$dst"
  chmod +x "$dst"
}

build_refimpl_actor() {
  if ! command -v cargo >/dev/null 2>&1; then
    echo "[4D] ERROR: cargo not found; Rust toolchain is required for refimpl actors." >&2
    return 2
  fi

  echo "[4D] building Rust refimpl_actor (release)" >&2
  (cd "$ROOT" && cargo build --release -p refimpl_actor --locked)
  install_refimpl_actor
}

# Build the actor binary up front so harness execution is fail-closed if build breaks.
build_refimpl_actor

STUB_MODE=""
STUB_PIDS=()

start_inprocess_stubs() {
  echo "[4D] docker compose not available; starting in-process stubs on 18080/18081/18082" >&2
  python3 "$ROOT/tests/harness/4b/stubs/stub_server.py" --service rsf --port 18080 >"$ART_DIR/stub_rsf.log" 2>&1 &
  STUB_PIDS+=("$!")
  python3 "$ROOT/tests/harness/4b/stubs/stub_server.py" --service pds --port 18081 >"$ART_DIR/stub_pds.log" 2>&1 &
  STUB_PIDS+=("$!")
  python3 "$ROOT/tests/harness/4b/stubs/stub_server.py" --service ktl --port 18082 >"$ART_DIR/stub_ktl.log" 2>&1 &
  STUB_PIDS+=("$!")
}

stop_inprocess_stubs() {
  if ((${#STUB_PIDS[@]} > 0)); then
    for pid in "${STUB_PIDS[@]}"; do
      kill "$pid" >/dev/null 2>&1 || true
    done
    for pid in "${STUB_PIDS[@]}"; do
      wait "$pid" >/dev/null 2>&1 || true
    done
  fi
}

compose_up() {
  if command -v docker >/dev/null 2>&1 && docker compose version >/dev/null 2>&1; then
    STUB_MODE="compose"
    docker compose -f "$ROOT/tests/harness/4b/docker-compose.yml" up -d --build
  else
    STUB_MODE="inprocess"
    start_inprocess_stubs
  fi
}

compose_down() {
  if [[ "${STUB_MODE:-}" == "compose" ]]; then
    docker compose -f "$ROOT/tests/harness/4b/docker-compose.yml" down -v --remove-orphans || true
  elif [[ "${STUB_MODE:-}" == "inprocess" ]]; then
    stop_inprocess_stubs
  fi
}

finalize_retention() {
  rm -rf "$EVI_DIR"/*
  cp -a "$ART_DIR"/. "$EVI_DIR"/
}

on_exit() {
  code="$?"
  set +e
  compose_down

  # best-effort manifest/retention summaries (do not overwrite original exit code)
  python3 "$ROOT/tests/harness/4b/runner.py" manifest \
    --artifacts "$ART_DIR" \
    --out "$ART_DIR" \
    --run-id "$RUN_ID" \
    --git-commit "$GIT_COMMIT" >/dev/null 2>&1 || true

  python3 "$ROOT/tests/harness/4b/runner.py" retention \
    --artifacts "$ART_DIR" \
    --evidence "$EVI_DIR" \
    --out "$ART_DIR" \
    --run-id "$RUN_ID" \
    --git-commit "$GIT_COMMIT" >/dev/null 2>&1 || true

  finalize_retention || true
  exit "$code"
}
trap on_exit EXIT

echo "[4D] Run ID: $RUN_ID"
echo "[4D] Artifacts: $ART_DIR"
echo "[4D] Evidence:   $EVI_DIR"
echo "[4D] Actors:     $ACTORS_CONFIG"

compose_up

set +e

python3 "$ROOT/tests/harness/4b/runner.py" preflight \
  --phase2-zip "$PHASE2_ZIP" \
  --phase3-zip "$PHASE3_ZIP" \
  --actors "$ACTORS_CONFIG" \
  --out "$ART_DIR" \
  --run-id "$RUN_ID" \
  --git-commit "$GIT_COMMIT" 2>&1 | tee "$ART_DIR/B0_preflight.log"
rc_B0=${PIPESTATUS[0]}

python3 "$ROOT/tests/harness/4b/runner.py" interop \
  --phase3-zip "$PHASE3_ZIP" \
  --phase2-zip "$PHASE2_ZIP" \
  --actors "$ACTORS_CONFIG" \
  --out "$ART_DIR" \
  --run-id "$RUN_ID" \
  --git-commit "$GIT_COMMIT" \
  --profile security \
  --out-name "D1_interop_security.json" 2>&1 | tee "$ART_DIR/D1_interop_security.log"
rc_D1=${PIPESTATUS[0]}

set -e

if [[ "${rc_B0:-0}" -ne 0 || "${rc_D1:-0}" -ne 0 ]]; then
  echo "[4D] One or more stages failed (preflight=$rc_B0 interop_security=$rc_D1)" >&2
  exit 1
fi

echo "[4D] Complete."
