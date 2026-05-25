#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT_DIR"

FIXTURE_FILE=${1:-inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json}
TMP_ROOT=${NA0359_HARNESS_TMP_ROOT:-/srv/qbuild/tmp}

case "$TMP_ROOT" in
  /srv/qbuild/tmp|/srv/qbuild/tmp/*) ;;
  *)
    echo "NA0359_HARNESS_TMP_ROOT must be under /srv/qbuild/tmp" >&2
    exit 1
    ;;
esac

if [ ! -f "$FIXTURE_FILE" ]; then
  echo "missing fixture file: $FIXTURE_FILE" >&2
  exit 1
fi

ARTIFACT_DIR=$(mktemp -d "$TMP_ROOT/NA-0359_restore_drill_dry_run.XXXXXX")

python3 - "$FIXTURE_FILE" "$ARTIFACT_DIR" <<'PY'
import copy
import hashlib
import json
import pathlib
import re
import sys

fixture_path = pathlib.Path(sys.argv[1])
artifact_dir = pathlib.Path(sys.argv[2]).resolve()
tmp_root = pathlib.Path("/srv/qbuild/tmp").resolve()

if tmp_root not in (artifact_dir, *artifact_dir.parents):
    raise SystemExit("artifact directory is outside /srv/qbuild/tmp")

fixture = json.loads(fixture_path.read_text(encoding="utf-8"))

REQUIRED_FIELDS = {
    "schema_version",
    "artifact_class",
    "goals",
    "authorization",
    "source_classification",
    "backup_state_classification",
    "restore_state_classification",
    "target_type",
    "restore_mode",
    "no_secret_sentinels",
    "manifest_entries",
    "checksums",
    "expected_validation_outcomes",
    "tamper_negative_cases",
    "cleanup_plan",
    "monitoring_alert_plan",
    "operator_runbook_summary",
    "forbidden_operations",
    "claim_boundaries",
    "required_markers",
    "backup_plan_impact",
    "qsl_server_boundary",
    "qsl_attachments_boundary",
    "qshield_demo_boundary",
}

REQUIRED_MARKERS = {
    "NA0359_RESTORE_DRILL_AUTHORIZATION_OK",
    "NA0359_DRY_RUN_RESTORE_HARNESS_OK",
    "NA0359_NO_SECRET_FIXTURE_OK",
    "NA0359_MANIFEST_CHECKSUM_VALIDATION_OK",
    "NA0359_ARTIFACT_REDACTION_OK",
    "NA0359_FAILED_VALIDATION_FAILS_CLOSED_OK",
    "NA0359_CLEANUP_MARKER_OK",
    "NA0359_MONITORING_ALERT_PLAN_MARKER_OK",
    "NA0359_OPERATOR_RUNBOOK_MARKER_OK",
    "NA0359_BACKUP_PLAN_IMPACT_OK",
    "NA0359_NO_RESTORE_EXECUTION_OK",
    "NA0359_NO_RESTORE_TARGET_CREATION_OK",
    "NA0359_NO_KEY_GENERATION_OK",
    "NA0359_NO_PASSPHRASE_COLLECTION_OK",
    "NA0359_NO_SECRET_MATERIAL_OK",
    "NA0359_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK",
    "NA0359_NO_PRODUCTION_READY_CLAIM_OK",
    "NA0359_NO_PUBLIC_INTERNET_READY_CLAIM_OK",
    "NA0359_METADATA_RUNTIME_RESTORE_DRY_RUN_OK",
}

REQUIRED_FORBIDDEN = {
    "backup_execution",
    "restore_execution",
    "restore_target_creation",
    "restore_target_mount",
    "restore_copy",
    "key_generation",
    "key_upload",
    "passphrase_collection",
    "private_key_inspection",
    "secret_handling",
    "off_host_setup",
    "deploy",
    "rollback",
    "local_backup_mutation",
    "backup_script_timer_fstab_mutation",
    "qsl_server_mutation",
    "qsl_attachments_mutation",
    "qshield_runtime_mutation",
    "protocol_crypto_qsc_qsp_mutation",
    "dependency_change",
    "workflow_change",
    "website_public_docs_change",
}

CLAIM_KEYS = {
    "production_readiness",
    "public_internet_readiness",
    "external_review_complete",
    "metadata_free",
    "anonymity",
    "untraceable",
    "attachment_size_hidden",
    "timing_metadata_hidden",
    "traffic_shape_hidden",
    "padding_hides_all_metadata",
    "local_continuity_is_disaster_recovery",
    "off_host_backup_complete",
    "real_restore_drill_executed",
    "key_custody_implemented",
    "key_recovery_implemented",
}

NEGATIVE_CASES = {
    "missing_manifest",
    "checksum_mismatch",
    "missing_cleanup_plan",
    "prohibited_operation_field",
    "missing_claim_boundary",
    "missing_no_secret_fixture_marker",
    "sentinel_leak_detection",
}

SECRET_PATTERNS = [
    re.compile(r"-----BEGIN (?:RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----"),
    re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{30,}\b"),
    re.compile(r"\bxox[baprs]-[A-Za-z0-9-]{20,}\b"),
    re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b"),
    re.compile(r"\bAIza[0-9A-Za-z_-]{35}\b"),
    re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b"),
    re.compile(r"\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}\b"),
]


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def require_mapping(data, name):
    value = data.get(name)
    require(isinstance(value, dict), f"{name} must be an object")
    return value


def strings_except_sentinel_list(value, path=()):
    if path == ("no_secret_sentinels",):
        return
    if isinstance(value, str):
        yield value
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from strings_except_sentinel_list(item, path + (str(index),))
    elif isinstance(value, dict):
        for key, item in value.items():
            yield from strings_except_sentinel_list(item, path + (str(key),))


def count_secret_patterns(text):
    return sum(1 for pattern in SECRET_PATTERNS if pattern.search(text))


def count_sentinel_leaks(text, sentinels):
    return sum(text.count(sentinel) for sentinel in sentinels)


def validate_fixture(data):
    missing = sorted(REQUIRED_FIELDS - set(data))
    require(not missing, f"missing required fields: {', '.join(missing)}")
    require(
        data.get("schema_version") == "qsl.metadata_runtime.restore_drill_dry_run_fixture.v1",
        "schema_version mismatch",
    )
    require(data.get("artifact_class") == "restore_drill_dry_run_no_secret_v1", "artifact_class mismatch")
    require(set(data.get("goals", [])) == {"G1", "G2", "G3", "G4", "G5"}, "goals mismatch")
    require(data.get("target_type") == "fixture-only", "target_type mismatch")
    require(data.get("restore_mode") == "dry-run only", "restore_mode mismatch")

    authorization = require_mapping(data, "authorization")
    require(authorization.get("real_restore_authorized") is False, "real restore is authorized")
    require(authorization.get("backup_authorized") is False, "backup is authorized")
    require(authorization.get("off_host_authorized") is False, "off-host operation is authorized")
    require(authorization.get("key_handling_authorized") is False, "key handling is authorized")

    sources = require_mapping(data, "source_classification")
    require(sources.get("qsl_server") == "FRESH_SOURCE", "qsl-server source is not fresh")
    require(sources.get("qsl_attachments") == "FRESH_SOURCE", "qsl-attachments source is not fresh")
    require(sources.get("qshield_demo") == "REFERENCE_ORACLE_ONLY", "qshield demo boundary mismatch")

    backup_state = require_mapping(data, "backup_state_classification")
    require(backup_state.get("local_continuity") == "LOCAL_CONTINUITY_PROVEN", "local continuity classification mismatch")
    require(backup_state.get("off_host_backup") == "OFF_HOST_BACKUP_NOT_READY", "off-host backup classification mismatch")
    require(backup_state.get("key_custody") == "KEY_CUSTODY_PARTIAL", "key custody classification mismatch")
    require(backup_state.get("key_recovery") == "KEY_RECOVERY_PARTIAL", "key recovery classification mismatch")

    restore_state = require_mapping(data, "restore_state_classification")
    require(restore_state.get("real_restore") == "REAL_RESTORE_NOT_AUTHORIZED", "real restore boundary mismatch")
    require(restore_state.get("restore_target") == "RESTORE_TARGET_NOT_CREATED", "restore target boundary mismatch")
    require(restore_state.get("operation_executed_by_harness") is False, "operation executed flag is true")

    sentinels = data.get("no_secret_sentinels")
    require(isinstance(sentinels, list) and sentinels, "no_secret_sentinels must be a non-empty list")
    require(len(set(sentinels)) == len(sentinels), "duplicate no-secret sentinels")
    for sentinel in sentinels:
        require(isinstance(sentinel, str) and sentinel.startswith("NA0359_SECRET_SENTINEL_"), "invalid sentinel label")

    manifest_entries = data.get("manifest_entries")
    require(isinstance(manifest_entries, list) and manifest_entries, "manifest_entries must be a non-empty list")
    checksums = require_mapping(data, "checksums")
    require(checksums.get("algorithm") == "sha256", "checksum algorithm mismatch")
    checksum_entries = checksums.get("entries")
    require(isinstance(checksum_entries, list) and checksum_entries, "checksum entries missing")
    checksum_by_id = {}
    for item in checksum_entries:
        require(isinstance(item, dict), "checksum entry must be an object")
        checksum_by_id[item.get("id")] = item.get("sha256")

    seen_ids = set()
    for entry in manifest_entries:
        require(isinstance(entry, dict), "manifest entry must be an object")
        entry_id = entry.get("id")
        require(isinstance(entry_id, str) and entry_id, "manifest entry id missing")
        require(entry_id not in seen_ids, f"duplicate manifest id: {entry_id}")
        seen_ids.add(entry_id)
        require(entry.get("required") is True, f"manifest entry is not required: {entry_id}")
        require(entry.get("contains_secret_material") is False, f"manifest entry secret flag set: {entry_id}")
        content = entry.get("content")
        require(isinstance(content, str), f"manifest entry content missing: {entry_id}")
        expected = hashlib.sha256(content.encode("utf-8")).hexdigest()
        require(entry.get("sha256") == expected, f"manifest checksum mismatch: {entry_id}")
        require(checksum_by_id.get(entry_id) == expected, f"checksum table mismatch: {entry_id}")

    require(set(checksum_by_id) == seen_ids, "checksum ids do not match manifest ids")

    outcomes = require_mapping(data, "expected_validation_outcomes")
    require(outcomes.get("valid_fixture") == "PASS", "valid fixture outcome mismatch")
    for case in NEGATIVE_CASES:
        require(outcomes.get(case) == "FAIL_CLOSED", f"negative case outcome mismatch: {case}")

    negative_cases = data.get("tamper_negative_cases")
    require(isinstance(negative_cases, list), "tamper_negative_cases must be a list")
    names = {case.get("name") for case in negative_cases if isinstance(case, dict)}
    require(names == NEGATIVE_CASES, "negative case set mismatch")
    for case in negative_cases:
        require(case.get("expected") == "FAIL_CLOSED", f"negative case is not fail-closed: {case.get('name')}")

    cleanup = require_mapping(data, "cleanup_plan")
    require(cleanup.get("classification") == "TEMPORARY_PROOF_ONLY", "cleanup classification mismatch")
    require(cleanup.get("artifact_root") == "/srv/qbuild/tmp/NA-0359_*", "cleanup artifact root mismatch")
    require(cleanup.get("durable_artifacts_allowed") is False, "durable artifacts allowed")
    require(cleanup.get("restored_payloads_allowed") is False, "restored payloads allowed")
    require(cleanup.get("staging_cleanup_required") is True, "staging cleanup marker missing")

    monitoring = require_mapping(data, "monitoring_alert_plan")
    require(monitoring.get("live_monitoring_mutation_authorized") is False, "live monitoring mutation authorized")
    require(monitoring.get("marker") == "NA0359_MONITORING_ALERT_PLAN_MARKER_OK", "monitoring marker mismatch")

    runbook = require_mapping(data, "operator_runbook_summary")
    require(runbook.get("marker") == "NA0359_OPERATOR_RUNBOOK_MARKER_OK", "operator runbook marker mismatch")
    emergency_stop = runbook.get("emergency_stop")
    require(isinstance(emergency_stop, list) and emergency_stop, "emergency stop runbook missing")

    forbidden = set(data.get("forbidden_operations", []))
    require(REQUIRED_FORBIDDEN <= forbidden, "forbidden operation set incomplete")

    claims = require_mapping(data, "claim_boundaries")
    require(CLAIM_KEYS <= set(claims), "claim boundary set incomplete")
    for key in CLAIM_KEYS:
        claim = claims[key]
        require(isinstance(claim, dict), f"claim {key} must be an object")
        require(claim.get("status") in {"PROHIBITED", "NOT_READY", "FUTURE_GATE"}, f"claim {key} status invalid")
        require(claim.get("negated") is True, f"claim {key} is not negated")

    markers = data.get("required_markers")
    require(isinstance(markers, list), "required_markers must be a list")
    require(set(markers) == REQUIRED_MARKERS, "required marker set mismatch")

    impact = require_mapping(data, "backup_plan_impact")
    require(impact.get("update_required_now") is False, "backup-plan update required now")

    for boundary_name in ("qsl_server_boundary", "qsl_attachments_boundary"):
        boundary = require_mapping(data, boundary_name)
        require(boundary.get("mutation_authorized") is False, f"{boundary_name} mutation authorized")
        require(boundary.get("production_public_internet_proof") is False, f"{boundary_name} overclaims production proof")

    qshield = require_mapping(data, "qshield_demo_boundary")
    require(qshield.get("runtime_mutation_authorized") is False, "qshield runtime mutation authorized")
    require(qshield.get("production_proof") is False, "qshield demo overclaims production proof")

    other_fixture_strings = "\n".join(strings_except_sentinel_list(data))
    require(count_sentinel_leaks(other_fixture_strings, sentinels) == 0, "sentinel leaked outside sentinel list")
    require(count_secret_patterns(json.dumps(data, sort_keys=True)) == 0, "secret-like pattern found in fixture")


def expect_failure(name, mutated):
    try:
        validate_fixture(mutated)
    except AssertionError:
        return
    raise AssertionError(f"negative case did not fail closed: {name}")


def run_negative_cases(data):
    passed = []
    for case in data["tamper_negative_cases"]:
        name = case["name"]
        if name == "sentinel_leak_detection":
            proof = "proof line\n" + data["no_secret_sentinels"][0] + "\n"
            require(count_sentinel_leaks(proof, data["no_secret_sentinels"]) > 0, "sentinel leak detector did not trigger")
            passed.append(name)
            continue

        mutated = copy.deepcopy(data)
        if name == "missing_manifest":
            mutated.pop("manifest_entries", None)
        elif name == "checksum_mismatch":
            mutated["manifest_entries"][0]["sha256"] = "0" * 64
        elif name == "missing_cleanup_plan":
            mutated.pop("cleanup_plan", None)
        elif name == "prohibited_operation_field":
            mutated["restore_state_classification"]["operation_executed_by_harness"] = True
        elif name == "missing_claim_boundary":
            mutated["claim_boundaries"].pop("production_readiness", None)
        elif name == "missing_no_secret_fixture_marker":
            mutated["required_markers"].remove("NA0359_NO_SECRET_FIXTURE_OK")
        else:
            raise AssertionError(f"unknown negative case: {name}")
        expect_failure(name, mutated)
        passed.append(name)
    require(set(passed) == NEGATIVE_CASES, "not all negative cases passed")
    return passed


validate_fixture(fixture)
negative_passed = run_negative_cases(fixture)

artifact_file = artifact_dir / "na0359_restore_drill_dry_run_proof.txt"
proof_lines = [
    "NA0359_ARTIFACT_CLASS restore_drill_dry_run_no_secret_v1",
    "NA0359_ARTIFACT_PATH " + str(artifact_file),
    "NA0359_ARTIFACT_DIR " + str(artifact_dir),
    "NA0359_OPERATION_EXECUTED_COUNT 0",
    "NA0359_RESTORE_TARGET_CREATED_COUNT 0",
    "NA0359_RESTORE_TARGET_MOUNTED_COUNT 0",
    "NA0359_RESTORE_COPY_COUNT 0",
    "NA0359_KEY_OPERATION_COUNT 0",
    "NA0359_OFF_HOST_OPERATION_COUNT 0",
    "NA0359_DEPLOY_ROLLBACK_OPERATION_COUNT 0",
    "NA0359_BACKUP_PLAN_UPDATE_REQUIRED no",
    "NA0359_NEGATIVE_CASES_PASSED " + str(len(negative_passed)),
    "NA0359_NEGATIVE_CASE_NAMES " + ",".join(sorted(negative_passed)),
    "RESTORE_DRY_RUN_SECRET_FINDING_COUNT 0",
    "NA0359_SENTINEL_LEAK_FINDING_COUNT 0",
]
proof_lines.extend(fixture["required_markers"])
proof = "\n".join(proof_lines) + "\n"

require(count_sentinel_leaks(proof, fixture["no_secret_sentinels"]) == 0, "sentinel leaked into proof")
require(count_secret_patterns(proof) == 0, "secret-like pattern found in proof")

artifact_file.write_text(proof, encoding="utf-8")

print(proof, end="")
PY
