#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

VECTOR_FILE="${1:-inputs/metadata_phase2/identifier_padding_policy_vectors_v1.json}"
TIMEOUT_SECONDS="${NA0291_HARNESS_TIMEOUT_SECONDS:-120}"

if [ ! -f "$VECTOR_FILE" ]; then
  echo "missing vector file: $VECTOR_FILE" >&2
  exit 1
fi

timeout "$TIMEOUT_SECONDS" python3 - "$VECTOR_FILE" <<'PY'
import hashlib
import json
import re
import sys
from dataclasses import dataclass

vector_path = sys.argv[1]
with open(vector_path, "r", encoding="utf-8") as handle:
    vectors = json.load(handle)

MAX_FIXTURES = int(vectors.get("max_fixture_count", 0))
if MAX_FIXTURES <= 0 or MAX_FIXTURES > 64:
    raise SystemExit("invalid max_fixture_count")

HANDLE_RE = re.compile(r"^m2h_[0-9a-f]{32}$")
IDENT_RE = re.compile(r"^[a-z0-9][a-z0-9._-]{0,63}$")
PRINTED_LINES: list[str] = []


def fail(msg: str) -> None:
    raise AssertionError(msg)


def emit(line: str) -> None:
    PRINTED_LINES.append(line)


def require(condition: bool, msg: str) -> None:
    if not condition:
        fail(msg)


def check_count(name: str, values: list[object]) -> None:
    require(len(values) <= MAX_FIXTURES, f"{name} exceeds max fixture count")


def valid_identifier(value: object) -> bool:
    return isinstance(value, str) and IDENT_RE.fullmatch(value) is not None


def derive_handle(case: dict[str, object]) -> str:
    for key in ("sender", "contact", "route", "session", "epoch", "operation"):
        if not valid_identifier(case.get(key)):
            raise ValueError(f"invalid identifier field: {key}")
    material = "|".join(
        [
            "qsl-na0291-handle-v1",
            str(case["sender"]),
            str(case["contact"]),
            str(case["route"]),
            str(case["session"]),
            str(case["epoch"]),
            str(case["operation"]),
        ]
    )
    digest = hashlib.sha256(material.encode("utf-8")).hexdigest()[:32]
    return f"m2h_{digest}"


def validate_handle(value: object) -> bool:
    return isinstance(value, str) and HANDLE_RE.fullmatch(value) is not None


def exercise_identifier_policy() -> dict[str, str]:
    policy = vectors["handle_policy"]
    fixtures = list(policy["fixtures"])
    invalid_inputs = list(policy["invalid_inputs"])
    check_count("handle fixtures", fixtures)
    check_count("invalid handle fixtures", invalid_inputs)

    by_id: dict[str, str] = {}
    by_handle: dict[str, str] = {}
    for case in fixtures:
        case_id = str(case["case_id"])
        first = derive_handle(case)
        second = derive_handle(case)
        require(first == second, f"{case_id} handle derivation is not deterministic")
        require(first == case["expected_handle"], f"{case_id} expected handle mismatch")
        require(validate_handle(first), f"{case_id} handle format invalid")
        if "expect_same_as" in case:
            ref = by_id[str(case["expect_same_as"])]
            require(first == ref, f"{case_id} expected same handle as {case['expect_same_as']}")
        if "expect_diff_from" in case:
            ref = by_id[str(case["expect_diff_from"])]
            require(first != ref, f"{case_id} did not rotate from {case['expect_diff_from']}")
        if first in by_handle and "expect_same_as" not in case:
            fail(f"unexpected handle collision between {case_id} and {by_handle[first]}")
        by_id[case_id] = first
        by_handle.setdefault(first, case_id)

    required_boundaries = {
        "session-boundary",
        "epoch-boundary",
        "route-boundary",
        "contact-boundary",
        "attachment-boundary",
    }
    require(required_boundaries.issubset(by_id), "rotation boundary fixtures missing")

    state = {"accepted": dict(by_id), "reject_count": 0}
    before = json.dumps(state, sort_keys=True)
    rejected = 0
    for case in invalid_inputs:
        try:
            if "handle" in case:
                ok = validate_handle(case["handle"])
            else:
                derive_handle(case)
                ok = True
        except (KeyError, TypeError, ValueError):
            ok = False
        require(not ok, f"invalid identifier case accepted: {case.get('case_id')}")
        rejected += 1
    after = json.dumps(state, sort_keys=True)
    require(before == after, "invalid identifier inputs mutated accepted state")
    require(rejected == len(invalid_inputs), "not all invalid identifier inputs rejected")
    return by_id


def validate_bucket_config(buckets: object, max_payload_len: int) -> bool:
    if not isinstance(buckets, list) or not buckets:
        return False
    prev = 0
    for bucket in buckets:
        if not isinstance(bucket, int):
            return False
        if bucket <= 0 or bucket <= prev or bucket > max_payload_len:
            return False
        prev = bucket
    return True


def choose_bucket(length: int, buckets: list[int]) -> int:
    if length <= 0:
        raise ValueError("invalid payload length")
    for bucket in buckets:
        if length <= bucket:
            return bucket
    raise ValueError("payload exceeds largest bucket")


@dataclass(frozen=True)
class PaddingMetadata:
    wire_len: int
    pad_len: int
    bucket: int | None


def pad_payload(payload: bytes, buckets: list[int]) -> tuple[bytes, PaddingMetadata]:
    bucket = choose_bucket(len(payload), buckets)
    pad_len = bucket - len(payload)
    return payload + (b"\x00" * pad_len), PaddingMetadata(len(payload) + pad_len, pad_len, bucket)


def verify_and_strip(wire_hex: object, pad_len: object, bucket: object) -> bytes:
    if not isinstance(wire_hex, str) or len(wire_hex) % 2 != 0:
        raise ValueError("bad wire hex")
    wire = bytes.fromhex(wire_hex)
    if pad_len is None:
        pad_len = 0
    if not isinstance(pad_len, int) or pad_len < 0:
        raise ValueError("invalid pad_len")
    if bucket is None:
        if pad_len != 0:
            raise ValueError("pad without bucket")
    else:
        if not isinstance(bucket, int) or len(wire) != bucket:
            raise ValueError("bucket mismatch")
    if pad_len > len(wire):
        raise ValueError("pad too large")
    return wire[: len(wire) - pad_len] if pad_len else wire


def exercise_padding_policy() -> None:
    policy = vectors["padding_policy"]
    buckets = list(policy["buckets"])
    max_payload_len = int(policy["max_payload_len"])
    require(validate_bucket_config(buckets, max_payload_len), "default bucket config invalid")
    require(buckets == [512, 1024, 2048, 4096], "unexpected bucket table")

    expectations = list(policy["length_expectations"])
    check_count("padding expectations", expectations)
    for case in expectations:
        payload_len = int(case["payload_len"])
        bucket = choose_bucket(payload_len, buckets)
        require(bucket == int(case["bucket"]), f"bucket mismatch for length {payload_len}")
        require(bucket - payload_len == int(case["pad_len"]), f"pad_len mismatch for length {payload_len}")

    observed_overhead = max(choose_bucket(length, buckets) - length for length in range(1, max_payload_len + 1))
    require(observed_overhead == int(policy["max_overhead"]), "max overhead fixture mismatch")
    require(observed_overhead <= 2047, "max overhead bound exceeded")

    for case in policy["invalid_configs"]:
        require(
            not validate_bucket_config(case["buckets"], max_payload_len),
            f"invalid padding config accepted: {case['case_id']}",
        )

    for case in policy["strip_verify_cases"]:
        payload = bytes.fromhex(case["payload_hex"])
        padded, meta = pad_payload(payload, buckets)
        require(meta.bucket == int(case["bucket"]), f"strip case bucket mismatch: {case['case_id']}")
        stripped = verify_and_strip(padded.hex(), meta.pad_len, meta.bucket)
        require(stripped == payload, f"strip case failed: {case['case_id']}")
        public_meta = json.dumps({"wire_len": meta.wire_len, "pad_len": meta.pad_len, "bucket": meta.bucket})
        require(vectors["secret_sentinel"] not in public_meta, "sentinel leaked through padding metadata")
        require(payload.decode("utf-8") not in public_meta, "payload text leaked through padding metadata")

    for case in policy["invalid_metadata"]:
        try:
            verify_and_strip(case["wire_hex"], case.get("pad_len"), case.get("bucket"))
            ok = True
        except (TypeError, ValueError):
            ok = False
        require(not ok, f"invalid padding metadata accepted: {case['case_id']}")

    for length in (0, max_payload_len + 1):
        try:
            choose_bucket(length, buckets)
            ok = True
        except ValueError:
            ok = False
        require(not ok, f"invalid payload length accepted: {length}")


try:
    handles = exercise_identifier_policy()
    exercise_padding_policy()
    require("baseline-session-a" in handles, "baseline handle missing")
    require(vectors["rotation_status"] == "design_only", "rotation status must remain design_only")
    require(vectors["padding_default_status"] == "design_only", "padding status must remain design_only")
except Exception as exc:
    print(f"NA0291_METADATA_PHASE2_HARNESS_FAIL reason={type(exc).__name__}", file=sys.stderr)
    raise

emit("DESIGN_ONLY_ROTATION_POLICY_PROOF")
emit("DESIGN_ONLY_PADDING_POLICY_PROOF")
emit("NA0291_IDENTIFIER_POLICY_OK")
emit("NA0291_ROTATION_POLICY_OK")
emit("NA0291_PADDING_POLICY_OK")
emit("NA0291_METADATA_PHASE2_HARNESS_OK")

joined = "\n".join(PRINTED_LINES)
if vectors["secret_sentinel"] in joined:
    raise SystemExit("sentinel leaked into harness output")
print(joined)
PY
