#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

FIXTURE_FILE="${1:-inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json}"
TIMEOUT_SECONDS="${NA0315_HARNESS_PLAN_TIMEOUT_SECONDS:-120}"

if [ ! -f "$FIXTURE_FILE" ]; then
  echo "missing fixture file: $FIXTURE_FILE" >&2
  exit 1
fi

timeout "$TIMEOUT_SECONDS" python3 - "$FIXTURE_FILE" <<'PY'
import json
import pathlib
import sys

fixture_path = pathlib.Path(sys.argv[1])
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))

REQUIRED_MARKERS = {
    "NA0315_IDENTIFIER_ROTATION_POLICY_OK",
    "NA0315_OPAQUE_HANDLE_BOUNDARY_OK",
    "NA0315_STALE_HANDLE_REJECT_OK",
    "NA0315_IDENTIFIER_NO_MUTATION_ON_REJECT_OK",
    "NA0315_IDENTIFIER_NO_SECRET_LOG_OK",
    "NA0315_DEFAULT_PADDING_POLICY_OK",
    "NA0315_PADDING_BUCKETS_OK",
    "NA0315_PADDING_INVALID_CONFIG_REJECT_OK",
    "NA0315_PADDING_STRIP_VERIFY_OK",
    "NA0315_PADDING_NO_MUTATION_ON_REJECT_OK",
    "NA0315_PADDING_NO_SECRET_LOG_OK",
    "NA0315_METADATA_RUNTIME_HARNESS_PLAN_OK",
}

FORBIDDEN_EMITTED_NOW = REQUIRED_MARKERS - {"NA0315_METADATA_RUNTIME_HARNESS_PLAN_OK"}
PANIC_PATTERNS = (
    "thread '",
    "panicked at",
    "stack backtrace",
    "RUST_BACKTRACE",
    "Traceback (most recent call last)",
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def as_list(name: str) -> list[object]:
    value = fixture.get(name)
    require(isinstance(value, list), f"{name} must be a list")
    return value


require(fixture.get("schema") == "qsl.metadata_runtime.identifier_padding_harness_plan.v1", "schema mismatch")
require(fixture.get("runtime_implementation_status") == "not_implemented", "runtime status must remain not_implemented")
require(fixture.get("selected_scope") == "non_runtime_executable_harness_plan", "scope mismatch")
require(
    fixture.get("selected_successor") == "NA-0316 -- Metadata Runtime qshield Poll No-Mutation Blocker Resolution",
    "successor mismatch",
)
require(fixture.get("max_fixture_count", 0) <= 32, "fixture count bound too high")

allowed = set(as_list("allowed_files"))
for expected in {
    "scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh",
    "inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json",
    "tests/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan_testplan.md",
    "docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
}:
    require(expected in allowed, f"allowed file missing: {expected}")

for prefix in ("apps/qshield-cli/src/", "apps/qsl-tui/src/", "qsl/qsl-client/qsc/src/", ".github/"):
    require(prefix in set(as_list("forbidden_files_or_prefixes")), f"forbidden prefix missing: {prefix}")

surfaces = as_list("candidate_runtime_surfaces")
require(4 <= len(surfaces) <= fixture["max_fixture_count"], "candidate surface count out of bounds")
surface_names = {surface.get("surface") for surface in surfaces if isinstance(surface, dict)}
for expected in {
    "qshield peer and bundle identifiers",
    "qshield relay queue and poll handles",
    "qshield default padding",
    "qsl-tui padded demo metadata",
    "qsc route and contact handles",
}:
    require(expected in surface_names, f"candidate surface missing: {expected}")

risk = fixture.get("qshield_poll_no_mutation_risk")
require(isinstance(risk, dict), "qshield risk missing")
require(risk.get("classification") == "STOP_RISK_FOR_RUNTIME_NO_MUTATION_PROOF", "qshield risk classification mismatch")
require(
    risk.get("required_successor") == fixture.get("selected_successor"),
    "qshield risk successor must match selected successor",
)
evidence = risk.get("evidence")
require(isinstance(evidence, list) and len(evidence) >= 3, "qshield risk evidence incomplete")
require(any("pop_front" in item for item in evidence), "qshield poll removal evidence missing")

identifier_plan = fixture.get("identifier_plan")
padding_plan = fixture.get("padding_plan")
require(isinstance(identifier_plan, dict), "identifier plan missing")
require(isinstance(padding_plan, dict), "padding plan missing")
for key in ("stale_handle", "malformed_handle", "wrong_scope_handle", "replayed_handle"):
    require(key in identifier_plan.get("required_rejects", []), f"identifier reject missing: {key}")
for key in ("invalid_bucket_config", "malformed_padded_input", "bucket_mismatch", "pad_len_too_large", "over_limit_payload"):
    require(key in padding_plan.get("required_rejects", []), f"padding reject missing: {key}")
require(padding_plan.get("required_strip_verify") is True, "strip/verify requirement missing")
require(padding_plan.get("bucket_table") == [512, 1024, 2048, 4096, 8192], "bucket table mismatch")

markers = fixture.get("required_future_markers")
require(isinstance(markers, list), "required marker list missing")
marker_status = {item.get("name"): item.get("status") for item in markers if isinstance(item, dict)}
require(set(marker_status) == REQUIRED_MARKERS, "required marker set mismatch")
for marker in FORBIDDEN_EMITTED_NOW:
    require(marker_status[marker].startswith("future_required"), f"runtime marker not future-gated: {marker}")
require(
    marker_status["NA0315_METADATA_RUNTIME_HARNESS_PLAN_OK"] == "emitted_by_this_plan_harness",
    "plan marker status mismatch",
)

claims = fixture.get("claims")
require(isinstance(claims, dict), "claim boundaries missing")
for key, value in claims.items():
    require(value is True, f"claim boundary not asserted: {key}")

rendered_fixture = json.dumps(fixture, sort_keys=True)
sentinel = fixture.get("internal_sentinel", "")
lines = [
    "NA0315_SCOPE_DECISION_PLAN_HARNESS_OK",
    "NA0315_RUNTIME_SURFACES_INVENTORIED_OK",
    "NA0315_QSHIELD_POLL_NO_MUTATION_BLOCKER_RECORDED_OK",
    "NA0315_REQUIRED_FUTURE_MARKERS_RECORDED_OK",
    "NA0315_METADATA_RUNTIME_HARNESS_PLAN_OK",
]
joined = "\n".join(lines)
require(sentinel not in joined, "internal sentinel leaked into output")
require(not any(pattern in joined for pattern in PANIC_PATTERNS), "panic/backtrace text leaked into output")
require(sentinel in rendered_fixture, "fixture sentinel missing")

print(joined)
PY
