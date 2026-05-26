#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT_DIR"

FIXTURE_FILE=${1:-inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json}
TMP_ROOT=${NA0365_HARNESS_TMP_ROOT:-/srv/qbuild/tmp}

case "$TMP_ROOT" in
  /srv/qbuild/tmp|/srv/qbuild/tmp/*) ;;
  *)
    echo "NA0365_HARNESS_TMP_ROOT must be under /srv/qbuild/tmp" >&2
    exit 1
    ;;
esac

if [ ! -f "$FIXTURE_FILE" ]; then
  echo "missing fixture file: $FIXTURE_FILE" >&2
  exit 1
fi

ARTIFACT_DIR=$(mktemp -d "$TMP_ROOT/NA-0365_restore_drill_isolated_restore_no_secret.XXXXXX")

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

fixture_text = fixture_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_text)

REQUIRED_FIELDS = {
    "schema_version",
    "artifact_class",
    "goals",
    "authorization",
    "source_classification",
    "local_backup_classification",
    "key_custody_classification",
    "off_host_target_tool_classification",
    "restore_classification",
    "restore_mode",
    "restore_target_mode",
    "simulated_restore_target_metadata",
    "simulated_manifest_metadata",
    "simulated_checksum_metadata",
    "simulated_old_archive_compatibility_metadata",
    "simulated_cleanup_metadata",
    "simulated_monitoring_alert_metadata",
    "simulated_operator_runbook_metadata",
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
    "selected_successor",
}

REQUIRED_MARKERS = {
    "NA0365_ISOLATED_RESTORE_AUTHORIZATION_OK",
    "NA0365_NO_SECRET_ISOLATED_RESTORE_HARNESS_OK",
    "NA0365_NO_SECRET_RESTORE_TARGET_HARNESS_OK",
    "NA0365_SIMULATED_RESTORE_TARGET_OK",
    "NA0365_SIMULATED_MANIFEST_CHECKSUM_RESTORE_OK",
    "NA0365_SIMULATED_OLD_ARCHIVE_COMPATIBILITY_OK",
    "NA0365_SIMULATED_CLEANUP_MONITORING_RUNBOOK_OK",
    "NA0365_BACKUP_PLAN_IMPACT_OK",
    "NA0365_NO_REAL_RESTORE_TARGET_CREATION_OK",
    "NA0365_NO_MOUNT_OK",
    "NA0365_NO_COPY_OK",
    "NA0365_NO_REAL_BACKUP_OK",
    "NA0365_NO_REAL_RESTORE_OK",
    "NA0365_NO_KEY_GENERATION_OK",
    "NA0365_NO_PASSPHRASE_COLLECTION_OK",
    "NA0365_NO_SECRET_MATERIAL_OK",
    "NA0365_NO_RESTORE_DRILL_COMPLETE_CLAIM_OK",
    "NA0365_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK",
    "NA0365_NO_PRODUCTION_READY_CLAIM_OK",
    "NA0365_NO_PUBLIC_INTERNET_READY_CLAIM_OK",
    "NA0365_METADATA_RUNTIME_ISOLATED_RESTORE_NO_SECRET_OK",
}

REQUIRED_FORBIDDEN = {
    "restore_target_creation",
    "mount",
    "copy",
    "real_backup",
    "real_restore",
    "remote_connection",
    "repository_init",
    "tool_installation",
    "key_generation",
    "key_upload",
    "passphrase_collection",
    "private_key_inspection",
    "secret_material_handling",
    "recovery_envelope_content_creation",
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

AUTHORIZATION_FALSE_KEYS = {
    "real_restore_target_creation_authorized",
    "restore_target_mount_authorized",
    "restore_copy_authorized",
    "real_backup_authorized",
    "real_restore_authorized",
    "remote_connection_authorized",
    "repository_init_authorized",
    "tool_installation_authorized",
    "key_generation_authorized",
    "key_upload_authorized",
    "passphrase_collection_authorized",
    "private_key_inspection_authorized",
    "secret_material_handling_authorized",
    "recovery_envelope_content_authorized",
    "off_host_setup_authorized",
    "deploy_authorized",
    "rollback_authorized",
}

OPERATION_COUNTER_KEYS = {
    "real_restore_target_creation",
    "restore_target_mount",
    "restore_copy",
    "real_backup",
    "real_restore",
    "remote_connection",
    "repository_init",
    "tool_installation",
    "key_generation",
    "key_upload",
    "passphrase_collection",
    "private_key_inspection",
    "secret_material_handling",
    "recovery_envelope_content_creation",
    "off_host_setup",
    "deploy",
    "rollback",
    "local_backup_mutation",
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
    "real_restore_complete",
    "real_key_custody_implemented",
    "real_key_recovery_implemented",
}

NEGATIVE_CASES = {
    "missing_restore_target_metadata",
    "missing_manifest",
    "checksum_mismatch",
    "missing_old_archive_compatibility",
    "missing_cleanup",
    "missing_monitoring_alert",
    "prohibited_operation_field",
    "mount_attempted",
    "copy_attempted",
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


def validate_hashed_items(items, hash_by_id, id_key):
    seen_ids = set()
    for item in items:
        require(isinstance(item, dict), "hashed item must be an object")
        item_id = item.get(id_key)
        require(isinstance(item_id, str) and item_id, f"{id_key} missing")
        require(item_id not in seen_ids, f"duplicate hashed item id: {item_id}")
        seen_ids.add(item_id)
        expected = hash_material(item)
        require(item.get("sha256") == expected, f"item hash mismatch: {item_id}")
        require(hash_by_id.get(item_id) == expected, f"integrity hash table mismatch: {item_id}")
        require(item.get("secret_material_present") is False, f"secret material flag set: {item_id}")
    return seen_ids


def validate_base(data):
    missing = sorted(REQUIRED_FIELDS - set(data))
    require(not missing, f"missing required fields: {', '.join(missing)}")
    require(
        data.get("schema_version")
        == "qsl.metadata_runtime.restore_drill_isolated_restore_no_secret_fixture.v1",
        "schema_version mismatch",
    )
    require(data.get("artifact_class") == "restore_drill_isolated_restore_no_secret_v1", "artifact_class mismatch")
    require(set(data.get("goals", [])) == {"G1", "G2", "G3", "G4", "G5"}, "goals mismatch")
    require(data.get("restore_mode") == "simulated isolated restore only", "restore mode mismatch")
    require(data.get("restore_target_mode") == "simulated only", "restore target mode mismatch")

    authorization = require_mapping(data, "authorization")
    for key in AUTHORIZATION_FALSE_KEYS:
        require(authorization.get(key) is False, f"authorization flag is not false: {key}")
    require(authorization.get("restore_mode") == "simulated isolated restore only", "authorization restore mode mismatch")
    require(authorization.get("restore_target_mode") == "simulated only", "authorization target mode mismatch")

    sources = require_mapping(data, "source_classification")
    require(sources.get("qsl_protocol") == "FRESH_SOURCE", "qsl-protocol source is not fresh")
    require(sources.get("qsl_server") == "FRESH_SOURCE", "qsl-server source is not fresh")
    require(sources.get("qsl_attachments") == "FRESH_SOURCE", "qsl-attachments source is not fresh")
    require(sources.get("qshield_demo") == "REFERENCE_ORACLE_ONLY", "qshield demo boundary mismatch")

    local_backup = require_mapping(data, "local_backup_classification")
    require(local_backup.get("local_continuity") == "LOCAL_CONTINUITY_PROVEN", "local continuity mismatch")
    require(local_backup.get("backup_scope") == "SAME_HOST_CONTINUITY_ONLY", "backup scope mismatch")
    require(local_backup.get("restore_dry_run") == "NO_SECRET_DRY_RUN_RESTORE_PROVEN", "restore dry-run mismatch")
    require(local_backup.get("local_backup_mutation_authorized") is False, "local backup mutation authorized")

    key_state = require_mapping(data, "key_custody_classification")
    require(
        key_state.get("no_secret_key_custody_recovery") == "NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN",
        "no-secret key custody classification mismatch",
    )
    require(key_state.get("real_key_custody") == "REAL_KEY_CUSTODY_NOT_READY", "real key custody mismatch")
    require(key_state.get("real_key_recovery") == "REAL_KEY_RECOVERY_NOT_READY", "real key recovery mismatch")

    off_host = require_mapping(data, "off_host_target_tool_classification")
    require(off_host.get("no_secret_target_tool") == "NO_SECRET_TARGET_TOOL_PROVEN", "target/tool proof mismatch")
    require(off_host.get("real_off_host_target") == "OFF_HOST_BACKUP_NOT_READY", "off-host readiness mismatch")
    require(off_host.get("remote_connection_authorized") is False, "remote connection authorized")

    restore = require_mapping(data, "restore_classification")
    require(
        restore.get("isolated_restore_harness") == "NO_SECRET_ISOLATED_RESTORE_HARNESS_READY",
        "isolated restore harness classification mismatch",
    )
    require(restore.get("real_isolated_restore") == "REAL_ISOLATED_RESTORE_BLOCKED", "real isolated restore mismatch")
    require(restore.get("real_restore") == "REAL_RESTORE_NOT_AUTHORIZED", "real restore mismatch")
    require(restore.get("restore_target") == "RESTORE_TARGET_NOT_CREATED", "restore target mismatch")
    require(restore.get("operation_executed_by_harness") is False, "operation executed flag is true")

    sentinels = require_list(data, "no_secret_sentinels")
    require(len(set(sentinels)) == len(sentinels), "duplicate no-secret sentinels")
    for sentinel in sentinels:
        require(isinstance(sentinel, str) and sentinel.startswith("NA0365_SECRET_SENTINEL_"), "invalid sentinel label")

    hash_table = require_mapping(data, "integrity_hashes")
    require(hash_table.get("algorithm") == "sha256", "integrity hash algorithm mismatch")
    hash_entries = require_list(hash_table, "entries")
    hash_by_id = {}
    for entry in hash_entries:
        require(isinstance(entry, dict), "integrity hash entry must be an object")
        entry_id = entry.get("id")
        require(isinstance(entry_id, str) and entry_id, "integrity hash id missing")
        require(entry_id not in hash_by_id, f"duplicate integrity hash id: {entry_id}")
        hash_by_id[entry_id] = entry.get("sha256")

    targets = require_list(data, "simulated_restore_target_metadata")
    target_ids = validate_hashed_items(targets, hash_by_id, "target_id")
    for target in targets:
        require(target.get("target_class") == "simulated_isolated_restore_target", "target class mismatch")
        require(target.get("target_created") is False, "restore target was created")
        require(target.get("target_mounted") is False, "restore target was mounted")
        require(target.get("restore_payload_copied") is False, "restore payload was copied")
        require(target.get("real_path_present") is False, "real target path present")
        require(target.get("mount_path_present") is False, "mount path present")
        require(target.get("marker") == "NA0365_SIMULATED_RESTORE_TARGET_OK", "target marker mismatch")

    manifests = require_list(data, "simulated_manifest_metadata")
    manifest_ids = validate_hashed_items(manifests, hash_by_id, "manifest_id")
    manifest_by_id = {item["manifest_id"]: item for item in manifests}
    for manifest in manifests:
        require(manifest.get("target_id") in target_ids, f"manifest target reference mismatch: {manifest.get('manifest_id')}")
        require(manifest.get("required") is True, "manifest is not required")
        require(manifest.get("contains_backup_payload") is False, "manifest contains backup payload")
        content = manifest.get("content")
        require(isinstance(content, str) and content, "manifest content missing")
        expected_content_hash = hashlib.sha256(content.encode("utf-8")).hexdigest()
        require(manifest.get("content_sha256") == expected_content_hash, "manifest content hash mismatch")
        require(manifest.get("marker") == "NA0365_SIMULATED_MANIFEST_CHECKSUM_RESTORE_OK", "manifest marker mismatch")

    checksums = require_list(data, "simulated_checksum_metadata")
    checksum_ids = validate_hashed_items(checksums, hash_by_id, "checksum_id")
    require(checksum_ids, "checksum ids missing")
    seen_checksum_manifest_ids = set()
    for checksum in checksums:
        manifest_id = checksum.get("manifest_id")
        require(manifest_id in manifest_by_id, f"checksum manifest reference mismatch: {manifest_id}")
        seen_checksum_manifest_ids.add(manifest_id)
        require(checksum.get("algorithm") == "sha256", "checksum algorithm mismatch")
        require(
            checksum.get("expected_content_sha256") == manifest_by_id[manifest_id].get("content_sha256"),
            f"checksum content hash mismatch: {manifest_id}",
        )
        require(checksum.get("marker") == "NA0365_SIMULATED_MANIFEST_CHECKSUM_RESTORE_OK", "checksum marker mismatch")
    require(seen_checksum_manifest_ids == manifest_ids, "checksum manifest ids do not cover manifests")

    archives = require_list(data, "simulated_old_archive_compatibility_metadata")
    validate_hashed_items(archives, hash_by_id, "archive_id")
    for archive in archives:
        require(archive.get("manifest_id") in manifest_ids, "old archive manifest reference mismatch")
        require(archive.get("compatibility_state") == "SIMULATED_COMPATIBLE", "old archive compatibility mismatch")
        require(archive.get("old_archive_payload_present") is False, "old archive payload present")
        require(
            archive.get("marker") == "NA0365_SIMULATED_OLD_ARCHIVE_COMPATIBILITY_OK",
            "old archive marker mismatch",
        )

    cleanups = require_list(data, "simulated_cleanup_metadata")
    validate_hashed_items(cleanups, hash_by_id, "cleanup_id")
    cleanup_target_ids = set()
    for cleanup in cleanups:
        target_id = cleanup.get("target_id")
        require(target_id in target_ids, "cleanup target reference mismatch")
        cleanup_target_ids.add(target_id)
        require(cleanup.get("cleanup_required_before_real_restore") is True, "cleanup requirement missing")
        require(cleanup.get("cleanup_operation_executed") is False, "cleanup operation executed")
        require(cleanup.get("target_created") is False, "cleanup target created flag is true")
        require(cleanup.get("target_mounted") is False, "cleanup target mounted flag is true")
        require(cleanup.get("restore_payload_copied") is False, "cleanup copy flag is true")
        require(
            cleanup.get("marker") == "NA0365_SIMULATED_CLEANUP_MONITORING_RUNBOOK_OK",
            "cleanup marker mismatch",
        )
    require(cleanup_target_ids == target_ids, "cleanup metadata does not cover restore targets")

    monitors = require_list(data, "simulated_monitoring_alert_metadata")
    validate_hashed_items(monitors, hash_by_id, "monitor_id")
    monitor_target_ids = set()
    for monitor in monitors:
        target_id = monitor.get("target_id")
        require(target_id in target_ids, "monitor target reference mismatch")
        monitor_target_ids.add(target_id)
        require(monitor.get("live_monitoring_mutation_authorized") is False, "monitoring mutation authorized")
        require(monitor.get("alert_channel_configured") is False, "alert channel configured")
        for key in ("alert_on_missing_manifest", "alert_on_checksum_mismatch", "alert_on_mount_attempt", "alert_on_copy_attempt"):
            require(monitor.get(key) is True, f"monitor alert flag missing: {key}")
        require(
            monitor.get("marker") == "NA0365_SIMULATED_CLEANUP_MONITORING_RUNBOOK_OK",
            "monitor marker mismatch",
        )
    require(monitor_target_ids == target_ids, "monitor metadata does not cover restore targets")

    runbook = require_mapping(data, "simulated_operator_runbook_metadata")
    validate_hashed_items([runbook], hash_by_id, "runbook_id")
    require(runbook.get("classification") == "NO_SECRET_OPERATOR_SUMMARY", "runbook classification mismatch")
    require(isinstance(runbook.get("emergency_stop"), list) and runbook["emergency_stop"], "runbook emergency stop missing")
    require(isinstance(runbook.get("operator_steps"), list) and runbook["operator_steps"], "runbook steps missing")
    require(
        runbook.get("marker") == "NA0365_SIMULATED_CLEANUP_MONITORING_RUNBOOK_OK",
        "runbook marker mismatch",
    )

    operation_counters = require_mapping(data, "operation_counters")
    require(set(operation_counters) == OPERATION_COUNTER_KEYS, "operation counter key set mismatch")
    for key, value in operation_counters.items():
        require(value == 0, f"operation counter is non-zero: {key}")

    forbidden = set(require_list(data, "forbidden_operations"))
    missing_forbidden = sorted(REQUIRED_FORBIDDEN - forbidden)
    require(not missing_forbidden, f"missing forbidden operations: {', '.join(missing_forbidden)}")

    claim_boundaries = require_mapping(data, "claim_boundaries")
    require(set(claim_boundaries) == CLAIM_KEYS, "claim boundary key set mismatch")
    for key, value in claim_boundaries.items():
        require(isinstance(value, dict), f"claim boundary is not an object: {key}")
        require(value.get("status") == "PROHIBITED", f"claim boundary is not prohibited: {key}")
        require(value.get("negated") is True, f"claim boundary is not negated: {key}")

    markers = set(require_list(data, "required_markers"))
    missing_markers = sorted(REQUIRED_MARKERS - markers)
    require(not missing_markers, f"missing markers: {', '.join(missing_markers)}")

    outcomes = require_mapping(data, "expected_validation_outcomes")
    require(outcomes.get("valid_fixture") == "PASS", "valid fixture outcome mismatch")
    for case in NEGATIVE_CASES:
        require(outcomes.get(case) == "FAIL_CLOSED", f"negative case outcome mismatch: {case}")

    negative_cases = require_list(data, "tamper_negative_cases")
    case_names = {case.get("name") for case in negative_cases if isinstance(case, dict)}
    require(case_names == NEGATIVE_CASES, "negative case set mismatch")
    for case in negative_cases:
        require(case.get("expected") == "FAIL_CLOSED", f"negative case expectation mismatch: {case.get('name')}")

    backup_plan = require_mapping(data, "backup_plan_impact")
    require(
        backup_plan.get("classification") == "NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW",
        "backup-plan impact classification mismatch",
    )
    future_update = backup_plan.get("future_update_required_for")
    require(isinstance(future_update, list) and future_update, "future backup-plan requirements missing")

    qsl_server = require_mapping(data, "qsl_server_boundary")
    require(qsl_server.get("mutation_authorized") is False, "qsl-server mutation authorized")
    require(qsl_server.get("classification") == "BOUNDED_END_TO_END_HARNESS_EVIDENCE_ONLY", "qsl-server boundary mismatch")

    qsl_attachments = require_mapping(data, "qsl_attachments_boundary")
    require(qsl_attachments.get("mutation_authorized") is False, "qsl-attachments mutation authorized")
    require(
        qsl_attachments.get("classification") == "SERVICE_LOCAL_PREREQUISITE_EVIDENCE_ONLY",
        "qsl-attachments boundary mismatch",
    )

    qshield = require_mapping(data, "qshield_demo_boundary")
    require(qshield.get("runtime_mutation_authorized") is False, "qshield runtime mutation authorized")
    require(qshield.get("classification") == "REFERENCE_ORACLE_ONLY", "qshield boundary mismatch")

    require(
        data.get("selected_successor")
        == "NA-0366 -- Metadata Runtime Off-Host Encrypted Backup Real Target / Tool Blocker Resolution",
        "selected successor mismatch",
    )

    searchable_text = "\n".join(strings_except_sentinel_list(data))
    require(count_secret_patterns(searchable_text) == 0, "secret-shaped pattern found in fixture")
    for sentinel in sentinels:
        require(sentinel not in searchable_text, f"sentinel leaked outside sentinel list: {sentinel}")


def apply_negative_case(data, name):
    mutated = copy.deepcopy(data)
    if name == "missing_restore_target_metadata":
        mutated.pop("simulated_restore_target_metadata", None)
    elif name == "missing_manifest":
        mutated.pop("simulated_manifest_metadata", None)
    elif name == "checksum_mismatch":
        mutated["simulated_checksum_metadata"][0]["expected_content_sha256"] = "0" * 64
    elif name == "missing_old_archive_compatibility":
        mutated.pop("simulated_old_archive_compatibility_metadata", None)
    elif name == "missing_cleanup":
        mutated.pop("simulated_cleanup_metadata", None)
    elif name == "missing_monitoring_alert":
        mutated.pop("simulated_monitoring_alert_metadata", None)
    elif name == "prohibited_operation_field":
        mutated["operation_counters"]["real_restore"] = 1
    elif name == "mount_attempted":
        mutated["simulated_restore_target_metadata"][0]["target_mounted"] = True
    elif name == "copy_attempted":
        mutated["simulated_restore_target_metadata"][0]["restore_payload_copied"] = True
    elif name == "missing_claim_boundary":
        mutated["claim_boundaries"].pop("production_readiness", None)
    elif name == "missing_no_secret_marker":
        mutated["required_markers"].remove("NA0365_NO_SECRET_MATERIAL_OK")
    else:
        raise AssertionError(f"unknown negative case: {name}")
    return mutated


def run_negative_cases(data):
    passed = 0
    for case in data["tamper_negative_cases"]:
        name = case["name"]
        if name == "sentinel_leak_detection":
            sentinels = data["no_secret_sentinels"]
            leak_text = f"proof leak: {sentinels[0]}"
            try:
                require(count_sentinel_leaks(leak_text, sentinels) == 0, "sentinel leaked into proof")
            except AssertionError:
                passed += 1
                continue
            raise AssertionError("sentinel leak negative case did not fail closed")
        mutated = apply_negative_case(data, name)
        try:
            validate_base(mutated)
        except AssertionError:
            passed += 1
            continue
        raise AssertionError(f"negative case did not fail closed: {name}")
    return passed


def proof_lines(data, fixture_sha, negative_count):
    markers = data["required_markers"]
    return [
        "NA0365 isolated restore no-secret proof",
        f"FIXTURE_SHA256 {fixture_sha}",
        "RESTORE_MODE simulated isolated restore only",
        "RESTORE_TARGET_MODE simulated only",
        "SOURCE_CLASSIFICATION qsl-protocol=FRESH_SOURCE qsl-server=FRESH_SOURCE qsl-attachments=FRESH_SOURCE",
        "LOCAL_BACKUP_CLASSIFICATION LOCAL_CONTINUITY_PROVEN SAME_HOST_CONTINUITY_ONLY",
        "NO_SECRET_DRY_RUN_RESTORE_PROVEN yes",
        "NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN yes",
        "NO_SECRET_TARGET_TOOL_PROVEN yes",
        "REAL_ISOLATED_RESTORE_BLOCKED yes",
        "REAL_KEY_CUSTODY_NOT_READY yes",
        "REAL_KEY_RECOVERY_NOT_READY yes",
        "OFF_HOST_BACKUP_NOT_READY yes",
        "NA0365_OPERATION_EXECUTED_COUNT 0",
        "NA0365_REAL_RESTORE_TARGET_CREATED_COUNT 0",
        "NA0365_MOUNT_ATTEMPT_COUNT 0",
        "NA0365_COPY_ATTEMPT_COUNT 0",
        "NA0365_REAL_BACKUP_COUNT 0",
        "NA0365_REAL_RESTORE_COUNT 0",
        "NA0365_KEY_OPERATION_COUNT 0",
        "NA0365_OFF_HOST_OPERATION_COUNT 0",
        "ISOLATED_RESTORE_SECRET_FINDING_COUNT 0",
        f"NA0365_NEGATIVE_CASES_PASSED {negative_count}",
        "BACKUP_PLAN_IMPACT NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW",
        "PUBLIC_CLAIM_BOUNDARY no production/public-internet/external-review/anonymity/metadata-free/untraceable/hidden-size/hidden-timing/hidden-traffic-shape claim",
        "PROOF_ARTIFACT_CLASS temporary /srv/qbuild/tmp no-secret text",
        *markers,
    ]


validate_base(fixture)
negative_count = run_negative_cases(fixture)

fixture_sha = hashlib.sha256(fixture_text.encode("utf-8")).hexdigest()
proof_path = artifact_dir / "na0365_restore_drill_isolated_restore_no_secret_proof.txt"
lines = proof_lines(fixture, fixture_sha, negative_count)
proof_text = "\n".join(lines) + "\n"
sentinels = fixture["no_secret_sentinels"]
require(count_sentinel_leaks(proof_text, sentinels) == 0, "sentinel leaked into proof artifact")
require(count_secret_patterns(proof_text) == 0, "secret-shaped pattern found in proof artifact")
proof_path.write_text(proof_text, encoding="utf-8")
proof_sha = hashlib.sha256(proof_path.read_bytes()).hexdigest()
proof_size = proof_path.stat().st_size

for marker in fixture["required_markers"]:
    print(marker)
print(f"NA0365_PROOF_ARTIFACT_PATH {proof_path}")
print(f"NA0365_PROOF_ARTIFACT_SIZE {proof_size}")
print(f"NA0365_PROOF_ARTIFACT_SHA256 {proof_sha}")
print("ISOLATED_RESTORE_SECRET_FINDING_COUNT 0")
print("NA0365_OPERATION_EXECUTED_COUNT 0")
print("NA0365_REAL_RESTORE_TARGET_CREATED_COUNT 0")
print("NA0365_MOUNT_ATTEMPT_COUNT 0")
print("NA0365_COPY_ATTEMPT_COUNT 0")
print("NA0365_REAL_BACKUP_COUNT 0")
print("NA0365_REAL_RESTORE_COUNT 0")
print("NA0365_KEY_OPERATION_COUNT 0")
print("NA0365_OFF_HOST_OPERATION_COUNT 0")
print(f"NA0365_NEGATIVE_CASES_PASSED {negative_count}")
PY
