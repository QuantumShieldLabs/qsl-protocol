#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

DEFAULT_RUNS=5
MAX_RUNS=10
RUNS="${DEMO_SOAK_RUNS:-$DEFAULT_RUNS}"
PROFILE="${DEMO_SOAK_PROFILE:-baseline}"
MAX_RUNTIME_S="${DEMO_SOAK_MAX_RUNTIME_S:-2700}"

case "$RUNS" in
  ""|*[!0-9]*)
    echo "unsupported DEMO_SOAK_RUNS: $RUNS" >&2
    exit 2
    ;;
esac
if [ "$RUNS" -lt 1 ] || [ "$RUNS" -gt "$MAX_RUNS" ]; then
  echo "DEMO_SOAK_RUNS must be between 1 and $MAX_RUNS, got $RUNS" >&2
  exit 2
fi

case "$MAX_RUNTIME_S" in
  ""|*[!0-9]*)
    echo "unsupported DEMO_SOAK_MAX_RUNTIME_S: $MAX_RUNTIME_S" >&2
    exit 2
    ;;
esac
if [ "$MAX_RUNTIME_S" -lt 60 ] || [ "$MAX_RUNTIME_S" -gt 7200 ]; then
  echo "DEMO_SOAK_MAX_RUNTIME_S must be between 60 and 7200, got $MAX_RUNTIME_S" >&2
  exit 2
fi

case "$PROFILE" in
  baseline)
    METADATA_MODE="once"
    ;;
  metadata-each)
    METADATA_MODE="each"
    ;;
  *)
    echo "unsupported DEMO_SOAK_PROFILE: $PROFILE" >&2
    echo "supported profiles: baseline, metadata-each" >&2
    exit 2
    ;;
esac

timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
ARTIFACT_DIR="${DEMO_SOAK_ARTIFACT_DIR:-/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_${timestamp}}"
if [ -d "$ARTIFACT_DIR" ] && find "$ARTIFACT_DIR" -mindepth 1 -maxdepth 1 | grep -q .; then
  echo "DEMO_SOAK_ARTIFACT_DIR must be empty or absent: $ARTIFACT_DIR" >&2
  exit 2
fi
mkdir -p "$ARTIFACT_DIR"

TRANSCRIPT="$ARTIFACT_DIR/demo_soak_repeated_run_transcript.log"
MARKERS="$ARTIFACT_DIR/demo_soak_repeated_run_markers.log"
MATRIX="$ARTIFACT_DIR/summary_matrix.tsv"
RUN_STATE="$ARTIFACT_DIR/run_state.tsv"
LEAK_SCAN="$ARTIFACT_DIR/leak_scan.txt"
PANIC_SCAN="$ARTIFACT_DIR/panic_scan.txt"
MANIFEST="$ARTIFACT_DIR/ARTIFACT_MANIFEST.txt"
: >"$TRANSCRIPT"
: >"$MARKERS"
: >"$MATRIX"
: >"$RUN_STATE"
: >"$LEAK_SCAN"
: >"$PANIC_SCAN"

exec > >(tee -a "$TRANSCRIPT") 2>&1

START_EPOCH="$(date +%s)"
printf 'run\trun_id\tdemo\tstress\tmetadata\tartifact_dir\ttmp_dir\n' >"$MATRIX"

die() {
  echo "demo-soak-repeated-run: ERROR: $*" >&2
  echo "demo-soak-repeated-run: artifacts: $ARTIFACT_DIR" >&2
  exit 1
}

mark() {
  printf '%s\n' "$1" | tee -a "$MARKERS"
}

remaining_seconds() {
  now="$(date +%s)"
  elapsed=$((now - START_EPOCH))
  remaining=$((MAX_RUNTIME_S - elapsed))
  if [ "$remaining" -lt 1 ]; then
    echo 0
  else
    echo "$remaining"
  fi
}

run_bounded_to_file() {
  label="$1"
  outfile="$2"
  shift 2

  remaining="$(remaining_seconds)"
  if [ "$remaining" -le 0 ]; then
    die "$label skipped because soak runtime budget expired"
  fi

  echo "RUN $label timeout=${remaining}s"
  set +e
  timeout "${remaining}s" "$@" >"$outfile" 2>&1
  status=$?
  set -e
  if [ "$status" -ne 0 ]; then
    echo "== $label output ==" >&2
    sed -n '1,220p' "$outfile" >&2 || true
    die "$label failed with status $status"
  fi
}

require_marker() {
  file="$1"
  marker="$2"
  if ! grep -F "$marker" "$file" >/dev/null; then
    die "required marker missing from $file: $marker"
  fi
}

scan_no_secret_leak() {
  : >"$LEAK_SCAN"
  found=0
  for pattern in \
    "NA0244_SECRET_SENTINEL" \
    "NA0246_SECRET_SENTINEL" \
    "NA0262_SECRET_SENTINEL" \
    "NA0266_SECRET_SENTINEL"
  do
    if grep -F -r -l "$pattern" "$ARTIFACT_DIR" >/tmp/na0266_soak_leak_hits.$$ 2>/dev/null; then
      sed "s#^#${pattern}\t#" /tmp/na0266_soak_leak_hits.$$ >>"$LEAK_SCAN"
      found=1
    fi
  done
  rm -f /tmp/na0266_soak_leak_hits.$$

  if [ -n "${QSHIELD_RELAY_TOKEN:-}" ]; then
    if grep -F -r -l "$QSHIELD_RELAY_TOKEN" "$ARTIFACT_DIR" >>"$LEAK_SCAN" 2>/dev/null; then
      found=1
    fi
  fi

  if [ "$found" -ne 0 ]; then
    sed -n '1,80p' "$LEAK_SCAN" >&2 || true
    die "token/secret/plaintext sentinel leak scan failed"
  fi
  echo "no token/secret/plaintext sentinel leakage detected" >"$LEAK_SCAN"
}

scan_no_panic() {
  : >"$PANIC_SCAN"
  if grep -E -r -l 'panicked at|stack backtrace|RUST_BACKTRACE|called .*unwrap' "$ARTIFACT_DIR" >"$PANIC_SCAN" 2>/dev/null; then
    sed -n '1,80p' "$PANIC_SCAN" >&2 || true
    die "panic/backtrace/unwrap marker scan failed"
  fi
  echo "no panic/backtrace/unwrap markers detected" >"$PANIC_SCAN"
}

write_manifest() {
  {
    echo "artifact_dir=$ARTIFACT_DIR"
    echo "runs=$RUNS"
    echo "profile=$PROFILE"
    echo "metadata_mode=$METADATA_MODE"
    echo "max_runtime_s=$MAX_RUNTIME_S"
    echo
    find "$ARTIFACT_DIR" -type f | sort | while IFS= read -r file; do
      rel="${file#"$ARTIFACT_DIR"/}"
      bytes="$(wc -c <"$file" | tr -d ' ')"
      printf '%s\t%s bytes\n' "$rel" "$bytes"
    done
  } >"$MANIFEST"
}

mark "NA0266_SOAK_START"
echo "artifact_dir=$ARTIFACT_DIR"
echo "runs=$RUNS"
echo "profile=$PROFILE"
echo "metadata_mode=$METADATA_MODE"
echo "bounded_runtime_seconds=$MAX_RUNTIME_S"
echo "loopback_only=true"
echo "non_production_demo_only=true"

for run in $(seq 1 "$RUNS"); do
  run_label="$(printf '%02d' "$run")"
  run_id="NA0266-${timestamp}-${run_label}"
  run_dir="$ARTIFACT_DIR/run_${run_label}"
  run_tmp="$run_dir/tmp"
  mkdir -p "$run_tmp"
  printf '%s\t%s\t%s\n' "$run" "$run_id" "$run_tmp" >>"$RUN_STATE"

  demo_log="$run_dir/demo_cli_smoke.log"
  stress_log="$run_dir/demo_adversarial_stress.log"
  stress_dir="$run_dir/demo_adversarial_stress"
  metadata_log="$run_dir/metadata_conformance_smoke.log"

  echo "NA0266_SOAK_RUN_START run=$run run_id=$run_id"
  export TMPDIR="$run_tmp"
  run_bounded_to_file \
    "run $run demo_cli_smoke" \
    "$demo_log" \
    bash scripts/ci/demo_cli_smoke.sh
  require_marker "$demo_log" "DEMO_ACCEPTANCE_OK"
  require_marker "$demo_log" "DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK"
  require_marker "$demo_log" "DEMO_NO_SECRET_LEAK_OK"
  mark "NA0266_SOAK_RUN_${run}_DEMO_OK"

  export DEMO_STRESS_PROFILE=baseline
  export DEMO_STRESS_ARTIFACT_DIR="$stress_dir"
  run_bounded_to_file \
    "run $run demo_adversarial_stress baseline" \
    "$stress_log" \
    bash scripts/ci/demo_adversarial_stress.sh
  require_marker "$stress_log" "DEMO_STRESS_NO_SECRET_LEAK_OK"
  require_marker "$stress_log" "DEMO_STRESS_NO_PANIC_OK"
  require_marker "$stress_log" "NA0262_DEMO_ADVERSARIAL_STRESS_OK"
  mark "NA0266_SOAK_RUN_${run}_STRESS_OK"

  metadata_state="not-run"
  if [ "$METADATA_MODE" = "each" ]; then
    export TMPDIR="$run_tmp"
    run_bounded_to_file \
      "run $run metadata_conformance_smoke" \
      "$metadata_log" \
      bash scripts/ci/metadata_conformance_smoke.sh
    require_marker "$metadata_log" "metadata-conformance-smoke: OK"
    metadata_state="ok"
  fi

  printf '%s\t%s\tok\tok\t%s\t%s\t%s\n' "$run" "$run_id" "$metadata_state" "$run_dir" "$run_tmp" >>"$MATRIX"
  scan_no_secret_leak
  scan_no_panic
done

if [ "$METADATA_MODE" = "once" ]; then
  metadata_dir="$ARTIFACT_DIR/metadata_once"
  metadata_tmp="$metadata_dir/tmp"
  mkdir -p "$metadata_tmp"
  export TMPDIR="$metadata_tmp"
  run_bounded_to_file \
    "metadata_conformance_smoke once" \
    "$metadata_dir/metadata_conformance_smoke.log" \
    bash scripts/ci/metadata_conformance_smoke.sh
  require_marker "$metadata_dir/metadata_conformance_smoke.log" "metadata-conformance-smoke: OK"
  printf 'metadata-once\tNA0266-%s-metadata\tn/a\tn/a\tok\t%s\t%s\n' "$timestamp" "$metadata_dir" "$metadata_tmp" >>"$MATRIX"
fi

unique_tmp_count="$(cut -f3 "$RUN_STATE" | sort -u | wc -l | tr -d ' ')"
if [ "$unique_tmp_count" != "$RUNS" ]; then
  die "state isolation proof failed: expected $RUNS unique run temp dirs, got $unique_tmp_count"
fi
for run in $(seq 1 "$RUNS"); do
  run_label="$(printf '%02d' "$run")"
  [ -d "$ARTIFACT_DIR/run_${run_label}" ] || die "missing per-run artifact dir for run $run"
done
mark "NA0266_SOAK_NO_STATE_BLEED_OK"

scan_no_secret_leak
mark "NA0266_SOAK_NO_SECRET_LEAK_OK"
scan_no_panic
mark "NA0266_SOAK_NO_PANIC_OK"

write_manifest
if [ ! -s "$MANIFEST" ]; then
  die "artifact manifest was not written"
fi
mark "NA0266_SOAK_ARTIFACT_MANIFEST_OK"

elapsed=$(( $(date +%s) - START_EPOCH ))
echo "total_runtime_seconds=$elapsed"
echo "artifact_dir=$ARTIFACT_DIR"
mark "NA0266_DEMO_SOAK_REPEATED_RUN_OK"
echo "demo-soak-repeated-run: OK"
