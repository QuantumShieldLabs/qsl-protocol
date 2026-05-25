#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT_DIR"

FIXTURE_FILE=${1:-inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json}
TMP_ROOT=${NA0361_HARNESS_TMP_ROOT:-/srv/qbuild/tmp}

case "$TMP_ROOT" in
  /srv/qbuild/tmp|/srv/qbuild/tmp/*) ;;
  *)
    echo "NA0361_HARNESS_TMP_ROOT must be under /srv/qbuild/tmp" >&2
    exit 1
    ;;
esac

if [ ! -f "$FIXTURE_FILE" ]; then
  echo "missing fixture file: $FIXTURE_FILE" >&2
  exit 1
fi

ARTIFACT_DIR=$(mktemp -d "$TMP_ROOT/NA-0361_key_custody_recovery_no_secret.XXXXXX")

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
    "local_backup_classification",
    "off_host_classification",
    "custody_mode",
    "recovery_mode",
    "simulated_key_ids",
    "simulated_custody_records",
    "simulated_recovery_envelope_metadata",
    "simulated_rotation_matrix",
    "simulated_old_archive_compatibility_matrix",
    "simulated_incident_response_cases",
    "simulated_emergency_access_cases",
    "operator_runbook_markers",
    "integrity_hashes",
    "operation_counters",
    "no_secret_sentinels",
    "expected_validation_outcomes",
    "tamper_negative_cases",
    "forbidden_operations",
    "claim_boundaries",
    "required_markers",
    "backup_plan_impact",
    "qsl_server_boundary",
    "qsl_attachments_boundary",
    "qshield_demo_boundary",
}

REQUIRED_MARKERS = {
    "NA0361_KEY_CUSTODY_AUTHORIZATION_OK",
    "NA0361_KEY_RECOVERY_AUTHORIZATION_OK",
    "NA0361_NO_SECRET_KEY_CUSTODY_HARNESS_OK",
    "NA0361_NO_SECRET_KEY_RECOVERY_HARNESS_OK",
    "NA0361_SIMULATED_CUSTODY_FIXTURE_OK",
    "NA0361_SIMULATED_RECOVERY_ENVELOPE_OK",
    "NA0361_SIMULATED_ROTATION_MATRIX_OK",
    "NA0361_INCIDENT_RESPONSE_MARKER_OK",
    "NA0361_OPERATOR_RUNBOOK_MARKER_OK",
    "NA0361_BACKUP_PLAN_IMPACT_OK",
    "NA0361_NO_REAL_KEY_GENERATION_OK",
    "NA0361_NO_KEY_UPLOAD_OK",
    "NA0361_NO_PASSPHRASE_COLLECTION_OK",
    "NA0361_NO_PRIVATE_KEY_INSPECTION_OK",
    "NA0361_NO_SECRET_MATERIAL_OK",
    "NA0361_NO_SECRET_ARTIFACT_OK",
    "NA0361_NO_REAL_KEY_CUSTODY_CLAIM_OK",
    "NA0361_NO_REAL_KEY_RECOVERY_CLAIM_OK",
    "NA0361_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK",
    "NA0361_NO_PRODUCTION_READY_CLAIM_OK",
    "NA0361_NO_PUBLIC_INTERNET_READY_CLAIM_OK",
    "NA0361_METADATA_RUNTIME_KEY_CUSTODY_RECOVERY_NO_SECRET_OK",
}

REQUIRED_FORBIDDEN = {
    "real_key_generation",
    "key_upload",
    "passphrase_collection",
    "private_key_inspection",
    "secret_material_handling",
    "real_recovery_envelope_content_creation",
    "backup_execution",
    "restore_execution",
    "restore_target_creation",
    "restore_target_mount",
    "restore_copy",
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
    "real_key_custody_implemented",
    "real_key_recovery_implemented",
}

NEGATIVE_CASES = {
    "missing_custody_record",
    "missing_recovery_metadata",
    "simulated_rotation_mismatch",
    "missing_old_archive_compatibility",
    "prohibited_operation_field",
    "missing_claim_boundary",
    "missing_no_secret_marker",
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


def require_list(data, name):
    value = data.get(name)
    require(isinstance(value, list) and value, f"{name} must be a non-empty list")
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


def hash_material(item):
    material = item.get("checksum_material")
    require(isinstance(material, str) and material, "checksum material missing")
    return hashlib.sha256(material.encode("utf-8")).hexdigest()


def validate_hashed_items(items, hash_by_id):
    seen_ids = set()
    for item in items:
        require(isinstance(item, dict), "hashed item must be an object")
        item_id = item.get("record_id") or item.get("metadata_id") or item.get("rotation_id") or item.get("archive_id") or item.get("case_id")
        require(isinstance(item_id, str) and item_id, "hashed item id missing")
        require(item_id not in seen_ids, f"duplicate hashed item id: {item_id}")
        seen_ids.add(item_id)
        expected = hash_material(item)
        require(item.get("sha256") == expected, f"item hash mismatch: {item_id}")
        require(hash_by_id.get(item_id) == expected, f"integrity hash table mismatch: {item_id}")
    return seen_ids


def validate_fixture(data):
    missing = sorted(REQUIRED_FIELDS - set(data))
    require(not missing, f"missing required fields: {', '.join(missing)}")
    require(
        data.get("schema_version") == "qsl.metadata_runtime.key_custody_recovery_no_secret_fixture.v1",
        "schema_version mismatch",
    )
    require(data.get("artifact_class") == "key_custody_recovery_no_secret_v1", "artifact_class mismatch")
    require(set(data.get("goals", [])) == {"G1", "G2", "G3", "G4", "G5"}, "goals mismatch")
    require(data.get("custody_mode") == "simulated only", "custody mode mismatch")
    require(data.get("recovery_mode") == "simulated only", "recovery mode mismatch")

    authorization = require_mapping(data, "authorization")
    for key in (
        "real_key_generation_authorized",
        "key_upload_authorized",
        "passphrase_collection_authorized",
        "private_key_inspection_authorized",
        "secret_material_handling_authorized",
        "real_recovery_envelope_content_authorized",
        "backup_authorized",
        "restore_authorized",
        "off_host_authorized",
        "deploy_authorized",
        "rollback_authorized",
    ):
        require(authorization.get(key) is False, f"authorization flag is not false: {key}")
    require(authorization.get("custody_mode") == "simulated only", "authorization custody mode mismatch")
    require(authorization.get("recovery_mode") == "simulated only", "authorization recovery mode mismatch")

    sources = require_mapping(data, "source_classification")
    require(sources.get("qsl_protocol") == "FRESH_SOURCE", "qsl-protocol source is not fresh")
    require(sources.get("qsl_server") == "FRESH_SOURCE", "qsl-server source is not fresh")
    require(sources.get("qsl_attachments") == "FRESH_SOURCE", "qsl-attachments source is not fresh")
    require(sources.get("qshield_demo") == "REFERENCE_ORACLE_ONLY", "qshield demo boundary mismatch")

    local_backup = require_mapping(data, "local_backup_classification")
    require(local_backup.get("local_continuity") == "LOCAL_CONTINUITY_PROVEN", "local continuity classification mismatch")
    require(local_backup.get("backup_scope") == "SAME_HOST_CONTINUITY_ONLY", "backup scope mismatch")
    require(local_backup.get("restore_dry_run") == "NO_SECRET_DRY_RUN_RESTORE_PROVEN", "dry-run restore classification mismatch")
    require(local_backup.get("real_restore") == "REAL_RESTORE_NOT_AUTHORIZED", "real restore classification mismatch")
    require(local_backup.get("local_backup_mutation_authorized") is False, "local backup mutation authorized")

    off_host = require_mapping(data, "off_host_classification")
    require(off_host.get("off_host_backup") == "OFF_HOST_BACKUP_NOT_READY", "off-host backup classification mismatch")
    require(off_host.get("target_configured") is False, "off-host target configured")
    require(off_host.get("remote_endpoint_present") is False, "off-host endpoint present")
    require(off_host.get("operation_authorized") is False, "off-host operation authorized")

    key_ids = require_list(data, "simulated_key_ids")
    key_by_id = {}
    for item in key_ids:
        require(isinstance(item, dict), "simulated key id must be an object")
        key_id = item.get("id")
        require(isinstance(key_id, str) and key_id.startswith("sim-key-"), "invalid simulated key id")
        require(key_id not in key_by_id, f"duplicate simulated key id: {key_id}")
        require(item.get("contains_secret_material") is False, f"simulated key contains secret material: {key_id}")
        key_by_id[key_id] = item

    hashes = require_mapping(data, "integrity_hashes")
    require(hashes.get("algorithm") == "sha256", "integrity hash algorithm mismatch")
    hash_entries = require_list(hashes, "entries")
    hash_by_id = {}
    for entry in hash_entries:
        require(isinstance(entry, dict), "integrity hash entry must be an object")
        hash_by_id[entry.get("id")] = entry.get("sha256")

    custody_records = require_list(data, "simulated_custody_records")
    custody_ids = validate_hashed_items(custody_records, hash_by_id)
    for record in custody_records:
        require(record.get("simulated_key_id") in key_by_id, "custody record references unknown key id")
        require(record.get("custody_state") == "SIMULATED_OPERATOR_HELD", "custody state mismatch")
        require(record.get("secret_material_present") is False, "custody record contains secret material")
        require(record.get("recovery_envelope_content_present") is False, "custody record contains recovery envelope content")
        require(record.get("marker") == "NA0361_SIMULATED_CUSTODY_FIXTURE_OK", "custody marker mismatch")

    recovery_metadata = require_list(data, "simulated_recovery_envelope_metadata")
    recovery_ids = validate_hashed_items(recovery_metadata, hash_by_id)
    recovery_by_id = {item["metadata_id"]: item for item in recovery_metadata}
    for record in recovery_metadata:
        require(record.get("simulated_key_id") in key_by_id, "recovery metadata references unknown key id")
        require(record.get("envelope_class") == "metadata-only", "recovery envelope class mismatch")
        require(record.get("secret_material_present") is False, "recovery metadata contains secret material")
        require(record.get("envelope_content_present") is False, "recovery metadata contains envelope content")
        require(record.get("passphrase_present") is False, "recovery metadata contains passphrase")
        require(record.get("marker") == "NA0361_SIMULATED_RECOVERY_ENVELOPE_OK", "recovery marker mismatch")

    rotations = require_list(data, "simulated_rotation_matrix")
    rotation_ids = validate_hashed_items(rotations, hash_by_id)
    for rotation in rotations:
        require(rotation.get("from_key_id") in key_by_id, "rotation references unknown from key")
        require(rotation.get("to_key_id") in key_by_id, "rotation references unknown to key")
        require(rotation.get("from_key_id") != rotation.get("to_key_id"), "rotation key ids must differ")
        require(rotation.get("compatible_recovery_metadata_id") in recovery_by_id, "rotation references unknown recovery metadata")
        require(rotation.get("secret_material_present") is False, "rotation contains secret material")
        require(rotation.get("marker") == "NA0361_SIMULATED_ROTATION_MATRIX_OK", "rotation marker mismatch")

    old_archives = require_list(data, "simulated_old_archive_compatibility_matrix")
    archive_ids = validate_hashed_items(old_archives, hash_by_id)
    archive_by_id = {item["archive_id"]: item for item in old_archives}
    for archive in old_archives:
        require(archive.get("required_key_id") in key_by_id, "old archive references unknown key")
        require(archive.get("recovery_metadata_id") in recovery_by_id, "old archive references unknown recovery metadata")
        require(archive.get("compatibility_state") == "SIMULATED_COMPATIBLE", "old archive compatibility mismatch")
        require(archive.get("secret_material_present") is False, "old archive contains secret material")
    for rotation in rotations:
        archive = archive_by_id.get(rotation.get("old_archive_id"))
        require(archive is not None, "rotation references missing old archive")
        require(archive.get("required_key_id") == rotation.get("from_key_id"), "old archive required key does not match rotation from key")

    incidents = require_list(data, "simulated_incident_response_cases")
    incident_ids = validate_hashed_items(incidents, hash_by_id)
    for incident in incidents:
        require(incident.get("secret_material_present") is False, "incident case contains secret material")
        require(incident.get("marker") == "NA0361_INCIDENT_RESPONSE_MARKER_OK", "incident marker mismatch")

    emergency_cases = require_list(data, "simulated_emergency_access_cases")
    emergency_ids = validate_hashed_items(emergency_cases, hash_by_id)
    for emergency in emergency_cases:
        require(emergency.get("secret_material_present") is False, "emergency case contains secret material")
        require(emergency.get("marker") == "NA0361_OPERATOR_RUNBOOK_MARKER_OK", "emergency marker mismatch")

    expected_hash_ids = custody_ids | recovery_ids | rotation_ids | archive_ids | incident_ids | emergency_ids
    require(set(hash_by_id) == expected_hash_ids, "integrity hash ids do not match simulated records")

    runbook = require_mapping(data, "operator_runbook_markers")
    require(runbook.get("classification") == "NO_SECRET_OPERATOR_SUMMARY", "runbook classification mismatch")
    require(runbook.get("marker") == "NA0361_OPERATOR_RUNBOOK_MARKER_OK", "operator runbook marker mismatch")
    require(isinstance(runbook.get("emergency_stop"), list) and runbook["emergency_stop"], "emergency stop runbook missing")

    counters = require_mapping(data, "operation_counters")
    require(set(counters) >= {
        "real_key_generation",
        "key_upload",
        "passphrase_collection",
        "private_key_inspection",
        "secret_material_handling",
        "real_recovery_envelope_content_creation",
        "backup_execution",
        "restore_execution",
        "restore_target_creation",
        "restore_target_mount",
        "restore_copy",
        "off_host_setup",
        "deploy",
        "rollback",
        "local_backup_mutation",
    }, "operation counter set incomplete")
    for key, value in counters.items():
        require(value == 0, f"operation counter is nonzero: {key}")

    sentinels = require_list(data, "no_secret_sentinels")
    require(len(set(sentinels)) == len(sentinels), "duplicate no-secret sentinels")
    for sentinel in sentinels:
        require(isinstance(sentinel, str) and sentinel.startswith("NA0361_SECRET_SENTINEL_"), "invalid sentinel label")

    outcomes = require_mapping(data, "expected_validation_outcomes")
    require(outcomes.get("valid_fixture") == "PASS", "valid fixture outcome mismatch")
    for case in NEGATIVE_CASES:
        require(outcomes.get(case) == "FAIL_CLOSED", f"negative case outcome mismatch: {case}")

    negative_cases = require_list(data, "tamper_negative_cases")
    names = {case.get("name") for case in negative_cases if isinstance(case, dict)}
    require(names == NEGATIVE_CASES, "negative case set mismatch")
    for case in negative_cases:
        require(case.get("expected") == "FAIL_CLOSED", f"negative case is not fail-closed: {case.get('name')}")

    forbidden = set(data.get("forbidden_operations", []))
    require(REQUIRED_FORBIDDEN <= forbidden, "forbidden operation set incomplete")

    claims = require_mapping(data, "claim_boundaries")
    require(CLAIM_KEYS <= set(claims), "claim boundary set incomplete")
    for key in CLAIM_KEYS:
        claim = claims[key]
        require(isinstance(claim, dict), f"claim {key} must be an object")
        require(claim.get("status") in {"PROHIBITED", "NOT_READY", "FUTURE_GATE"}, f"claim {key} status invalid")
        require(claim.get("negated") is True, f"claim {key} is not negated")

    markers = require_list(data, "required_markers")
    require(set(markers) == REQUIRED_MARKERS, "required marker set mismatch")

    impact = require_mapping(data, "backup_plan_impact")
    require(impact.get("update_required_now") is False, "backup-plan update required now")
    require(impact.get("marker") == "NA0361_BACKUP_PLAN_IMPACT_OK", "backup-plan marker mismatch")

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
        if name == "missing_custody_record":
            mutated.pop("simulated_custody_records", None)
        elif name == "missing_recovery_metadata":
            mutated.pop("simulated_recovery_envelope_metadata", None)
        elif name == "simulated_rotation_mismatch":
            mutated["simulated_rotation_matrix"][0]["to_key_id"] = "sim-key-missing"
        elif name == "missing_old_archive_compatibility":
            mutated.pop("simulated_old_archive_compatibility_matrix", None)
        elif name == "prohibited_operation_field":
            mutated["operation_counters"]["real_key_generation"] = 1
        elif name == "missing_claim_boundary":
            mutated["claim_boundaries"].pop("production_readiness", None)
        elif name == "missing_no_secret_marker":
            mutated["required_markers"].remove("NA0361_NO_SECRET_MATERIAL_OK")
        else:
            raise AssertionError(f"unknown negative case: {name}")
        expect_failure(name, mutated)
        passed.append(name)
    require(set(passed) == NEGATIVE_CASES, "not all negative cases passed")
    return passed


validate_fixture(fixture)
negative_passed = run_negative_cases(fixture)

artifact_file = artifact_dir / "na0361_key_custody_recovery_no_secret_proof.txt"
proof_lines = [
    "NA0361_ARTIFACT_CLASS key_custody_recovery_no_secret_v1",
    "NA0361_ARTIFACT_PATH " + str(artifact_file),
    "NA0361_ARTIFACT_DIR " + str(artifact_dir),
    "NA0361_OPERATION_EXECUTED_COUNT 0",
    "NA0361_REAL_KEY_GENERATION_COUNT 0",
    "NA0361_KEY_UPLOAD_COUNT 0",
    "NA0361_PASSPHRASE_COLLECTION_COUNT 0",
    "NA0361_PRIVATE_KEY_INSPECTION_COUNT 0",
    "NA0361_SECRET_MATERIAL_HANDLING_COUNT 0",
    "NA0361_RECOVERY_ENVELOPE_CONTENT_CREATION_COUNT 0",
    "NA0361_BACKUP_OPERATION_COUNT 0",
    "NA0361_RESTORE_OPERATION_COUNT 0",
    "NA0361_RESTORE_TARGET_CREATED_COUNT 0",
    "NA0361_RESTORE_TARGET_MOUNTED_COUNT 0",
    "NA0361_RESTORE_COPY_COUNT 0",
    "NA0361_OFF_HOST_OPERATION_COUNT 0",
    "NA0361_DEPLOY_ROLLBACK_OPERATION_COUNT 0",
    "NA0361_LOCAL_BACKUP_MUTATION_COUNT 0",
    "NA0361_BACKUP_PLAN_UPDATE_REQUIRED no",
    "NA0361_NEGATIVE_CASES_PASSED " + str(len(negative_passed)),
    "NA0361_NEGATIVE_CASE_NAMES " + ",".join(sorted(negative_passed)),
    "KEY_CUSTODY_RECOVERY_SECRET_FINDING_COUNT 0",
    "NA0361_SENTINEL_LEAK_FINDING_COUNT 0",
]
proof_lines.extend(fixture["required_markers"])
proof = "\n".join(proof_lines) + "\n"

require(count_sentinel_leaks(proof, fixture["no_secret_sentinels"]) == 0, "sentinel leaked into proof")
require(count_secret_patterns(proof) == 0, "secret-like pattern found in proof")

artifact_file.write_text(proof, encoding="utf-8")

print(proof, end="")
PY
