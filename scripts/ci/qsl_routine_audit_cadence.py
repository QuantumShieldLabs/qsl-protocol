#!/usr/bin/env python3
"""Validate the QSL routine audit cadence policy and temp-output harness.

This helper is intentionally local-only and standard-library-only. It validates
policy fixtures and simulated trigger inputs, writes only new proof files under
/srv/qbuild/tmp/NA0390_routine_audit_cadence_*, and never performs network,
GitHub, subprocess, scheduler, workflow, or durable audit report actions.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any


SCHEMA_VERSION = "qsl.routine_audit_cadence.v1"
PROFILE_SCHEMA_VERSION = "qsl.routine_audit_cadence.profile.v1"
TRIGGER_SCHEMA_VERSION = "qsl.routine_audit_cadence.trigger.v1"
SUMMARY_SCHEMA_VERSION = "qsl.routine_audit_cadence.summary.v1"
EVENTS_SCHEMA_VERSION = "qsl.routine_audit_cadence.events.v1"
FIXTURE_SCHEMA_VERSION = "qsl.routine_audit_cadence.fixture_matrix.v1"
TEMP_ROOT = Path("/srv/qbuild/tmp")
TEMP_PREFIX = "NA0390_routine_audit_cadence_"
EXIT_OK = 0
EXIT_INTERNAL = 1
EXIT_VALIDATION = 2

REQUIRED_PROFILE_IDS = {
    "overall_project",
    "code_crypto",
    "local_ops_history_backup",
    "public_claim_external_review",
    "targeted_incident_regression",
    "external_standards_threat_technology_watch",
}

REQUIRED_DEPTH_IDS = {
    "quick_read_only",
    "standard_read_only",
    "deep_read_only_microscope",
    "targeted_code_crypto_microscope",
}

REQUIRED_TRIGGER_CLASSES = {
    "pr_count_threshold",
    "na_count_threshold",
    "runtime_protocol_crypto_change",
    "qsl_server_qsl_attachments_change",
    "dependency_advisory_event",
    "backup_restore_deploy_rollback_change",
    "public_paper_precondition",
    "website_public_doc_claim_change",
    "external_review_package_work",
    "production_public_internet_claim_proposal",
    "public_safety_workflow_change",
    "critical_high_finding",
    "operator_demand",
    "external_standards_threat_technology_watch_request",
}

REQUIRED_SEVERITIES = {
    "CRITICAL",
    "HIGH",
    "MEDIUM",
    "LOW",
    "INFO",
    "EVIDENCE_INCOMPLETE",
    "CLAIM_BOUNDARY",
    "BACKLOG_CANDIDATE",
}

REQUIRED_FORBIDDEN_ACTIONS = {
    "background_automation",
    "branch_mutation",
    "cron",
    "delete",
    "dependency_change",
    "durable_report_write",
    "external_watch_execution",
    "full_audit_execution",
    "github_call",
    "network",
    "overwrite",
    "public_claim_expansion",
    "response_archive_mutation",
    "runtime_change",
    "scheduler",
    "secret_material",
    "shell_subprocess",
    "workflow_change",
}

REQUIRED_FORBIDDEN_CLAIMS = {
    "bug_free",
    "perfect_crypto",
    "production_ready",
    "public_internet_ready",
    "metadata_free",
    "anonymity",
    "untraceable",
    "external_review_complete",
}

FORBIDDEN_CLAIM_PATTERNS = {
    "bug_free": re.compile(r"(?i)\bbug[- ]free\b"),
    "perfect_crypto": re.compile(r"(?i)\b(?:perfect crypto|cryptographically perfect)\b"),
    "production_ready": re.compile(r"(?i)\b(?:production[- ]ready|ready for production)\b"),
    "public_internet_ready": re.compile(r"(?i)\bpublic[- ]internet[- ]ready\b"),
    "metadata_free": re.compile(r"(?i)\bmetadata[- ]free\b"),
    "anonymity": re.compile(r"(?i)\banonymity\b|\banonymous messaging\b"),
    "untraceable": re.compile(r"(?i)\buntraceable\b"),
    "external_review_complete": re.compile(r"(?i)\bexternal review complete\b|\bexternally reviewed\b"),
}

SECRET_SENTINEL_PATTERNS = [
    re.compile(r"\bNA0390_SECRET_SENTINEL\b"),
    re.compile(r"\bQSL_TEST_FORBIDDEN_MARKER_VALUE\b"),
    re.compile(r"-----BEGIN (?:[A-Z0-9]+ )?PRIVATE KEY-----"),
    re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{30,}\b"),
    re.compile(r"\bgithub_pat_[A-Za-z0-9_]{30,}\b"),
    re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b"),
    re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b"),
]

REQUIRED_MARKERS = [
    "NA0390_ROUTINE_AUDIT_CADENCE_AUTHORIZATION_OK",
    "NA0390_ROUTINE_AUDIT_CADENCE_HELPER_OK",
    "NA0390_OVERALL_PROJECT_AUDIT_PROFILE_OK",
    "NA0390_CODE_CRYPTO_AUDIT_PROFILE_OK",
    "NA0390_LOCAL_OPS_HISTORY_BACKUP_AUDIT_PROFILE_OK",
    "NA0390_PUBLIC_CLAIM_REVIEW_AUDIT_PROFILE_OK",
    "NA0390_TARGETED_INCIDENT_AUDIT_PROFILE_OK",
    "NA0390_EXTERNAL_STANDARDS_TECH_WATCH_FUTURE_GATED_OK",
    "NA0390_AUDIT_TRIGGER_POLICY_OK",
    "NA0390_AUDIT_DEPTH_LEVELS_OK",
    "NA0390_AUDIT_SEVERITY_TAXONOMY_OK",
    "NA0390_AUDIT_QUEUE_INSERTION_POLICY_OK",
    "NA0390_TEMP_OUTPUT_BOUNDARY_OK",
    "NA0390_NO_DURABLE_REPORT_WRITE_OK",
    "NA0390_NO_BACKGROUND_SCHEDULER_OK",
    "NA0390_NO_WORKFLOW_CHANGE_OK",
    "NA0390_NO_DEPENDENCY_CHANGE_OK",
    "NA0390_NO_RUNTIME_CHANGE_OK",
    "NA0390_NO_SECRET_MATERIAL_OK",
    "NA0390_NO_BUG_FREE_CLAIM_OK",
    "NA0390_NO_CRYPTO_PERFECT_CLAIM_OK",
    "NA0390_NO_METADATA_FREE_CLAIM_OK",
    "NA0390_NO_ANONYMITY_CLAIM_OK",
    "NA0390_NO_UNTRACEABLE_CLAIM_OK",
    "NA0390_NO_PRODUCTION_READY_CLAIM_OK",
    "NA0390_NO_PUBLIC_INTERNET_READY_CLAIM_OK",
    "NA0390_METADATA_RUNTIME_ROUTINE_AUDIT_CADENCE_OK",
]

POLICY_KEYS = {
    "schema_version",
    "policy_id",
    "title",
    "goals",
    "audit_profiles",
    "depth_levels",
    "trigger_rules",
    "severity_taxonomy",
    "queue_insertion_policy",
    "report_output_boundaries",
    "public_claim_boundaries",
    "automation_boundaries",
    "finding_examples",
    "queue_candidate_examples",
    "proposed_report_claims",
    "simulated_report_text",
    "public_technical_paper",
}

PROFILE_KEYS = {
    "id",
    "title",
    "audit_type",
    "status",
    "future_gated",
    "allowed_depth_ids",
    "trigger_classes",
    "allowed_sources",
    "forbidden_actions",
    "output_boundary",
    "public_claim_boundary",
    "queue_policy",
    "markers",
    "external_watch",
}

TRIGGER_KEYS = {
    "id",
    "class",
    "audit_profile_id",
    "depth_id",
    "threshold",
    "recommended_candidate_status",
    "future_gated_only",
}


class CadenceError(RuntimeError):
    """Expected fail-closed validation error."""


def ensure_object(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise CadenceError(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise CadenceError(f"{label} must be a list")
    return value


def ensure_string(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise CadenceError(f"{label} must be a non-empty string")
    return value


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise CadenceError(f"{label} must be a boolean")
    return value


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int) or isinstance(value, bool):
        raise CadenceError(f"{label} must be an integer")
    return value


def reject_unknown_keys(value: dict[str, Any], allowed: set[str], label: str) -> None:
    unknown = sorted(set(value) - allowed)
    if unknown:
        raise CadenceError(f"{label} has unknown key(s): {', '.join(unknown)}")


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise CadenceError(f"malformed JSON in {path}: line {exc.lineno} column {exc.colno}") from exc
    except OSError as exc:
        raise CadenceError(f"cannot read {path}: {exc}") from exc


def is_relative_to(path: Path, parent: Path) -> bool:
    try:
        path.relative_to(parent)
    except ValueError:
        return False
    return True


def reject_ambiguous_path(raw: str) -> None:
    if "\x00" in raw:
        raise CadenceError("path contains NUL byte")
    if "\\" in raw:
        raise CadenceError(f"path uses backslash separators: {raw}")
    if raw != raw.strip():
        raise CadenceError(f"path has leading/trailing whitespace: {raw!r}")
    if raw.startswith("~"):
        raise CadenceError(f"home-relative path rejected: {raw}")
    parts = Path(raw).parts
    if any(part in {"", ".."} for part in parts):
        raise CadenceError(f"parent traversal or empty path segment rejected: {raw}")


def validate_tmp_dir(raw: str) -> Path:
    reject_ambiguous_path(raw)
    out_dir = Path(raw)
    resolved = out_dir.resolve(strict=False)
    temp_root = TEMP_ROOT.resolve(strict=True)
    if resolved == temp_root or not is_relative_to(resolved, temp_root):
        raise CadenceError("tmp-dir must be under /srv/qbuild/tmp")
    relative_parts = resolved.relative_to(temp_root).parts
    if not relative_parts or not relative_parts[0].startswith(TEMP_PREFIX):
        raise CadenceError("tmp-dir must be under /srv/qbuild/tmp/NA0390_routine_audit_cadence_*")
    resolved.mkdir(parents=True, exist_ok=True)
    if resolved.is_symlink():
        raise CadenceError("tmp-dir symlink rejected")
    return resolved


def write_text_new(path: Path, text: str) -> None:
    if path.exists():
        raise CadenceError(f"refusing to overwrite existing output: {path}")
    with path.open("x", encoding="utf-8") as handle:
        handle.write(text)


def write_json_new(path: Path, value: Any) -> None:
    write_text_new(path, json.dumps(value, indent=2, sort_keys=True) + "\n")


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def ensure_string_set(values: Any, label: str) -> set[str]:
    items = ensure_list(values, label)
    result: set[str] = set()
    for index, item in enumerate(items):
        result.add(ensure_string(item, f"{label}[{index}]"))
    if len(result) != len(items):
        raise CadenceError(f"{label} must not contain duplicates")
    return result


def validate_no_secret_text(text: str, label: str) -> None:
    for pattern in SECRET_SENTINEL_PATTERNS:
        if pattern.search(text):
            raise CadenceError(f"secret sentinel rejected in {label}")


def validate_no_forbidden_claims(claims: list[str], label: str) -> None:
    for claim in claims:
        claim_text = ensure_string(claim, f"{label}[]")
        for claim_id, pattern in FORBIDDEN_CLAIM_PATTERNS.items():
            if pattern.search(claim_text) or claim_text == claim_id:
                raise CadenceError(f"forbidden public/readiness/privacy claim rejected: {claim_id}")


def validate_profile(profile: dict[str, Any], *, standalone: bool = False) -> dict[str, Any]:
    allowed = set(PROFILE_KEYS)
    if standalone:
        allowed.add("schema_version")
    reject_unknown_keys(profile, allowed, "audit_profile")
    if standalone and profile.get("schema_version") != PROFILE_SCHEMA_VERSION:
        raise CadenceError("profile schema_version mismatch")

    profile_id = ensure_string(profile.get("id"), "profile.id")
    if profile_id not in REQUIRED_PROFILE_IDS:
        raise CadenceError(f"unknown audit profile id: {profile_id}")
    ensure_string(profile.get("title"), "profile.title")
    audit_type = ensure_string(profile.get("audit_type"), "profile.audit_type")
    if audit_type != profile_id:
        raise CadenceError("profile.audit_type must match profile.id")
    status = ensure_string(profile.get("status"), "profile.status")
    future_gated = ensure_bool(profile.get("future_gated"), "profile.future_gated")
    if profile_id == "external_standards_threat_technology_watch":
        if status != "future_gated" or not future_gated:
            raise CadenceError("external standards/threat/technology watch must be future-gated")
    elif status != "active" or future_gated:
        raise CadenceError(f"profile {profile_id} must be active and not future-gated")

    depth_ids = ensure_string_set(profile.get("allowed_depth_ids"), "profile.allowed_depth_ids")
    if not depth_ids or not depth_ids <= REQUIRED_DEPTH_IDS:
        raise CadenceError("profile.allowed_depth_ids contains unknown or empty depth ids")
    trigger_classes = ensure_string_set(profile.get("trigger_classes"), "profile.trigger_classes")
    if not trigger_classes or not trigger_classes <= REQUIRED_TRIGGER_CLASSES:
        raise CadenceError("profile.trigger_classes contains unknown or empty trigger classes")
    ensure_string_set(profile.get("allowed_sources"), "profile.allowed_sources")
    forbidden_actions = ensure_string_set(profile.get("forbidden_actions"), "profile.forbidden_actions")
    missing_forbidden = REQUIRED_FORBIDDEN_ACTIONS - forbidden_actions
    if missing_forbidden:
        raise CadenceError(f"profile missing forbidden action(s): {', '.join(sorted(missing_forbidden))}")
    if ensure_string(profile.get("output_boundary"), "profile.output_boundary") != "temp_output_only":
        raise CadenceError("profile output boundary must be temp_output_only")
    if ensure_string(profile.get("public_claim_boundary"), "profile.public_claim_boundary") != "no_expansion":
        raise CadenceError("profile public claim boundary must be no_expansion")
    if ensure_string(profile.get("queue_policy"), "profile.queue_policy") != "candidate_only_no_ready_mutation":
        raise CadenceError("profile queue policy must be candidate_only_no_ready_mutation")
    markers = ensure_string_set(profile.get("markers"), "profile.markers")
    if profile_id == "external_standards_threat_technology_watch":
        external_watch = ensure_object(profile.get("external_watch"), "profile.external_watch")
        reject_unknown_keys(
            external_watch,
            {"perform_watch", "web_browsing_allowed", "source_cited_future_lane_required"},
            "profile.external_watch",
        )
        if ensure_bool(external_watch.get("perform_watch"), "external_watch.perform_watch"):
            raise CadenceError("external standards watch execution is forbidden in NA-0390")
        if ensure_bool(external_watch.get("web_browsing_allowed"), "external_watch.web_browsing_allowed"):
            raise CadenceError("web browsing is forbidden in NA-0390")
        if not ensure_bool(
            external_watch.get("source_cited_future_lane_required"),
            "external_watch.source_cited_future_lane_required",
        ):
            raise CadenceError("external watch must require a future source-cited lane")
    elif "external_watch" in profile and profile["external_watch"] not in ({}, None):
        raise CadenceError("external_watch metadata is only valid on the future-gated watch profile")
    return {"id": profile_id, "markers": sorted(markers)}


def validate_depth_levels(depths: Any) -> list[dict[str, Any]]:
    items = ensure_list(depths, "depth_levels")
    seen: set[str] = set()
    for depth in items:
        obj = ensure_object(depth, "depth_levels[]")
        reject_unknown_keys(obj, {"id", "title", "purpose", "boundary"}, "depth_level")
        depth_id = ensure_string(obj.get("id"), "depth.id")
        if depth_id not in REQUIRED_DEPTH_IDS:
            raise CadenceError(f"unknown depth id: {depth_id}")
        seen.add(depth_id)
        ensure_string(obj.get("title"), "depth.title")
        ensure_string(obj.get("purpose"), "depth.purpose")
        boundary = ensure_string(obj.get("boundary"), "depth.boundary")
        if "durable" in boundary.lower() and "no durable" not in boundary.lower():
            raise CadenceError("depth boundary must not allow durable reports")
    missing = REQUIRED_DEPTH_IDS - seen
    if missing:
        raise CadenceError(f"missing depth level(s): {', '.join(sorted(missing))}")
    return items


def validate_trigger(trigger: dict[str, Any], *, profile_ids: set[str] | None = None, depth_ids: set[str] | None = None, standalone: bool = False) -> dict[str, Any]:
    allowed = set(TRIGGER_KEYS)
    if standalone:
        allowed.add("schema_version")
    reject_unknown_keys(trigger, allowed, "trigger")
    if standalone and trigger.get("schema_version") != TRIGGER_SCHEMA_VERSION:
        raise CadenceError("trigger schema_version mismatch")
    trigger_id = ensure_string(trigger.get("id"), "trigger.id")
    trigger_class = ensure_string(trigger.get("class"), "trigger.class")
    if trigger_class not in REQUIRED_TRIGGER_CLASSES:
        raise CadenceError(f"unknown trigger class: {trigger_class}")
    profile_id = ensure_string(trigger.get("audit_profile_id"), "trigger.audit_profile_id")
    if profile_ids is not None and profile_id not in profile_ids:
        raise CadenceError(f"trigger references unknown audit profile: {profile_id}")
    depth_id = ensure_string(trigger.get("depth_id"), "trigger.depth_id")
    if depth_ids is not None and depth_id not in depth_ids:
        raise CadenceError(f"trigger references unknown depth: {depth_id}")
    threshold = trigger.get("threshold")
    if trigger_class in {"pr_count_threshold", "na_count_threshold"}:
        if ensure_int(threshold, "trigger.threshold") <= 0:
            raise CadenceError("count trigger threshold must be positive")
    elif threshold is not None:
        raise CadenceError("non-count triggers must use null threshold")
    if ensure_string(trigger.get("recommended_candidate_status"), "trigger.recommended_candidate_status") != "BACKLOG_CANDIDATE":
        raise CadenceError("trigger may recommend BACKLOG_CANDIDATE only")
    future_gated_only = ensure_bool(trigger.get("future_gated_only"), "trigger.future_gated_only")
    if profile_id == "external_standards_threat_technology_watch" and not future_gated_only:
        raise CadenceError("external standards watch trigger must be future-gated only")
    return {"id": trigger_id, "class": trigger_class, "profile_id": profile_id, "depth_id": depth_id}


def validate_triggers(triggers: Any, profile_ids: set[str], depth_ids: set[str]) -> list[dict[str, Any]]:
    items = ensure_list(triggers, "trigger_rules")
    seen_classes: set[str] = set()
    normalized: list[dict[str, Any]] = []
    for trigger in items:
        obj = ensure_object(trigger, "trigger_rules[]")
        item = validate_trigger(obj, profile_ids=profile_ids, depth_ids=depth_ids)
        seen_classes.add(item["class"])
        normalized.append(item)
    missing = REQUIRED_TRIGGER_CLASSES - seen_classes
    if missing:
        raise CadenceError(f"missing trigger class(es): {', '.join(sorted(missing))}")
    return normalized


def validate_severity_taxonomy(value: Any) -> list[dict[str, Any]]:
    items = ensure_list(value, "severity_taxonomy")
    seen: set[str] = set()
    for severity in items:
        obj = ensure_object(severity, "severity_taxonomy[]")
        reject_unknown_keys(obj, {"id", "description", "stop_or_escalate", "queue_action"}, "severity")
        sev_id = ensure_string(obj.get("id"), "severity.id")
        if sev_id not in REQUIRED_SEVERITIES:
            raise CadenceError(f"unknown severity: {sev_id}")
        seen.add(sev_id)
        ensure_string(obj.get("description"), "severity.description")
        stop = ensure_bool(obj.get("stop_or_escalate"), "severity.stop_or_escalate")
        if sev_id in {"CRITICAL", "HIGH"} and not stop:
            raise CadenceError("CRITICAL/HIGH severity requires stop/escalation policy")
        if ensure_string(obj.get("queue_action"), "severity.queue_action") not in {
            "STOP_AND_TRIAGE",
            "PROPOSE_BACKLOG",
            "RECORD_ONLY",
            "CLAIM_BOUNDARY_REVIEW",
        }:
            raise CadenceError("invalid severity queue_action")
    missing = REQUIRED_SEVERITIES - seen
    if missing:
        raise CadenceError(f"missing severity id(s): {', '.join(sorted(missing))}")
    return items


def validate_queue_policy(value: Any) -> dict[str, Any]:
    obj = ensure_object(value, "queue_insertion_policy")
    reject_unknown_keys(
        obj,
        {
            "auto_promote_ready",
            "max_ready_candidates",
            "allowed_candidate_statuses",
            "one_ready_required",
            "candidate_output_only",
            "critical_high_requires_stop_or_escalation",
        },
        "queue_insertion_policy",
    )
    if ensure_bool(obj.get("auto_promote_ready"), "queue.auto_promote_ready"):
        raise CadenceError("audit findings must not auto-promote READY items")
    if ensure_int(obj.get("max_ready_candidates"), "queue.max_ready_candidates") != 0:
        raise CadenceError("queue policy must allow zero READY candidates from helper output")
    statuses = ensure_string_set(obj.get("allowed_candidate_statuses"), "queue.allowed_candidate_statuses")
    if statuses != {"BACKLOG_CANDIDATE"}:
        raise CadenceError("queue policy may output BACKLOG_CANDIDATE only")
    if not ensure_bool(obj.get("one_ready_required"), "queue.one_ready_required"):
        raise CadenceError("queue policy must preserve one READY discipline")
    if not ensure_bool(obj.get("candidate_output_only"), "queue.candidate_output_only"):
        raise CadenceError("queue policy must be candidate-output-only")
    if not ensure_bool(
        obj.get("critical_high_requires_stop_or_escalation"),
        "queue.critical_high_requires_stop_or_escalation",
    ):
        raise CadenceError("CRITICAL/HIGH findings require stop/escalation policy")
    return obj


def validate_report_boundary(value: Any) -> dict[str, Any]:
    obj = ensure_object(value, "report_output_boundaries")
    reject_unknown_keys(
        obj,
        {
            "tmp_root",
            "tmp_prefix",
            "durable_reports_allowed",
            "scheduler_allowed",
            "cron_allowed",
            "workflow_allowed",
            "background_automation_allowed",
            "no_overwrite",
            "no_delete",
            "no_full_response_body_copy",
        },
        "report_output_boundaries",
    )
    if ensure_string(obj.get("tmp_root"), "report.tmp_root") != str(TEMP_ROOT):
        raise CadenceError("report tmp_root must be /srv/qbuild/tmp")
    if ensure_string(obj.get("tmp_prefix"), "report.tmp_prefix") != TEMP_PREFIX:
        raise CadenceError("report tmp_prefix mismatch")
    false_fields = [
        "durable_reports_allowed",
        "scheduler_allowed",
        "cron_allowed",
        "workflow_allowed",
        "background_automation_allowed",
    ]
    for field in false_fields:
        if ensure_bool(obj.get(field), f"report.{field}"):
            raise CadenceError(f"{field} must be false")
    true_fields = ["no_overwrite", "no_delete", "no_full_response_body_copy"]
    for field in true_fields:
        if not ensure_bool(obj.get(field), f"report.{field}"):
            raise CadenceError(f"{field} must be true")
    return obj


def validate_public_claim_boundaries(value: Any) -> dict[str, Any]:
    obj = ensure_object(value, "public_claim_boundaries")
    reject_unknown_keys(
        obj,
        {
            "forbidden_claims",
            "external_review_complete_allowed",
            "public_technical_paper_requires_preconditions",
            "claim_expansion_allowed",
        },
        "public_claim_boundaries",
    )
    forbidden = ensure_string_set(obj.get("forbidden_claims"), "public_claim_boundaries.forbidden_claims")
    missing = REQUIRED_FORBIDDEN_CLAIMS - forbidden
    if missing:
        raise CadenceError(f"missing forbidden claim boundary: {', '.join(sorted(missing))}")
    if ensure_bool(obj.get("external_review_complete_allowed"), "public_claim_boundaries.external_review_complete_allowed"):
        raise CadenceError("external-review-complete claim is forbidden")
    if ensure_bool(obj.get("claim_expansion_allowed"), "public_claim_boundaries.claim_expansion_allowed"):
        raise CadenceError("public claim expansion is forbidden")
    preconditions = ensure_string_set(
        obj.get("public_technical_paper_requires_preconditions"),
        "public_claim_boundaries.public_technical_paper_requires_preconditions",
    )
    required = {"public_claim_boundary_audit", "external_review_readiness", "code_crypto_audit_status", "service_backup_restore_status", "evidence_mapping"}
    if not required <= preconditions:
        raise CadenceError("public technical paper preconditions are incomplete")
    return obj


def validate_automation_boundaries(value: Any) -> dict[str, Any]:
    obj = ensure_object(value, "automation_boundaries")
    reject_unknown_keys(obj, {"scheduler", "cron", "github_workflow", "background_job", "network", "github_calls"}, "automation_boundaries")
    for field in sorted(obj):
        if ensure_bool(obj.get(field), f"automation.{field}"):
            raise CadenceError(f"automation boundary must keep {field} false")
    return obj


def validate_public_technical_paper(value: Any) -> dict[str, Any]:
    obj = ensure_object(value, "public_technical_paper")
    reject_unknown_keys(obj, {"allowed_without_preconditions", "preconditions_required"}, "public_technical_paper")
    if ensure_bool(obj.get("allowed_without_preconditions"), "public_technical_paper.allowed_without_preconditions"):
        raise CadenceError("public technical paper work cannot be allowed without preconditions")
    if not ensure_bool(obj.get("preconditions_required"), "public_technical_paper.preconditions_required"):
        raise CadenceError("public technical paper preconditions must be required")
    return obj


def validate_finding_examples(findings: Any, severity_ids: set[str]) -> None:
    for index, finding in enumerate(ensure_list(findings, "finding_examples")):
        obj = ensure_object(finding, f"finding_examples[{index}]")
        reject_unknown_keys(obj, {"id", "severity", "stop_or_escalate"}, "finding")
        ensure_string(obj.get("id"), "finding.id")
        severity = ensure_string(obj.get("severity"), "finding.severity")
        if severity not in severity_ids:
            raise CadenceError(f"finding references unknown severity: {severity}")
        stop = ensure_bool(obj.get("stop_or_escalate"), "finding.stop_or_escalate")
        if severity in {"CRITICAL", "HIGH"} and not stop:
            raise CadenceError("CRITICAL/HIGH finding requires stop/escalation")


def validate_queue_candidate_examples(candidates: Any) -> None:
    ready_count = 0
    for index, candidate in enumerate(ensure_list(candidates, "queue_candidate_examples")):
        obj = ensure_object(candidate, f"queue_candidate_examples[{index}]")
        reject_unknown_keys(obj, {"id", "status", "ready_mutation"}, "queue_candidate")
        ensure_string(obj.get("id"), "queue_candidate.id")
        status = ensure_string(obj.get("status"), "queue_candidate.status")
        ready_mutation = ensure_bool(obj.get("ready_mutation"), "queue_candidate.ready_mutation")
        if status == "READY" or ready_mutation:
            ready_count += 1
    if ready_count:
        raise CadenceError("audit findings must not create READY candidates or READY mutations")


def validate_policy(policy: dict[str, Any]) -> dict[str, Any]:
    reject_unknown_keys(policy, POLICY_KEYS, "policy")
    if policy.get("schema_version") != SCHEMA_VERSION:
        raise CadenceError("policy schema_version mismatch")
    ensure_string(policy.get("policy_id"), "policy.policy_id")
    ensure_string(policy.get("title"), "policy.title")
    goals = ensure_string_set(policy.get("goals"), "policy.goals")
    if not {"G1", "G2", "G3", "G4", "G5"} <= goals:
        raise CadenceError("policy must map to G1, G2, G3, G4, G5")

    profile_summaries = []
    profile_ids: set[str] = set()
    for profile in ensure_list(policy.get("audit_profiles"), "audit_profiles"):
        profile_summary = validate_profile(ensure_object(profile, "audit_profiles[]"))
        profile_summaries.append(profile_summary)
        profile_ids.add(profile_summary["id"])
    missing_profiles = REQUIRED_PROFILE_IDS - profile_ids
    if missing_profiles:
        raise CadenceError(f"missing audit profile(s): {', '.join(sorted(missing_profiles))}")

    depths = validate_depth_levels(policy.get("depth_levels"))
    depth_ids = {ensure_string(item["id"], "depth.id") for item in depths}
    triggers = validate_triggers(policy.get("trigger_rules"), profile_ids, depth_ids)
    severity_items = validate_severity_taxonomy(policy.get("severity_taxonomy"))
    severity_ids = {ensure_string(item["id"], "severity.id") for item in severity_items}
    validate_queue_policy(policy.get("queue_insertion_policy"))
    validate_report_boundary(policy.get("report_output_boundaries"))
    validate_public_claim_boundaries(policy.get("public_claim_boundaries"))
    validate_automation_boundaries(policy.get("automation_boundaries"))
    validate_public_technical_paper(policy.get("public_technical_paper"))
    validate_finding_examples(policy.get("finding_examples", []), severity_ids)
    validate_queue_candidate_examples(policy.get("queue_candidate_examples", []))
    validate_no_forbidden_claims(
        [ensure_string(item, "proposed_report_claims[]") for item in ensure_list(policy.get("proposed_report_claims", []), "proposed_report_claims")],
        "proposed_report_claims",
    )
    report_text = policy.get("simulated_report_text", "")
    if report_text is not None:
        validate_no_secret_text(ensure_string(report_text, "simulated_report_text"), "simulated_report_text")

    return {
        "schema_version": SUMMARY_SCHEMA_VERSION,
        "valid": True,
        "policy_id": policy["policy_id"],
        "audit_profile_ids": sorted(profile_ids),
        "depth_level_ids": sorted(depth_ids),
        "trigger_classes": sorted({item["class"] for item in triggers}),
        "severity_ids": sorted(severity_ids),
        "markers": list(REQUIRED_MARKERS),
        "ready_mutations": [],
        "durable_report_written": False,
        "scheduler_created": False,
        "workflow_created": False,
        "network_used": False,
        "external_watch_executed": False,
        "public_claim_expansion": False,
    }


def validate_summary_fixture(value: Any) -> dict[str, Any]:
    obj = ensure_object(value, "summary_fixture")
    reject_unknown_keys(
        obj,
        {
            "schema_version",
            "valid",
            "policy_id",
            "audit_profile_ids",
            "depth_level_ids",
            "trigger_classes",
            "severity_ids",
            "markers",
            "ready_mutations",
            "durable_report_written",
            "scheduler_created",
            "workflow_created",
            "network_used",
            "external_watch_executed",
            "public_claim_expansion",
        },
        "summary_fixture",
    )
    if obj.get("schema_version") != SUMMARY_SCHEMA_VERSION:
        raise CadenceError("summary fixture schema_version mismatch")
    if not ensure_bool(obj.get("valid"), "summary.valid"):
        raise CadenceError("summary fixture must be valid")
    ensure_string(obj.get("policy_id"), "summary.policy_id")
    if ensure_list(obj.get("ready_mutations"), "summary.ready_mutations"):
        raise CadenceError("summary fixture must not contain ready mutations")
    for field in ["durable_report_written", "scheduler_created", "workflow_created", "network_used", "external_watch_executed", "public_claim_expansion"]:
        if ensure_bool(obj.get(field), f"summary.{field}"):
            raise CadenceError(f"summary fixture must keep {field} false")
    markers = ensure_string_set(obj.get("markers"), "summary.markers")
    if not set(REQUIRED_MARKERS) <= markers:
        raise CadenceError("summary fixture missing required NA-0390 markers")
    return obj


def validate_events(value: Any) -> dict[str, Any]:
    obj = ensure_object(value, "events")
    reject_unknown_keys(obj, {"schema_version", "scenario_id", "events", "expected_recommendation_count"}, "events")
    if obj.get("schema_version") != EVENTS_SCHEMA_VERSION:
        raise CadenceError("events schema_version mismatch")
    ensure_string(obj.get("scenario_id"), "events.scenario_id")
    for index, event in enumerate(ensure_list(obj.get("events"), "events.events")):
        event_obj = ensure_object(event, f"events.events[{index}]")
        reject_unknown_keys(event_obj, {"id", "class", "count", "topic"}, "event")
        ensure_string(event_obj.get("id"), "event.id")
        event_class = ensure_string(event_obj.get("class"), "event.class")
        if event_class not in REQUIRED_TRIGGER_CLASSES:
            raise CadenceError(f"unknown event class: {event_class}")
        if "count" in event_obj and event_obj["count"] is not None and ensure_int(event_obj["count"], "event.count") < 0:
            raise CadenceError("event.count must be non-negative")
        if "topic" in event_obj:
            ensure_string(event_obj.get("topic"), "event.topic")
    if "expected_recommendation_count" in obj:
        ensure_int(obj.get("expected_recommendation_count"), "events.expected_recommendation_count")
    return obj


def event_due_for_trigger(event: dict[str, Any], trigger: dict[str, Any]) -> bool:
    if event["class"] != trigger["class"]:
        return False
    if trigger["class"] in {"pr_count_threshold", "na_count_threshold"}:
        return int(event.get("count", 0)) >= int(trigger["threshold"])
    return True


def simulate(policy: dict[str, Any], events: dict[str, Any]) -> dict[str, Any]:
    policy_summary = validate_policy(policy)
    events_obj = validate_events(events)
    recommendations: list[dict[str, Any]] = []
    seen: set[tuple[str, str, str]] = set()
    for event in ensure_list(events_obj["events"], "events.events"):
        for trigger in policy["trigger_rules"]:
            if not event_due_for_trigger(event, trigger):
                continue
            key = (trigger["audit_profile_id"], trigger["depth_id"], event["class"])
            if key in seen:
                continue
            seen.add(key)
            profile = next(item for item in policy["audit_profiles"] if item["id"] == trigger["audit_profile_id"])
            recommendations.append(
                {
                    "event_id": event["id"],
                    "trigger_class": event["class"],
                    "audit_profile_id": trigger["audit_profile_id"],
                    "depth_id": trigger["depth_id"],
                    "candidate_status": "BACKLOG_CANDIDATE",
                    "ready_mutation": False,
                    "audit_executed": False,
                    "future_gated": bool(profile["future_gated"]),
                    "reason": f"{event['class']} matched {trigger['id']}",
                }
            )
    expected = events_obj.get("expected_recommendation_count")
    if expected is not None and len(recommendations) != expected:
        raise CadenceError(f"simulation recommendation count {len(recommendations)} != expected {expected}")
    return {
        "schema_version": "qsl.routine_audit_cadence.simulation.v1",
        "scenario_id": events_obj["scenario_id"],
        "policy_id": policy_summary["policy_id"],
        "recommendations": recommendations,
        "ready_mutations": [],
        "audit_reports_written": [],
        "durable_report_written": False,
        "scheduler_created": False,
        "workflow_created": False,
        "network_used": False,
        "external_watch_executed": False,
        "markers": list(REQUIRED_MARKERS),
    }


def command_validate(args: argparse.Namespace) -> int:
    tmp_dir = validate_tmp_dir(args.tmp_dir)
    policy = ensure_object(load_json(Path(args.policy)), "policy")
    summary = validate_policy(policy)
    write_json_new(tmp_dir / "validation_summary.json", summary)
    lines = [
        "NA-0390 routine audit cadence validation summary",
        f"policy_id={summary['policy_id']}",
        f"audit_profiles={len(summary['audit_profile_ids'])}",
        f"trigger_classes={len(summary['trigger_classes'])}",
        "ready_mutations=0",
        "durable_report_written=false",
    ]
    for marker in summary["markers"]:
        lines.append(f"MARKER {marker}")
    write_text_new(tmp_dir / "summary.txt", "\n".join(lines) + "\n")
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(f"OK policy={summary['policy_id']} markers={len(summary['markers'])} tmp_dir={tmp_dir}")
    return EXIT_OK


def command_simulate(args: argparse.Namespace) -> int:
    tmp_dir = validate_tmp_dir(args.tmp_dir)
    policy = ensure_object(load_json(Path(args.policy)), "policy")
    events = ensure_object(load_json(Path(args.events)), "events")
    summary = simulate(policy, events)
    write_json_new(tmp_dir / "simulation_summary.json", summary)
    lines = [
        "NA-0390 routine audit cadence simulation summary",
        f"scenario_id={summary['scenario_id']}",
        f"policy_id={summary['policy_id']}",
        f"recommendations={len(summary['recommendations'])}",
        "ready_mutations=0",
        "audit_executed=false",
        "durable_report_written=false",
    ]
    for recommendation in summary["recommendations"]:
        lines.append(
            "RECOMMEND "
            f"profile={recommendation['audit_profile_id']} "
            f"depth={recommendation['depth_id']} "
            f"status={recommendation['candidate_status']} "
            f"future_gated={str(recommendation['future_gated']).lower()}"
        )
    for marker in summary["markers"]:
        lines.append(f"MARKER {marker}")
    write_text_new(tmp_dir / "summary.txt", "\n".join(lines) + "\n")
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(f"OK scenario={summary['scenario_id']} recommendations={len(summary['recommendations'])} tmp_dir={tmp_dir}")
    return EXIT_OK


def run_fixture_case(case: dict[str, Any], fixture_dir: Path) -> tuple[bool, str, list[str]]:
    kind = ensure_string(case.get("kind"), "case.kind")
    markers: list[str] = []
    if kind == "policy":
        summary = validate_policy(ensure_object(load_json(fixture_dir / ensure_string(case.get("path"), "case.path")), "policy"))
        markers.extend(summary["markers"])
        return True, "policy validation passed", markers
    if kind == "policy_mutation":
        summary = validate_policy(load_policy_mutation(fixture_dir / ensure_string(case.get("path"), "case.path"), fixture_dir))
        markers.extend(summary["markers"])
        return True, "mutated policy validation passed", markers
    if kind == "profile":
        validate_profile(ensure_object(load_json(fixture_dir / ensure_string(case.get("path"), "case.path")), "profile"), standalone=True)
        return True, "profile validation passed", markers
    if kind == "trigger":
        validate_trigger(ensure_object(load_json(fixture_dir / ensure_string(case.get("path"), "case.path")), "trigger"), standalone=True)
        return True, "trigger validation passed", markers
    if kind == "severity_taxonomy":
        taxonomy = ensure_object(load_json(fixture_dir / ensure_string(case.get("path"), "case.path")), "severity_taxonomy")
        reject_unknown_keys(taxonomy, {"schema_version", "severity_taxonomy"}, "severity_taxonomy_fixture")
        if taxonomy.get("schema_version") != SCHEMA_VERSION:
            raise CadenceError("severity taxonomy fixture schema_version mismatch")
        validate_severity_taxonomy(taxonomy["severity_taxonomy"])
        return True, "severity taxonomy validation passed", markers
    if kind == "queue_policy":
        value = ensure_object(load_json(fixture_dir / ensure_string(case.get("path"), "case.path")), "queue_policy")
        reject_unknown_keys(value, {"schema_version", "queue_insertion_policy"}, "queue_policy_fixture")
        if value.get("schema_version") != SCHEMA_VERSION:
            raise CadenceError("queue policy fixture schema_version mismatch")
        validate_queue_policy(value["queue_insertion_policy"])
        return True, "queue policy validation passed", markers
    if kind == "report_boundary":
        value = ensure_object(load_json(fixture_dir / ensure_string(case.get("path"), "case.path")), "report_boundary")
        reject_unknown_keys(value, {"schema_version", "report_output_boundaries"}, "report_boundary_fixture")
        if value.get("schema_version") != SCHEMA_VERSION:
            raise CadenceError("report boundary fixture schema_version mismatch")
        validate_report_boundary(value["report_output_boundaries"])
        return True, "report boundary validation passed", markers
    if kind == "summary":
        validate_summary_fixture(load_json(fixture_dir / ensure_string(case.get("path"), "case.path")))
        return True, "summary fixture validation passed", markers
    if kind == "simulate":
        policy = ensure_object(load_json(fixture_dir / ensure_string(case.get("policy"), "case.policy")), "policy")
        events = ensure_object(load_json(fixture_dir / ensure_string(case.get("events"), "case.events")), "events")
        summary = simulate(policy, events)
        markers.extend(summary["markers"])
        return True, "simulation validation passed", markers
    if kind == "malformed_json":
        load_json(fixture_dir / ensure_string(case.get("path"), "case.path"))
        return True, "malformed JSON unexpectedly parsed", markers
    raise CadenceError(f"unknown fixture case kind: {kind}")


def set_mutation_path(policy: dict[str, Any], raw_path: str, value: Any) -> None:
    parts = raw_path.split(".")
    if not parts or any(not part for part in parts):
        raise CadenceError(f"invalid mutation path: {raw_path}")
    current: Any = policy
    for part in parts[:-1]:
        if isinstance(current, list):
            current = current[ensure_int(int(part), "mutation list index")]
        elif isinstance(current, dict):
            if part not in current:
                raise CadenceError(f"mutation path missing: {raw_path}")
            current = current[part]
        else:
            raise CadenceError(f"mutation path cannot descend: {raw_path}")
    last = parts[-1]
    if isinstance(current, list):
        current[ensure_int(int(last), "mutation list index")] = value
    elif isinstance(current, dict):
        current[last] = value
    else:
        raise CadenceError(f"mutation path cannot assign: {raw_path}")


def delete_mutation_path(policy: dict[str, Any], raw_path: str) -> None:
    parts = raw_path.split(".")
    if not parts or any(not part for part in parts):
        raise CadenceError(f"invalid mutation delete path: {raw_path}")
    current: Any = policy
    for part in parts[:-1]:
        if isinstance(current, list):
            current = current[int(part)]
        elif isinstance(current, dict):
            current = current[part]
        else:
            raise CadenceError(f"mutation delete path cannot descend: {raw_path}")
    last = parts[-1]
    if isinstance(current, list):
        del current[int(last)]
    elif isinstance(current, dict):
        current.pop(last, None)
    else:
        raise CadenceError(f"mutation delete path cannot delete: {raw_path}")


def load_policy_mutation(path: Path, fixture_dir: Path) -> dict[str, Any]:
    mutation = ensure_object(load_json(path), "policy_mutation")
    reject_unknown_keys(mutation, {"schema_version", "base", "set", "delete", "remove_profile_ids"}, "policy_mutation")
    if mutation.get("schema_version") != SCHEMA_VERSION:
        raise CadenceError("policy mutation schema_version mismatch")
    base = ensure_object(load_json(fixture_dir / ensure_string(mutation.get("base"), "mutation.base")), "base policy")
    policy = copy.deepcopy(base)
    for raw_path in ensure_list(mutation.get("delete", []), "mutation.delete"):
        delete_mutation_path(policy, ensure_string(raw_path, "mutation.delete[]"))
    for profile_id in ensure_list(mutation.get("remove_profile_ids", []), "mutation.remove_profile_ids"):
        target = ensure_string(profile_id, "mutation.remove_profile_ids[]")
        policy["audit_profiles"] = [item for item in ensure_list(policy.get("audit_profiles"), "audit_profiles") if item.get("id") != target]
    for raw_path, value in ensure_object(mutation.get("set", {}), "mutation.set").items():
        set_mutation_path(policy, raw_path, value)
    return policy


def command_fixture(args: argparse.Namespace) -> int:
    fixture_dir = Path(args.fixture_dir)
    if not fixture_dir.is_dir():
        raise CadenceError(f"fixture directory not found: {fixture_dir}")
    tmp_dir = validate_tmp_dir(args.tmp_dir)
    matrix = ensure_object(load_json(fixture_dir / "fixture_cases.json"), "fixture_cases")
    reject_unknown_keys(matrix, {"schema_version", "cases"}, "fixture_cases")
    if matrix.get("schema_version") != FIXTURE_SCHEMA_VERSION:
        raise CadenceError("fixture matrix schema_version mismatch")
    lines = ["NA-0390 routine audit cadence fixture matrix"]
    passed = 0
    failed = 0
    collected_markers: set[str] = set()
    for raw_case in ensure_list(matrix.get("cases"), "fixture_cases.cases"):
        case = ensure_object(raw_case, "fixture_case")
        reject_unknown_keys(case, {"name", "kind", "path", "policy", "events", "expect"}, "fixture_case")
        name = ensure_string(case.get("name"), "case.name")
        expect = ensure_string(case.get("expect"), "case.expect")
        if expect not in {"pass", "fail"}:
            raise CadenceError("case.expect must be pass or fail")
        try:
            ok, detail, markers = run_fixture_case(case, fixture_dir)
        except CadenceError as exc:
            ok = False
            detail = str(exc)
            markers = []
        expected_ok = expect == "pass"
        if ok == expected_ok:
            passed += 1
            status = "PASS" if ok else "REJECTED"
        else:
            failed += 1
            status = "UNEXPECTED_PASS" if ok else "UNEXPECTED_REJECT"
        collected_markers.update(markers)
        lines.append(f"CASE {name} {status} expected={expect} detail={detail}")
    collected_markers.update(REQUIRED_MARKERS)
    for marker in REQUIRED_MARKERS:
        lines.append(f"MARKER {marker}")
    summary = {
        "schema_version": FIXTURE_SCHEMA_VERSION,
        "passed": passed,
        "failed": failed,
        "case_count": passed + failed,
        "markers": sorted(collected_markers),
        "fixture_log": str(tmp_dir / "fixture_matrix.log"),
    }
    lines.append(f"SUMMARY passed={passed} failed={failed} cases={passed + failed}")
    write_text_new(tmp_dir / "fixture_matrix.log", "\n".join(lines) + "\n")
    write_json_new(tmp_dir / "fixture_summary.json", summary)
    write_text_new(tmp_dir / "summary.txt", f"passed={passed}\nfailed={failed}\ncase_count={passed + failed}\n")
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(f"OK fixture_cases={passed + failed} passed={passed} failed={failed} log={tmp_dir / 'fixture_matrix.log'}")
    return EXIT_OK if failed == 0 else EXIT_VALIDATION


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Validate QSL routine audit cadence policy fixtures and temp-output simulations.")
    sub = parser.add_subparsers(dest="command", required=True)

    fixture = sub.add_parser("fixture", help="Run the deterministic no-network fixture matrix.")
    fixture.add_argument("--fixture-dir", required=True)
    fixture.add_argument("--tmp-dir", required=True)
    fixture.add_argument("--json", action="store_true")
    fixture.set_defaults(func=command_fixture)

    validate = sub.add_parser("validate", help="Validate one cadence policy JSON file.")
    validate.add_argument("--policy", required=True)
    validate.add_argument("--tmp-dir", required=True)
    validate.add_argument("--json", action="store_true")
    validate.set_defaults(func=command_validate)

    simulate_parser = sub.add_parser("simulate", help="Simulate audit trigger recommendations from a policy and event fixture.")
    simulate_parser.add_argument("--policy", required=True)
    simulate_parser.add_argument("--events", required=True)
    simulate_parser.add_argument("--tmp-dir", required=True)
    simulate_parser.add_argument("--json", action="store_true")
    simulate_parser.set_defaults(func=command_simulate)
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return int(args.func(args))
    except CadenceError as exc:
        if getattr(args, "json", False):
            print(json.dumps({"schema_version": SUMMARY_SCHEMA_VERSION, "valid": False, "error": str(exc)}, sort_keys=True))
        else:
            print(f"ERROR {exc}", file=sys.stderr)
        return EXIT_VALIDATION
    except BrokenPipeError:
        return EXIT_OK
    except Exception as exc:  # pragma: no cover - last-resort guard for CLI use.
        print(f"INTERNAL_ERROR {exc}", file=sys.stderr)
        return EXIT_INTERNAL


if __name__ == "__main__":
    raise SystemExit(main())
