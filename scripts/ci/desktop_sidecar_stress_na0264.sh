#!/bin/sh
set -eu

repo_root="$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)"
artifact_dir="${NA0264_ARTIFACT_DIR:-/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_manual}"
log="${artifact_dir}/desktop_sidecar_stress_na0264.log"

mkdir -p "${artifact_dir}"
: > "${log}"

run_logged() {
  printf '$' >> "${log}"
  printf ' %s' "$@" >> "${log}"
  printf '\n' >> "${log}"
  "$@" >> "${log}" 2>&1
}

marker() {
  printf '%s\n' "$1" | tee -a "${log}"
}

cd "${repo_root}"

run_logged cargo test --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked sidecar_path_validation_rejects_missing_override
marker NA0264_SIDECAR_MISSING_REJECT_OK
marker NA0264_SIDECAR_BAD_PATH_REJECT_OK

run_logged cargo test --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked sidecar_path_validation_rejects_non_executable_file
marker NA0264_SIDECAR_NONEXEC_REJECT_OK

run_logged cargo test --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked malformed_doctor_output_rejects_without_panic
marker NA0264_SIDECAR_MALFORMED_OUTPUT_REJECT_OK

run_logged cargo test --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked non_marker_sidecar_failure_suppresses_stderr_secrets
marker NA0264_SIDECAR_NONZERO_REJECT_OK
marker NA0264_NO_SECRET_LEAK_OK

run_logged cargo test --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked sidecar_timeout_kills_hung_child
marker NA0264_SIDECAR_TIMEOUT_REJECT_OK

run_logged cargo test -p qsc --locked --test cli doctor_check_only_no_dir
marker NA0264_MISSING_STORE_REJECT_OK

run_logged cargo test -p qsc --locked --test cli symlink_path_rejected_no_mutation
marker NA0264_INVALID_STORE_REJECT_OK

run_logged cargo test -p qsc --locked --test qsp_protocol_gate send_refuses_when_protocol_inactive
run_logged cargo test -p qsc --locked --test qsp_protocol_gate receive_refuses_when_protocol_inactive
marker NA0264_PROTOCOL_INACTIVE_FAIL_CLOSED_OK

run_logged cargo test -p qsc --locked --test send_commit send_failure_no_commit
marker NA0264_RELAY_UNAVAILABLE_REJECT_OK

marker NA0264_NO_PANIC_OK
marker NA0264_DESKTOP_SIDECAR_STRESS_OK
