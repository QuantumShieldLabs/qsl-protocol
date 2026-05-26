#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT_DIR"

FIXTURE_FILE=${1:-inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json}
TMP_ROOT=${NA0363_HARNESS_TMP_ROOT:-/srv/qbuild/tmp}

case "$TMP_ROOT" in
  /srv/qbuild/tmp|/srv/qbuild/tmp/*) ;;
  *)
    echo "NA0363_HARNESS_TMP_ROOT must be under /srv/qbuild/tmp" >&2
    exit 1
    ;;
esac

if [ ! -f "$FIXTURE_FILE" ]; then
  echo "missing fixture file: $FIXTURE_FILE" >&2
  exit 1
fi

ARTIFACT_DIR=$(mktemp -d "$TMP_ROOT/NA-0363_off_host_backup_target_tool_no_secret.XXXXXX")

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
    "restore_classification",
    "target_mode",
    "tool_mode",
    "simulated_ssh_sftp_target_metadata",
    "simulated_target_identity_metadata",
    "simulated_restic_style_repository_metadata",
    "simulated_snapshot_metadata",
    "simulated_check_metadata",
    "simulated_prune_metadata",
    "simulated_restore_metadata",
    "simulated_snapshot_check_prune_restore_matrix",
    "simulated_retention_purge_matrix",
    "simulated_monitoring_alert_matrix",
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
    "selected_successor",
}

REQUIRED_MARKERS = {
    "NA0363_TARGET_TOOL_AUTHORIZATION_OK",
    "NA0363_NO_SECRET_TARGET_HARNESS_OK",
    "NA0363_NO_SECRET_TOOL_HARNESS_OK",
    "NA0363_SIMULATED_SSH_SFTP_TARGET_OK",
    "NA0363_SIMULATED_RESTIC_STYLE_REPOSITORY_OK",
    "NA0363_SIMULATED_SNAPSHOT_CHECK_PRUNE_RESTORE_MATRIX_OK",
    "NA0363_SIMULATED_RETENTION_PURGE_MATRIX_OK",
    "NA0363_SIMULATED_MONITORING_ALERT_MATRIX_OK",
    "NA0363_OPERATOR_RUNBOOK_MARKER_OK",
    "NA0363_BACKUP_PLAN_IMPACT_OK",
    "NA0363_NO_REMOTE_CONNECTION_OK",
    "NA0363_NO_REPOSITORY_INIT_OK",
    "NA0363_NO_TOOL_INSTALLATION_OK",
    "NA0363_NO_REAL_BACKUP_OK",
    "NA0363_NO_REAL_RESTORE_OK",
    "NA0363_NO_KEY_GENERATION_OK",
    "NA0363_NO_PASSPHRASE_COLLECTION_OK",
    "NA0363_NO_SECRET_MATERIAL_OK",
    "NA0363_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK",
    "NA0363_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK",
    "NA0363_NO_PRODUCTION_READY_CLAIM_OK",
    "NA0363_NO_PUBLIC_INTERNET_READY_CLAIM_OK",
    "NA0363_METADATA_RUNTIME_OFF_HOST_TARGET_TOOL_NO_SECRET_OK",
}

REQUIRED_FORBIDDEN = {
    "remote_connection",
    "repository_init",
    "tool_installation",
    "real_backup",
    "real_restore",
    "restore_target_creation",
    "restore_target_mount",
    "restore_copy",
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
    "missing_target_metadata",
    "missing_repository_metadata",
    "snapshot_check_mismatch",
    "missing_retention_purge_entry",
    "missing_monitoring_alert_entry",
    "prohibited_operation_field",
    "remote_connection_attempted",
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


def validate_fixture(data):
    missing = sorted(REQUIRED_FIELDS - set(data))
    require(not missing, f"missing required fields: {', '.join(missing)}")
    require(
        data.get("schema_version") == "qsl.metadata_runtime.off_host_backup_target_tool_no_secret_fixture.v1",
        "schema_version mismatch",
    )
    require(data.get("artifact_class") == "off_host_backup_target_tool_no_secret_v1", "artifact_class mismatch")
    require(set(data.get("goals", [])) == {"G1", "G2", "G3", "G4", "G5"}, "goals mismatch")
    require(data.get("target_mode") == "simulated only", "target mode mismatch")
    require(data.get("tool_mode") == "simulated only", "tool mode mismatch")
    require(
        data.get("selected_successor")
        == "NA-0364 -- Metadata Runtime Restore Drill Isolated Restore Authorization Plan",
        "selected successor mismatch",
    )

    authorization = require_mapping(data, "authorization")
    for key in (
        "remote_connection_authorized",
        "repository_init_authorized",
        "tool_installation_authorized",
        "real_backup_authorized",
        "real_restore_authorized",
        "restore_target_creation_authorized",
        "key_generation_authorized",
        "key_upload_authorized",
        "passphrase_collection_authorized",
        "private_key_inspection_authorized",
        "secret_material_handling_authorized",
        "recovery_envelope_content_authorized",
        "off_host_setup_authorized",
        "deploy_authorized",
        "rollback_authorized",
    ):
        require(authorization.get(key) is False, f"authorization flag is not false: {key}")

    sources = require_mapping(data, "source_classification")
    require(sources.get("qsl_protocol") == "FRESH_SOURCE", "qsl-protocol source is not fresh")
    require(sources.get("qsl_server") == "FRESH_SOURCE", "qsl-server source is not fresh")
    require(sources.get("qsl_attachments") == "FRESH_SOURCE", "qsl-attachments source is not fresh")
    require(sources.get("qshield_demo") == "REFERENCE_ORACLE_ONLY", "qshield demo boundary mismatch")

    local_backup = require_mapping(data, "local_backup_classification")
    require(local_backup.get("local_continuity") == "LOCAL_CONTINUITY_PROVEN", "local continuity classification mismatch")
    require(local_backup.get("backup_scope") == "SAME_HOST_CONTINUITY_ONLY", "backup scope mismatch")
    require(local_backup.get("local_backup_mutation_authorized") is False, "local backup mutation authorized")
    require(local_backup.get("off_host_backup") == "OFF_HOST_TARGET_NOT_READY", "off-host target classification mismatch")
    require(local_backup.get("off_host_tool") == "OFF_HOST_TOOL_NOT_READY", "off-host tool classification mismatch")

    key_class = require_mapping(data, "key_custody_classification")
    require(
        key_class.get("no_secret_key_custody_recovery") == "NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN",
        "no-secret key custody/recovery classification mismatch",
    )
    require(key_class.get("real_key_custody") == "REAL_KEY_CUSTODY_NOT_READY", "real key custody classification mismatch")
    require(key_class.get("real_key_recovery") == "REAL_KEY_RECOVERY_NOT_READY", "real key recovery classification mismatch")
    require(key_class.get("real_recovery_envelope_content") == "NOT_CREATED", "recovery envelope boundary mismatch")

    restore_class = require_mapping(data, "restore_classification")
    require(
        restore_class.get("no_secret_dry_run_restore") == "NO_SECRET_DRY_RUN_RESTORE_PROVEN",
        "no-secret restore classification mismatch",
    )
    require(restore_class.get("real_restore") == "REAL_RESTORE_NOT_AUTHORIZED", "real restore classification mismatch")
    require(restore_class.get("restore_target") == "RESTORE_TARGET_NOT_CREATED", "restore target classification mismatch")

    hashes = require_mapping(data, "integrity_hashes")
    require(hashes.get("algorithm") == "sha256", "integrity hash algorithm mismatch")
    hash_entries = require_list(hashes, "entries")
    hash_by_id = {}
    for entry in hash_entries:
        require(isinstance(entry, dict), "integrity hash entry must be an object")
        hash_by_id[entry.get("id")] = entry.get("sha256")

    targets = require_list(data, "simulated_ssh_sftp_target_metadata")
    target_ids = validate_hashed_items(targets, hash_by_id, "target_id")
    target_by_id = {item["target_id"]: item for item in targets}
    for target in targets:
        require(target.get("transport_class") == "simulated_ssh_sftp", "target transport class mismatch")
        require(target.get("endpoint_present") is False, "target endpoint present")
        require(target.get("real_remote_host_present") is False, "real remote host present")
        require(target.get("connection_attempted") is False, "remote connection attempted")
        require(target.get("remote_directory_created") is False, "remote directory created")
        require(target.get("marker") == "NA0363_SIMULATED_SSH_SFTP_TARGET_OK", "target marker mismatch")

    identities = require_list(data, "simulated_target_identity_metadata")
    identity_ids = validate_hashed_items(identities, hash_by_id, "identity_id")
    for identity in identities:
        require(identity.get("target_id") in target_by_id, "target identity references unknown target")
        require(identity.get("identity_class") == "metadata-label-only", "target identity class mismatch")
        require(identity.get("host_key_material_present") is False, "host key material present")
        require(identity.get("private_key_material_present") is False, "private key material present")

    repositories = require_list(data, "simulated_restic_style_repository_metadata")
    repository_ids = validate_hashed_items(repositories, hash_by_id, "repository_id")
    repository_by_id = {item["repository_id"]: item for item in repositories}
    for repository in repositories:
        require(repository.get("target_id") in target_by_id, "repository references unknown target")
        require(
            repository.get("tool_class") == "simulated_restic_style_snapshot_repository",
            "repository tool class mismatch",
        )
        require(repository.get("client_side_encryption_model") == "metadata-only", "repository encryption model mismatch")
        require(repository.get("repository_initialized") is False, "repository initialized")
        require(repository.get("repository_init_attempted") is False, "repository init attempted")
        require(repository.get("real_repository_present") is False, "real repository present")
        require(repository.get("tool_installed_by_harness") is False, "tool installed by harness")
        require(repository.get("marker") == "NA0363_SIMULATED_RESTIC_STYLE_REPOSITORY_OK", "repository marker mismatch")

    snapshots = require_list(data, "simulated_snapshot_metadata")
    snapshot_ids = validate_hashed_items(snapshots, hash_by_id, "snapshot_id")
    snapshot_by_id = {item["snapshot_id"]: item for item in snapshots}
    for snapshot in snapshots:
        require(snapshot.get("repository_id") in repository_by_id, "snapshot references unknown repository")
        require(snapshot.get("target_id") in target_by_id, "snapshot references unknown target")
        require(snapshot.get("snapshot_operation_executed") is False, "snapshot operation executed")
        require(snapshot.get("content_payload_present") is False, "snapshot content payload present")

    checks = require_list(data, "simulated_check_metadata")
    check_ids = validate_hashed_items(checks, hash_by_id, "check_id")
    check_by_id = {item["check_id"]: item for item in checks}
    for check in checks:
        require(check.get("repository_id") in repository_by_id, "check references unknown repository")
        require(check.get("snapshot_id") in snapshot_by_id, "check references unknown snapshot")
        require(check.get("simulated_result") == "SIMULATED_CHECK_OK", "check result mismatch")
        require(check.get("check_operation_executed") is False, "check operation executed")

    prunes = require_list(data, "simulated_prune_metadata")
    prune_ids = validate_hashed_items(prunes, hash_by_id, "prune_id")
    prune_by_id = {item["prune_id"]: item for item in prunes}
    for prune in prunes:
        require(prune.get("repository_id") in repository_by_id, "prune references unknown repository")
        require(prune.get("prune_operation_executed") is False, "prune operation executed")
        require(prune.get("purge_operation_executed") is False, "purge operation executed")

    restores = require_list(data, "simulated_restore_metadata")
    restore_ids = validate_hashed_items(restores, hash_by_id, "restore_id")
    restore_by_id = {item["restore_id"]: item for item in restores}
    for restore in restores:
        require(restore.get("repository_id") in repository_by_id, "restore references unknown repository")
        require(restore.get("snapshot_id") in snapshot_by_id, "restore references unknown snapshot")
        require(restore.get("restore_operation_executed") is False, "restore operation executed")
        require(restore.get("restore_target_created") is False, "restore target created")
        require(restore.get("restore_target_mounted") is False, "restore target mounted")
        require(restore.get("restore_payload_copied") is False, "restore payload copied")

    matrices = require_list(data, "simulated_snapshot_check_prune_restore_matrix")
    matrix_ids = validate_hashed_items(matrices, hash_by_id, "matrix_id")
    for matrix in matrices:
        repository = repository_by_id.get(matrix.get("repository_id"))
        snapshot = snapshot_by_id.get(matrix.get("snapshot_id"))
        check = check_by_id.get(matrix.get("check_id"))
        prune = prune_by_id.get(matrix.get("prune_id"))
        restore = restore_by_id.get(matrix.get("restore_id"))
        require(repository is not None, "matrix references unknown repository")
        require(snapshot is not None, "matrix references unknown snapshot")
        require(check is not None, "matrix references unknown check")
        require(prune is not None, "matrix references unknown prune")
        require(restore is not None, "matrix references unknown restore")
        require(snapshot["repository_id"] == repository["repository_id"], "snapshot repository mismatch")
        require(check["snapshot_id"] == snapshot["snapshot_id"], "check snapshot mismatch")
        require(check["repository_id"] == repository["repository_id"], "check repository mismatch")
        require(prune["repository_id"] == repository["repository_id"], "prune repository mismatch")
        require(restore["snapshot_id"] == snapshot["snapshot_id"], "restore snapshot mismatch")
        require(restore["repository_id"] == repository["repository_id"], "restore repository mismatch")
        require(matrix.get("operation_executed") is False, "matrix operation executed")
        require(
            matrix.get("marker") == "NA0363_SIMULATED_SNAPSHOT_CHECK_PRUNE_RESTORE_MATRIX_OK",
            "matrix marker mismatch",
        )

    retention_entries = require_list(data, "simulated_retention_purge_matrix")
    retention_ids = validate_hashed_items(retention_entries, hash_by_id, "retention_id")
    retention_by_id = {item["retention_id"]: item for item in retention_entries}
    for retention in retention_entries:
        require(retention.get("repository_id") in repository_by_id, "retention references unknown repository")
        require(retention.get("purge_authorized") is False, "purge authorized")
        require(retention.get("purge_operation_executed") is False, "purge operation executed")
        require(retention.get("marker") == "NA0363_SIMULATED_RETENTION_PURGE_MATRIX_OK", "retention marker mismatch")
    for prune in prunes:
        require(prune.get("retention_id") in retention_by_id, "prune references unknown retention")

    monitoring_entries = require_list(data, "simulated_monitoring_alert_matrix")
    monitor_ids = validate_hashed_items(monitoring_entries, hash_by_id, "monitor_id")
    for monitor in monitoring_entries:
        require(monitor.get("repository_id") in repository_by_id, "monitor references unknown repository")
        require(monitor.get("live_monitoring_mutation_authorized") is False, "monitoring mutation authorized")
        require(monitor.get("alert_channel_configured") is False, "alert channel configured")
        require(monitor.get("marker") == "NA0363_SIMULATED_MONITORING_ALERT_MATRIX_OK", "monitor marker mismatch")

    expected_hash_ids = (
        target_ids
        | identity_ids
        | repository_ids
        | snapshot_ids
        | check_ids
        | prune_ids
        | restore_ids
        | matrix_ids
        | retention_ids
        | monitor_ids
    )
    require(set(hash_by_id) == expected_hash_ids, "integrity hash ids do not match simulated records")

    runbook = require_mapping(data, "operator_runbook_markers")
    require(runbook.get("classification") == "NO_SECRET_OPERATOR_SUMMARY", "runbook classification mismatch")
    require(runbook.get("marker") == "NA0363_OPERATOR_RUNBOOK_MARKER_OK", "runbook marker mismatch")
    require(isinstance(runbook.get("emergency_stop"), list) and runbook["emergency_stop"], "emergency stop runbook missing")

    counters = require_mapping(data, "operation_counters")
    require(
        set(counters)
        >= {
            "remote_connection",
            "repository_init",
            "tool_installation",
            "real_backup",
            "real_restore",
            "restore_target_creation",
            "restore_target_mount",
            "restore_copy",
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
        },
        "operation counter set incomplete",
    )
    for key, value in counters.items():
        require(value == 0, f"operation counter is nonzero: {key}")

    sentinels = require_list(data, "no_secret_sentinels")
    require(len(set(sentinels)) == len(sentinels), "duplicate no-secret sentinels")
    for sentinel in sentinels:
        require(isinstance(sentinel, str) and sentinel.startswith("NA0363_SECRET_SENTINEL_"), "invalid sentinel label")

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
    require(impact.get("classification") == "NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW", "backup-plan classification mismatch")
    require(impact.get("update_required_now") is False, "backup-plan update required now")
    require(impact.get("marker") == "NA0363_BACKUP_PLAN_IMPACT_OK", "backup-plan marker mismatch")

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
        if name == "missing_target_metadata":
            mutated.pop("simulated_ssh_sftp_target_metadata", None)
        elif name == "missing_repository_metadata":
            mutated.pop("simulated_restic_style_repository_metadata", None)
        elif name == "snapshot_check_mismatch":
            mutated["simulated_check_metadata"][0]["snapshot_id"] = "sim-snapshot-missing"
        elif name == "missing_retention_purge_entry":
            mutated.pop("simulated_retention_purge_matrix", None)
        elif name == "missing_monitoring_alert_entry":
            mutated.pop("simulated_monitoring_alert_matrix", None)
        elif name == "prohibited_operation_field":
            mutated["operation_counters"]["real_backup"] = 1
        elif name == "remote_connection_attempted":
            mutated["simulated_ssh_sftp_target_metadata"][0]["connection_attempted"] = True
        elif name == "missing_claim_boundary":
            mutated["claim_boundaries"].pop("production_readiness", None)
        elif name == "missing_no_secret_marker":
            mutated["required_markers"].remove("NA0363_NO_SECRET_MATERIAL_OK")
        else:
            raise AssertionError(f"unknown negative case: {name}")
        expect_failure(name, mutated)
        passed.append(name)
    require(set(passed) == NEGATIVE_CASES, "not all negative cases passed")
    return passed


validate_fixture(fixture)
negative_passed = run_negative_cases(fixture)

artifact_file = artifact_dir / "na0363_off_host_backup_target_tool_no_secret_proof.txt"
fixture_sha = hashlib.sha256(fixture_text.encode("utf-8")).hexdigest()
proof_lines = [
    "NA0363_ARTIFACT_CLASS off_host_backup_target_tool_no_secret_v1",
    "NA0363_ARTIFACT_PATH " + str(artifact_file),
    "NA0363_ARTIFACT_DIR " + str(artifact_dir),
    "NA0363_FIXTURE_SHA256 " + fixture_sha,
    "NA0363_OPERATION_EXECUTED_COUNT 0",
    "NA0363_REMOTE_CONNECTION_COUNT 0",
    "NA0363_REPOSITORY_INIT_COUNT 0",
    "NA0363_TOOL_INSTALLATION_COUNT 0",
    "NA0363_REAL_BACKUP_COUNT 0",
    "NA0363_REAL_RESTORE_COUNT 0",
    "NA0363_RESTORE_TARGET_CREATED_COUNT 0",
    "NA0363_RESTORE_TARGET_MOUNTED_COUNT 0",
    "NA0363_RESTORE_COPY_COUNT 0",
    "NA0363_KEY_GENERATION_COUNT 0",
    "NA0363_KEY_UPLOAD_COUNT 0",
    "NA0363_PASSPHRASE_COLLECTION_COUNT 0",
    "NA0363_PRIVATE_KEY_INSPECTION_COUNT 0",
    "NA0363_SECRET_MATERIAL_HANDLING_COUNT 0",
    "NA0363_RECOVERY_ENVELOPE_CONTENT_CREATION_COUNT 0",
    "NA0363_OFF_HOST_SETUP_COUNT 0",
    "NA0363_DEPLOY_ROLLBACK_OPERATION_COUNT 0",
    "NA0363_LOCAL_BACKUP_MUTATION_COUNT 0",
    "NA0363_BACKUP_PLAN_UPDATE_REQUIRED no",
    "NA0363_SELECTED_SUCCESSOR " + fixture["selected_successor"],
    "NA0363_NEGATIVE_CASES_PASSED " + str(len(negative_passed)),
    "NA0363_NEGATIVE_CASE_NAMES " + ",".join(sorted(negative_passed)),
    "OFF_HOST_TARGET_TOOL_SECRET_FINDING_COUNT 0",
    "NA0363_SENTINEL_LEAK_FINDING_COUNT 0",
]
proof_lines.extend(fixture["required_markers"])
proof = "\n".join(proof_lines) + "\n"

require(count_sentinel_leaks(proof, fixture["no_secret_sentinels"]) == 0, "sentinel leaked into proof")
require(count_secret_patterns(proof) == 0, "secret-like pattern found in proof")

artifact_file.write_text(proof, encoding="utf-8")

print(proof, end="")
PY
