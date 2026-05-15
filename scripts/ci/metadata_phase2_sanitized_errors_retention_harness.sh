#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

VECTOR_FILE="${1:-inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json}"
POLICY_TIMEOUT_SECONDS="${NA0293_POLICY_TIMEOUT_SECONDS:-120}"
CROSS_SURFACE_TIMEOUT_SECONDS="${NA0293_CROSS_SURFACE_TIMEOUT_SECONDS:-900}"
RUN_METADATA_SMOKE="${NA0293_RUN_METADATA_SMOKE:-1}"
RUN_NA0291_HARNESS="${NA0293_RUN_NA0291_HARNESS:-1}"
RUN_DEMO_SMOKE="${NA0293_RUN_DEMO_SMOKE:-1}"
RUN_DEMO_STRESS="${NA0293_RUN_DEMO_STRESS:-0}"

if [ ! -f "$VECTOR_FILE" ]; then
  echo "missing vector file: $VECTOR_FILE" >&2
  exit 1
fi

ARTIFACT_DIR="${NA0293_HARNESS_ARTIFACT_DIR:-$(mktemp -d /srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.XXXXXX)}"
mkdir -p "$ARTIFACT_DIR"

run_child() {
  label="$1"
  shift
  log="$ARTIFACT_DIR/${label}.log"
  echo "NA0293_CHILD_START ${label}"
  set +e
  timeout "$CROSS_SURFACE_TIMEOUT_SECONDS" "$@" >"$log" 2>&1
  status=$?
  set -e
  cat "$log"
  if [ "$status" -ne 0 ]; then
    echo "NA0293_CHILD_FAIL ${label} status=${status}" >&2
    exit "$status"
  fi
  echo "NA0293_CHILD_OK ${label}"
}

POLICY_LOG="$ARTIFACT_DIR/policy_fixture.log"
set +e
timeout "$POLICY_TIMEOUT_SECONDS" python3 - "$VECTOR_FILE" >"$POLICY_LOG" 2>&1 <<'PY'
import copy
import json
import re
import sys
from dataclasses import dataclass
from typing import Any

vector_path = sys.argv[1]
with open(vector_path, "r", encoding="utf-8") as handle:
    vectors = json.load(handle)

MAX_FIXTURES = int(vectors.get("max_fixture_count", 0))
if MAX_FIXTURES <= 0 or MAX_FIXTURES > 64:
    raise SystemExit("invalid max_fixture_count")

SECRET_SENTINELS = list((vectors.get("secret_sentinels") or {}).values())
PANIC_PATTERNS = [
    "thread '",
    "panicked at",
    "stack backtrace",
    "RUST_BACKTRACE",
    "called `",
    "Traceback (most recent call last)",
]
FORBIDDEN_ERROR_KEYS = {
    "route_token",
    "capability",
    "identifier",
    "handle",
    "descriptor",
    "plaintext",
    "ciphertext",
    "internal_path",
    "deleted_count",
    "expired_count",
    "purged_count",
    "queue_depth",
    "object_exists",
    "exact_age_seconds",
}


def require(condition: bool, msg: str) -> None:
    if not condition:
        raise AssertionError(msg)


def stable_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def check_count(name: str, values: list[Any]) -> None:
    require(len(values) <= MAX_FIXTURES, f"{name} exceeds max fixture count")


def output_is_forbidden(value: Any) -> bool:
    text = stable_json(value)
    lowered = text.lower()
    if any(sentinel and sentinel in text for sentinel in SECRET_SENTINELS):
        return True
    if any(pattern.lower() in lowered for pattern in PANIC_PATTERNS):
        return True
    if isinstance(value, dict):
        if FORBIDDEN_ERROR_KEYS.intersection(value.keys()):
            return True
        for child in value.values():
            if output_is_forbidden(child):
                return True
    elif isinstance(value, list):
        return any(output_is_forbidden(child) for child in value)
    return False


@dataclass(frozen=True)
class ErrorEnvelope:
    status_class: str
    reason_code: str

    def body(self) -> dict[str, str | bool]:
        return {
            "ok": False,
            "status_class": self.status_class,
            "reason_code": self.reason_code,
        }


def sanitized_error(category: str) -> ErrorEnvelope:
    if category in {
        "malformed_metadata",
        "invalid_identifier",
        "invalid_padding_config",
        "oversized_metadata",
    }:
        return ErrorEnvelope("bad_request", "REJECT_METADATA_INVALID")
    if category == "unauthorized_capability":
        return ErrorEnvelope("unauthorized", "REJECT_METADATA_UNAUTHORIZED")
    if category in {
        "expired_state_access",
        "deleted_state_access",
        "purged_state_access",
        "tombstone_access",
        "missing_record_access",
        "purge_error",
    }:
        return ErrorEnvelope("unavailable", "REJECT_METADATA_UNAVAILABLE")
    raise ValueError(f"unknown category: {category}")


def validate_sanitized_errors() -> None:
    policy = vectors["sanitized_error_policy"]
    allowed_status = set(policy["allowed_status_classes"])
    allowed_reasons = set(policy["allowed_reason_codes"])
    fixtures = list(policy["fixtures"])
    check_count("sanitized-error fixtures", fixtures)

    accepted_state = {
        "accepted_handles": ["m2h_safe_fixture_a"],
        "accepted_records": ["active-record"],
    }
    before = stable_json(accepted_state)

    for case in fixtures:
        env = sanitized_error(str(case["category"]))
        body = env.body()
        require(env.status_class == case["expected_status_class"], f"{case['case_id']} status mismatch")
        require(env.reason_code == case["expected_reason_code"], f"{case['case_id']} reason mismatch")
        require(env.status_class in allowed_status, f"{case['case_id']} status not allowed")
        require(env.reason_code in allowed_reasons, f"{case['case_id']} reason not allowed")
        require(not output_is_forbidden(body), f"{case['case_id']} leaked forbidden output")
        require(stable_json(accepted_state) == before, f"{case['case_id']} mutated accepted state")

    for case in policy["forbidden_output_cases"]:
        rejected = output_is_forbidden(case["body"])
        require(rejected is bool(case["expect_scanner_reject"]), f"{case['case_id']} scanner expectation mismatch")


def retention_action(created_at: int, now: int, window_seconds: int) -> str:
    require(window_seconds > 0 and window_seconds <= 86_400, "retention window out of bounds")
    require(now >= created_at, "retention case has negative age")
    return "retain" if now - created_at <= window_seconds else "expire"


def retention_error_for_state(state: str) -> ErrorEnvelope:
    if state == "active":
        raise ValueError("active state is not a retention error")
    if state in {"deleted", "expired", "purged", "tombstoned"}:
        return sanitized_error("tombstone_access")
    raise ValueError(f"unknown record state: {state}")


def validate_retention_purge() -> None:
    policy = vectors["retention_purge_policy"]
    records = {case["record_id"]: dict(case) for case in policy["records"]}
    check_count("retention records", list(records.values()))
    check_count("retention access fixtures", list(policy["access_fixtures"]))
    check_count("retention window fixtures", list(policy["retention_window_cases"]))
    check_count("rejected state fixtures", list(policy["rejected_state_fixtures"]))

    before_records = stable_json(records)
    for case in policy["access_fixtures"]:
        record = records[str(case["record_id"])]
        before_one = copy.deepcopy(record)
        env = retention_error_for_state(str(record["state"]))
        body = env.body()
        require(env.status_class == case["expected_status_class"], f"{case['case_id']} status mismatch")
        require(env.reason_code == case["expected_reason_code"], f"{case['case_id']} reason mismatch")
        require(record["state"] == case["expected_state_after"], f"{case['case_id']} resurrected or mutated state")
        require(record == before_one, f"{case['case_id']} mutated record")
        require(not output_is_forbidden(body), f"{case['case_id']} leaked forbidden output")

    for case in policy["retention_window_cases"]:
        action1 = retention_action(int(case["created_at"]), int(case["now"]), int(case["window_seconds"]))
        action2 = retention_action(int(case["created_at"]), int(case["now"]), int(case["window_seconds"]))
        require(action1 == action2, f"{case['case_id']} retention action is not deterministic")
        require(action1 == case["expected_action"], f"{case['case_id']} retention action mismatch")

    for case in policy["rejected_state_fixtures"]:
        before = stable_json(records)
        category = "missing_record_access" if str(case["record_id"]) not in records else "purge_error"
        env = sanitized_error(category)
        body = env.body()
        require(env.status_class == case["expected_status_class"], f"{case['case_id']} status mismatch")
        require(env.reason_code == case["expected_reason_code"], f"{case['case_id']} reason mismatch")
        require(stable_json(records) == before, f"{case['case_id']} mutated records")
        require(not output_is_forbidden(body), f"{case['case_id']} leaked forbidden output")

    require(stable_json(records) == before_records, "retention/purge fixtures mutated global records")


try:
    require(vectors["policy_status"] == "harness_only", "policy status must remain harness_only")
    require(
        vectors["runtime_implementation_status"] == "not_implemented",
        "runtime implementation status must remain not_implemented",
    )
    validate_sanitized_errors()
    validate_retention_purge()
except Exception as exc:
    print(f"NA0293_METADATA_PHASE2_SANITIZED_RETENTION_FAIL reason={type(exc).__name__}", file=sys.stderr)
    raise

print("NA0293_POLICY_FIXTURE_MODEL_OK")
PY
policy_status=$?
set -e
cat "$POLICY_LOG"
if [ "$policy_status" -ne 0 ]; then
  echo "NA0293_POLICY_FIXTURE_FAIL status=${policy_status}" >&2
  exit "$policy_status"
fi

if [ "$RUN_NA0291_HARNESS" = "1" ] && [ -x scripts/ci/metadata_phase2_identifier_padding_harness.sh ]; then
  run_child "metadata_phase2_identifier_padding_harness" scripts/ci/metadata_phase2_identifier_padding_harness.sh
fi

if [ "$RUN_METADATA_SMOKE" = "1" ] && [ -x scripts/ci/metadata_conformance_smoke.sh ]; then
  run_child "metadata_conformance_smoke" scripts/ci/metadata_conformance_smoke.sh
fi

if [ "$RUN_DEMO_SMOKE" = "1" ] && [ -x scripts/ci/demo_cli_smoke.sh ]; then
  run_child "demo_cli_smoke" scripts/ci/demo_cli_smoke.sh
fi

if [ "$RUN_DEMO_STRESS" = "1" ] && [ -x scripts/ci/demo_adversarial_stress.sh ]; then
  DEMO_STRESS_ARTIFACT_DIR="$ARTIFACT_DIR/demo_adversarial_stress" \
    DEMO_STRESS_PROFILE="${DEMO_STRESS_PROFILE:-baseline}" \
    run_child "demo_adversarial_stress" scripts/ci/demo_adversarial_stress.sh
fi

python3 - "$VECTOR_FILE" "$ARTIFACT_DIR" <<'PY'
import json
import pathlib
import sys

vector_path = pathlib.Path(sys.argv[1])
artifact_dir = pathlib.Path(sys.argv[2])
vectors = json.loads(vector_path.read_text(encoding="utf-8"))
sentinels = [v for v in (vectors.get("secret_sentinels") or {}).values() if v]
panic_patterns = [
    "thread '",
    "panicked at",
    "stack backtrace",
    "RUST_BACKTRACE",
    "Traceback (most recent call last)",
]

bad = []
for path in sorted(p for p in artifact_dir.rglob("*") if p.is_file()):
    text = path.read_text(encoding="utf-8", errors="replace")
    for sentinel in sentinels:
        if sentinel in text:
            bad.append((path.as_posix(), "sentinel"))
    lowered = text.lower()
    for pattern in panic_patterns:
        if pattern.lower() in lowered:
            bad.append((path.as_posix(), "panic_or_backtrace"))

if bad:
    for path, kind in bad:
        print(f"NA0293_ARTIFACT_SCAN_FAIL {kind} {path}", file=sys.stderr)
    raise SystemExit(1)

print(f"NA0293_ARTIFACT_SCAN_OK files={len(list(artifact_dir.rglob('*')))}")
PY

echo "NA0293_HARNESS_ARTIFACT_DIR=$ARTIFACT_DIR"
echo "NA0293_SANITIZED_ERROR_POLICY_OK"
echo "NA0293_RETENTION_PURGE_POLICY_OK"
echo "NA0293_METADATA_PHASE2_SANITIZED_RETENTION_OK"
