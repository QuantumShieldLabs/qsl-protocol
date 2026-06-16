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
  if [ -d "${seed_dir}" ]; then
    cp -R "${seed_dir}/." "${run_dir}/"
  fi
  (
    cd "${FUZZ_DIR}"
    if [ "${target_name}" = "qsc_binding_semantics" ]; then
      target_rustflags="${RUSTFLAGS:-}"
      target_rustflags="${target_rustflags:+${target_rustflags} }--cfg qsc_binding_fuzz_helper"
      RUSTFLAGS="${target_rustflags}" cargo +nightly fuzz run "${target_name}" "${run_dir}" -- -max_total_time=10
    else
      cargo +nightly fuzz run "${target_name}" "${run_dir}" -- -max_total_time=10
    fi
  )
}

trap cleanup EXIT INT TERM

cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_properties
cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_miri

echo "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP"
cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test handshake_provider_error_no_mutation -- --test-threads=1

run_fuzz_target qsc_route_http
run_fuzz_target qsc_payload_boundaries
run_fuzz_target qsc_vault_envelope
echo "NA0487_FUZZ_CI_ADVERSARIAL_TARGET_INCLUDED_OK"
run_fuzz_target qsc_binding_semantics
