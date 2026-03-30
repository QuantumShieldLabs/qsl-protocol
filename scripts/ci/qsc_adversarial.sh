#!/bin/sh
set -eu

FUZZ_DIR="qsl/qsl-client/qsc/fuzz"
TMP_DIRS=""

cleanup() {
  if [ -n "${TMP_DIRS}" ]; then
    rm -rf ${TMP_DIRS}
  fi
}

run_fuzz_target() {
  target_name="$1"
  seed_dir="${FUZZ_DIR}/corpus/${target_name}"
  run_dir="$(mktemp -d)"
  TMP_DIRS="${TMP_DIRS} ${run_dir}"
  cp -R "${seed_dir}/." "${run_dir}/"
  (
    cd "${FUZZ_DIR}"
    cargo +nightly fuzz run "${target_name}" "${run_dir}" -- -max_total_time=10
  )
}

trap cleanup EXIT INT TERM

cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_properties
cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_miri

run_fuzz_target qsc_route_http
run_fuzz_target qsc_payload_boundaries
run_fuzz_target qsc_vault_envelope
