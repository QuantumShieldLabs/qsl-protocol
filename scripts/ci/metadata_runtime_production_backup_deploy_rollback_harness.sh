#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT_DIR"

FIXTURE_FILE=${1:-inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json}
TMP_ROOT=${NA0352_HARNESS_TMP_ROOT:-/srv/qbuild/tmp}

case "$TMP_ROOT" in
  /srv/qbuild/tmp|/srv/qbuild/tmp/*) ;;
  *)
    echo "NA0352_HARNESS_TMP_ROOT must be under /srv/qbuild/tmp" >&2
    exit 1
    ;;
esac

if [ ! -f "$FIXTURE_FILE" ]; then
  echo "missing fixture file: $FIXTURE_FILE" >&2
  exit 1
fi

ARTIFACT_DIR=$(mktemp -d "$TMP_ROOT/NA-0352_production_backup_deploy_rollback.XXXXXX")

python3 - "$FIXTURE_FILE" "$ARTIFACT_DIR" <<'PY'
import json
import pathlib
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
    "local_continuity_status",
    "off_host_backup_status",
    "qsl_server_source_scope",
    "qsl_attachments_source_scope",
    "runtime_config_roots",
    "service_data_roots",
    "backup_scope",
    "restore_scope",
    "deploy_scope",
    "rollback_scope",
    "secrets_env_scope",
    "monitoring_logging_scope",
    "public_ingress_scope",
    "forbidden_operations",
    "future_authorized_operations",
    "claim_boundaries",
    "required_markers",
    "no_secret_sentinels",
}

REQUIRED_MARKERS = {
    "NA0352_PRODUCTION_BACKUP_DEPLOY_ROLLBACK_AUTHORIZATION_OK",
    "NA0352_SOURCE_BACKUP_SCOPE_OK",
    "NA0352_RUNTIME_CONFIG_BACKUP_SCOPE_OK",
    "NA0352_SERVICE_DATA_BACKUP_SCOPE_OK",
    "NA0352_LOCAL_CONTINUITY_BOUNDARY_OK",
    "NA0352_OFF_HOST_BACKUP_BOUNDARY_OK",
    "NA0352_RESTORE_DRILL_AUTHORIZATION_OK",
    "NA0352_DEPLOY_AUTHORIZATION_OK",
    "NA0352_ROLLBACK_AUTHORIZATION_OK",
    "NA0352_SECRETS_ENV_BOUNDARY_OK",
    "NA0352_MONITORING_LOGGING_BOUNDARY_OK",
    "NA0352_PUBLIC_INGRESS_BOUNDARY_OK",
    "NA0352_NO_PRODUCTION_READY_CLAIM_OK",
    "NA0352_NO_PUBLIC_INTERNET_READY_CLAIM_OK",
    "NA0352_NO_EXTERNAL_REVIEW_COMPLETE_CLAIM_OK",
    "NA0352_NO_METADATA_FREE_CLAIM_OK",
    "NA0352_NO_ANONYMITY_CLAIM_OK",
    "NA0352_NO_BACKUP_OPERATION_OK",
    "NA0352_NO_DEPLOY_OPERATION_OK",
    "NA0352_NO_ROLLBACK_OPERATION_OK",
    "NA0352_NO_RESTORE_OPERATION_OK",
    "NA0352_METADATA_RUNTIME_PRODUCTION_HARDENING_HARNESS_OK",
}

REQUIRED_FORBIDDEN = {
    "live_backup",
    "live_restore",
    "live_deploy",
    "live_rollback",
    "purge",
    "production_service_start_stop_restart",
    "secret_dependent_operation",
    "backup_script_timer_fstab_mutation",
    "local_backup_source_list_mutation",
    "public_ingress_cutover",
    "qsl_server_mutation",
    "qsl_attachments_mutation",
    "qshield_runtime_mutation",
    "protocol_crypto_qsc_qsp_mutation",
    "dependency_or_workflow_mutation",
    "website_public_docs_readiness_claim",
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
}

ALLOWED_CLASSES = {"AUTHORIZED", "FORBIDDEN", "FUTURE_GATE"}


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def require_mapping(name):
    value = fixture.get(name)
    require(isinstance(value, dict), f"{name} must be an object")
    return value


def require_no_operation(scope_name, live_keys=()):
    scope = require_mapping(scope_name)
    require(scope.get("operation_class") in ALLOWED_CLASSES, f"{scope_name} operation_class invalid")
    require(scope.get("current_authorization") != "AUTHORIZED", f"{scope_name} is currently authorized")
    require(scope.get("operation_performed_by_harness") is False, f"{scope_name} performed an operation")
    for key in live_keys:
        require(scope.get(key) is False, f"{scope_name} has live flag enabled: {key}")
    return scope


missing = sorted(REQUIRED_FIELDS - set(fixture))
require(not missing, f"missing required fields: {', '.join(missing)}")
require(
    fixture.get("schema_version") == "qsl.metadata_runtime.production_backup_deploy_rollback_fixture.v1",
    "schema_version mismatch",
)
require(fixture.get("artifact_class") == "boundary_harness_not_operation", "artifact_class mismatch")
require(
    fixture.get("selected_successor") == "NA-0353 -- Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan",
    "selected successor mismatch",
)

local_continuity = require_mapping("local_continuity_status")
require(local_continuity.get("classification") == "LOCAL_CONTINUITY_ONLY", "local continuity classification mismatch")
require(local_continuity.get("not_disaster_recovery") is True, "local continuity must not be disaster recovery")
require(local_continuity.get("operation_performed_by_harness") is False, "local continuity operation was performed")

off_host = require_mapping("off_host_backup_status")
require(off_host.get("classification") == "FUTURE_GATE", "off-host backup must be future-gated")
require(off_host.get("evidence_present") is False, "off-host backup cannot be complete in this fixture")
require(off_host.get("not_complete") is True, "off-host backup incomplete flag missing")
require(off_host.get("current_authorization") != "AUTHORIZED", "off-host backup is currently authorized")

for source_name in ("qsl_server_source_scope", "qsl_attachments_source_scope"):
    source = require_mapping(source_name)
    require(source.get("mutation_authorized") is False, f"{source_name} mutation must be forbidden")
    require(source.get("source_classification") == "FRESH_SOURCE", f"{source_name} source not fresh")
    require(source.get("authority_classification") == "COMPLETE_AUTHORITY", f"{source_name} authority incomplete")
    require(source.get("ci_classification") == "COMPLETE_CI", f"{source_name} CI incomplete")

require_no_operation("runtime_config_roots")
require_no_operation("service_data_roots")
require_no_operation("backup_scope", ("live_backup_authorized", "backup_script_timer_fstab_mutation_authorized"))
require_no_operation("restore_scope", ("live_restore_authorized",))
require_no_operation("deploy_scope", ("live_deploy_authorized",))
require_no_operation("rollback_scope", ("live_rollback_authorized",))
require_no_operation("secrets_env_scope", ("secret_values_allowed",))
require_no_operation("monitoring_logging_scope", ("live_monitoring_mutation_authorized",))
require_no_operation("public_ingress_scope", ("public_cutover_authorized",))

forbidden = set(fixture.get("forbidden_operations", []))
require(REQUIRED_FORBIDDEN <= forbidden, "forbidden operation set incomplete")

future_ops = fixture.get("future_authorized_operations")
require(isinstance(future_ops, list) and future_ops, "future_authorized_operations must be a non-empty list")
for item in future_ops:
    require(isinstance(item, dict), "future operation must be an object")
    classification = item.get("classification")
    require(classification in ALLOWED_CLASSES, f"invalid future operation class: {classification}")
    if item.get("live_operation") is True:
        require(classification == "FORBIDDEN", f"live operation is not forbidden: {item.get('name')}")
    if classification == "AUTHORIZED":
        require(item.get("mode") == "local_fixture_only", "authorized operation must be local fixture only")

claims = require_mapping("claim_boundaries")
require(CLAIM_KEYS <= set(claims), "claim boundary set incomplete")
for key in CLAIM_KEYS:
    claim = claims[key]
    require(isinstance(claim, dict), f"claim {key} must be an object")
    require(claim.get("status") in {"PROHIBITED", "NOT_READY", "FUTURE_GATE"}, f"claim {key} status invalid")
    require(claim.get("negated") is True, f"claim {key} is not negated")

markers = fixture.get("required_markers")
require(isinstance(markers, list), "required_markers must be a list")
require(set(markers) == REQUIRED_MARKERS, "required marker set mismatch")

sentinels = fixture.get("no_secret_sentinels")
require(isinstance(sentinels, list) and sentinels, "no_secret_sentinels must be a non-empty list")
require(len(set(sentinels)) == len(sentinels), "duplicate secret sentinel")
for sentinel in sentinels:
    require(isinstance(sentinel, str) and sentinel.startswith("NA0352_SECRET_SENTINEL_"), "unsafe sentinel label")


def strings_except_sentinels(value, path=()):
    if path == ("no_secret_sentinels",):
        return
    if isinstance(value, str):
        yield value
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from strings_except_sentinels(item, path + (str(index),))
    elif isinstance(value, dict):
        for key, item in value.items():
            yield from strings_except_sentinels(item, path + (str(key),))


other_fixture_strings = "\n".join(strings_except_sentinels(fixture))
secret_findings = 0
for sentinel in sentinels:
    if sentinel in other_fixture_strings:
        secret_findings += 1

lines = list(markers)
artifact_file = artifact_dir / "na0352_boundary_harness_proof.txt"
proof_lines = [
    "NA0352_ARTIFACT_CLASS boundary_harness_not_operation",
    "NA0352_OPERATION_EXECUTED_COUNT 0",
    "NA0352_BACKUP_PLAN_UPDATE_REQUIRED no",
    "NA0352_SELECTED_SUCCESSOR NA-0353 -- Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan",
    "NA0352_ARTIFACT_PATH " + str(artifact_file),
] + lines + [
    "SECRET_FINDING_COUNT 0",
]
proof = "\n".join(proof_lines) + "\n"

for sentinel in sentinels:
    if sentinel in proof:
        secret_findings += 1

require(secret_findings == 0, f"secret sentinel findings: {secret_findings}")

artifact_file.write_text(proof, encoding="utf-8")

print(proof, end="")
PY
