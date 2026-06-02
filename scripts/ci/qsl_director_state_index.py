#!/usr/bin/env python3
"""Temp-output-only QSL Director State Index harness.

The helper is local-only, standard-library-only, and advisory-only. It reads
explicit qsl-protocol inputs, validates stale-state guards, writes only under
/srv/qbuild/tmp/NA0403_director_state_index_*, and never calls GitHub, git,
network APIs, workflow tools, or schedulers.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


SCHEMA = "qsl.director_state_index.v1"
FIXTURE_CASE_SCHEMA = "qsl.director_state_index.fixture_case.v1"
FIXTURE_SUMMARY_SCHEMA = "qsl.director_state_index.fixture_matrix.v1"
VALIDATION_SUMMARY_SCHEMA = "qsl.director_state_index.validation_summary.v1"
GENERATOR = "scripts/ci/qsl_director_state_index.py"
GENERATOR_DIRECTIVE_ID = "QSL-DIR-2026-06-02-223"
TEMP_ROOT = Path("/srv/qbuild/tmp")
TEMP_PREFIX = "NA0403_director_state_index_"

EXIT_OK = 0
EXIT_INTERNAL = 1
EXIT_VALIDATION = 2

REQUIRED_INDEX_FIELDS = [
    "schema",
    "generated_at_utc",
    "generator",
    "generator_directive_id",
    "repo_root",
    "origin_main_sha",
    "active_ready",
    "last_done_na",
    "latest_decision_id",
    "duplicate_decision_count",
    "recent_prs",
    "public_safety_status",
    "branch_protection_summary",
    "dependency_advisory_summary",
    "qsl_server_boundary",
    "qsl_attachments_boundary",
    "backup_status_summary",
    "local_history_availability",
    "project_goal_canon_status",
    "active_blockers",
    "public_claim_boundaries",
    "evidence_gaps",
    "future_candidate_lanes",
    "stale_detection",
    "source_references",
    "verification_commands",
    "advisory_only_disclaimer",
    "no_secret_scan_status",
    "markers",
]

REQUIRED_MARKERS = [
    "NA0403_DIRECTOR_STATE_INDEX_HELPER_OK",
    "NA0403_TEMP_OUTPUT_ONLY_OK",
    "NA0403_LIVE_REPO_AUTHORITY_OK",
    "NA0403_QUEUE_STATE_REFERENCE_OK",
    "NA0403_DECISION_STATE_REFERENCE_OK",
    "NA0403_PUBLIC_SAFETY_REFERENCE_OK",
    "NA0403_STALE_ORIGIN_REJECT_OK",
    "NA0403_READY_MISMATCH_REJECT_OK",
    "NA0403_DUPLICATE_DECISION_REJECT_OK",
    "NA0403_PUBLIC_CLAIM_OVERREACH_REJECT_OK",
    "NA0403_SECRET_SENTINEL_REJECT_OK",
    "NA0403_NO_BACKGROUND_WORK_OK",
    "NA0403_NO_DURABLE_LOCAL_INDEX_OK",
    "NA0403_NO_PUBLIC_READINESS_CLAIM_OK",
    "NA0403_NO_RUNTIME_CHANGE_OK",
    "NA0403_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK",
    "NA0403_NO_DEPENDENCY_CHANGE_OK",
    "NA0403_NO_WORKFLOW_CHANGE_OK",
    "NA0403_NO_SECRET_MATERIAL_OK",
    "NA0403_DIRECTOR_STATE_INDEX_HARNESS_OK",
]

REQUIRED_FIXTURES = [
    "valid_current_state.json",
    "stale_origin_main_reject.json",
    "ready_mismatch_reject.json",
    "latest_decision_mismatch_reject.json",
    "duplicate_decision_reject.json",
    "multiple_ready_reject.json",
    "missing_public_safety_reject.json",
    "red_public_safety_reject.json",
    "missing_branch_protection_warn.json",
    "qsl_server_unavailable_warn.json",
    "qsl_attachments_unavailable_warn.json",
    "backup_status_unavailable_warn.json",
    "secret_sentinel_reject.json",
    "public_claim_overreach_reject.json",
    "unknown_schema_reject.json",
    "malformed_json_reject.json",
    "durable_output_path_reject.json",
    "response_archive_output_path_reject.json",
    "public_docs_output_path_reject.json",
    "advisory_only_no_override_ok.json",
]

EXPECTED_FIXTURE_OUTCOMES = {
    "valid_current_state.json": ("pass", 0),
    "stale_origin_main_reject.json": ("fail", 0),
    "ready_mismatch_reject.json": ("fail", 0),
    "latest_decision_mismatch_reject.json": ("fail", 0),
    "duplicate_decision_reject.json": ("fail", 0),
    "multiple_ready_reject.json": ("fail", 0),
    "missing_public_safety_reject.json": ("fail", 0),
    "red_public_safety_reject.json": ("fail", 0),
    "missing_branch_protection_warn.json": ("pass", 1),
    "qsl_server_unavailable_warn.json": ("pass", 1),
    "qsl_attachments_unavailable_warn.json": ("pass", 1),
    "backup_status_unavailable_warn.json": ("pass", 1),
    "secret_sentinel_reject.json": ("fail", 0),
    "public_claim_overreach_reject.json": ("fail", 0),
    "unknown_schema_reject.json": ("fail", 0),
    "malformed_json_reject.json": ("fail", 0),
    "durable_output_path_reject.json": ("fail", 0),
    "response_archive_output_path_reject.json": ("fail", 0),
    "public_docs_output_path_reject.json": ("fail", 0),
    "advisory_only_no_override_ok.json": ("pass", 0),
}

EXPECTED_ORIGIN_MAIN = "779e1cf2edbb2f942ff940235c695d02e5b2beae"
EXPECTED_READY = "NA-0403"
EXPECTED_LATEST_DECISION = "D-0788"
EXPECTED_PUBLIC_SAFETY = "success"

SECRET_PATTERNS = [
    ("na0403_secret_sentinel", re.compile(r"\bNA0403_SECRET_SENTINEL\b")),
    ("qsl_forbidden_marker", re.compile(r"\bQSL_TEST_FORBIDDEN_MARKER_VALUE\b")),
    ("private_key", re.compile(r"-----BEGIN (?:[A-Z0-9]+ )?PRIVATE KEY-----")),
    ("github_token", re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{30,}\b")),
    ("github_pat", re.compile(r"\bgithub_pat_[A-Za-z0-9_]{30,}\b")),
    ("openai_key", re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b")),
    ("aws_access_key_id", re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b")),
]

PUBLIC_OVERCLAIM_PATTERNS = [
    ("reject_production_ready", re.compile(r"(?i)\bproduction[- ]ready\b|\bready for production\b")),
    ("reject_public_internet_ready", re.compile(r"(?i)\bpublic[- ]internet[- ]ready\b")),
    ("reject_external_review_complete", re.compile(r"(?i)\bexternal review complete\b|\bexternally reviewed\b")),
    ("reject_metadata_free_runtime", re.compile(r"(?i)\bmetadata[- ]free runtime\b")),
    ("reject_anonymous_by_default", re.compile(r"(?i)\banonymous by default\b")),
    ("reject_untraceable", re.compile(r"(?i)\buntraceable\b")),
    ("reject_vulnerability_free", re.compile(r"(?i)\bvulnerability[- ]free\b")),
    ("reject_bug_free", re.compile(r"(?i)\bbug[- ]free\b")),
    ("reject_perfect_crypto", re.compile(r"(?i)\bperfect crypto\b|\bcryptographically perfect\b")),
    ("reject_source_override_claim", re.compile(r"(?i)\bsource[- ]of[- ]truth index\b|\bauthoritative index\b")),
]

QUEUE_RE = re.compile(r"^### (NA-\d{4}[A-Z]?)\s+[—-]{1,2}\s+(.+)$")
DECISION_RE = re.compile(r"^\s*-\s+\*\*ID:\*\*\s+(D-\d{4})\s*$")


class IndexErrorClosed(RuntimeError):
    """Expected fail-closed validation error."""


def utc_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def is_relative_to(path: Path, parent: Path) -> bool:
    try:
        path.relative_to(parent)
    except ValueError:
        return False
    return True


def reject_ambiguous_path(raw: str, *, allow_absolute: bool) -> None:
    if "\x00" in raw:
        raise IndexErrorClosed("path contains NUL byte")
    if "\\" in raw:
        raise IndexErrorClosed(f"path uses backslash separators: {raw}")
    if raw != raw.strip():
        raise IndexErrorClosed(f"path has leading or trailing whitespace: {raw!r}")
    if raw.startswith("~"):
        raise IndexErrorClosed(f"home-relative path rejected: {raw}")
    if not allow_absolute and raw.startswith("/"):
        raise IndexErrorClosed(f"absolute path rejected: {raw}")
    parts = Path(raw).parts
    if any(part in {"..", ""} for part in parts):
        raise IndexErrorClosed(f"parent traversal or empty path segment rejected: {raw}")


def validate_temp_output_dir(raw: str) -> Path:
    reject_ambiguous_path(raw, allow_absolute=True)
    resolved = Path(raw).resolve(strict=False)
    temp_root = TEMP_ROOT.resolve(strict=True)
    if resolved == temp_root or not is_relative_to(resolved, temp_root):
        raise IndexErrorClosed("output directory must be under /srv/qbuild/tmp")
    rel_parts = resolved.relative_to(temp_root).parts
    if not rel_parts or not rel_parts[0].startswith(TEMP_PREFIX):
        raise IndexErrorClosed("output directory must use NA0403_director_state_index_* temp prefix")
    if resolved.is_symlink():
        raise IndexErrorClosed("output directory symlink rejected")
    resolved.mkdir(parents=True, exist_ok=True)
    return resolved


def validate_index_output_path(raw: str) -> None:
    reject_ambiguous_path(raw, allow_absolute=True)
    resolved = Path(raw).resolve(strict=False)
    forbidden_roots = [
        Path("/home/victor/work/qsl/codex").resolve(strict=False),
        Path("/srv/qbuild/tools").resolve(strict=False),
    ]
    for root in forbidden_roots:
        if resolved == root or is_relative_to(resolved, root):
            raise IndexErrorClosed(f"forbidden output path rejected: {raw}")
    parts = resolved.parts
    if "docs" in parts and "public" in parts:
        raise IndexErrorClosed(f"public docs output path rejected: {raw}")
    if "website" in parts:
        raise IndexErrorClosed(f"website output path rejected: {raw}")
    temp_root = TEMP_ROOT.resolve(strict=True)
    if resolved == temp_root or not is_relative_to(resolved, temp_root):
        raise IndexErrorClosed(f"durable index output path rejected: {raw}")
    rel_parts = resolved.relative_to(temp_root).parts
    if not rel_parts or not rel_parts[0].startswith(TEMP_PREFIX):
        raise IndexErrorClosed(f"non-NA0403 temp output path rejected: {raw}")


def ensure_object(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise IndexErrorClosed(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise IndexErrorClosed(f"{label} must be a list")
    return value


def walk_strings(value: Any) -> list[str]:
    strings: list[str] = []
    if isinstance(value, str):
        strings.append(value)
    elif isinstance(value, dict):
        for key, item in value.items():
            strings.extend(walk_strings(key))
            strings.extend(walk_strings(item))
    elif isinstance(value, list):
        for item in value:
            strings.extend(walk_strings(item))
    return strings


def scan_secret_strings(value: Any) -> list[str]:
    findings: list[str] = []
    for text in walk_strings(value):
        for name, pattern in SECRET_PATTERNS:
            if pattern.search(text):
                findings.append(name)
    return sorted(set(findings))


def scan_public_overclaims(value: Any) -> list[str]:
    findings: list[str] = []
    for text in walk_strings(value):
        for name, pattern in PUBLIC_OVERCLAIM_PATTERNS:
            if pattern.search(text):
                findings.append(name)
    return sorted(set(findings))


def load_json_file(path: Path) -> dict[str, Any]:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise IndexErrorClosed(f"could not read JSON file: {path}") from exc
    try:
        data = json.loads(text)
    except json.JSONDecodeError as exc:
        raise IndexErrorClosed(f"malformed JSON rejected: {path}: {exc.msg}") from exc
    return ensure_object(data, "index JSON")


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def short_sha(value: str) -> str:
    return value[:12]


def parse_queue(repo_root: Path) -> tuple[dict[str, str], dict[str, Any]]:
    next_actions = repo_root / "NEXT_ACTIONS.md"
    text = next_actions.read_text(encoding="utf-8")
    items: list[dict[str, str]] = []
    current: dict[str, str] | None = None
    for line in text.splitlines():
        match = QUEUE_RE.match(line)
        if match:
            current = {"id": match.group(1), "title": match.group(2).strip(), "status": "UNKNOWN"}
            items.append(current)
            continue
        if current is not None and line.startswith("Status:"):
            current["status"] = line.split(":", 1)[1].strip()
    ready = [item for item in items if item["status"] == "READY"]
    done = [item for item in items if item["status"] == "DONE"]
    active = ready[0] if ready else {"id": "NONE", "title": "NONE", "status": "MISSING"}
    last_done = max(done, key=lambda item: item["id"]) if done else {"id": "NONE", "title": "NONE", "status": "MISSING"}
    stale = {
        "ready_count": len(ready),
        "ready_ids": [item["id"] for item in ready],
        "next_actions_path": "NEXT_ACTIONS.md",
        "last_done_source": "NEXT_ACTIONS.md",
    }
    return {"id": active["id"], "title": active["title"], "status": active["status"]}, {
        "id": last_done["id"],
        "title": last_done["title"],
        "status": last_done["status"],
        **stale,
    }


def parse_decisions(repo_root: Path) -> tuple[str, int, dict[str, int]]:
    decisions = repo_root / "DECISIONS.md"
    counts: dict[str, int] = {}
    for line in decisions.read_text(encoding="utf-8").splitlines():
        match = DECISION_RE.match(line)
        if match:
            decision_id = match.group(1)
            counts[decision_id] = counts.get(decision_id, 0) + 1
    if not counts:
        return "NONE", 0, counts
    latest = max(counts, key=lambda item: int(item.split("-")[1]))
    duplicates = sum(1 for count in counts.values() if count > 1)
    return latest, duplicates, counts


def rustls_webpki_version(repo_root: Path) -> str:
    lock = repo_root / "Cargo.lock"
    if not lock.exists():
        return "missing"
    lines = lock.read_text(encoding="utf-8", errors="replace").splitlines()
    for index, line in enumerate(lines):
        if line.strip() == 'name = "rustls-webpki"':
            for candidate in lines[index : index + 8]:
                stripped = candidate.strip()
                if stripped.startswith("version = "):
                    return stripped.split("=", 1)[1].strip().strip('"')
    return "missing"


def project_goal_canon_status(repo_root: Path) -> dict[str, Any]:
    path = repo_root / "docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md"
    markers = [
        "QSL_PROJECT_GOAL_CANON_INTERNAL_ONLY",
        "QSL_ONE_READY_QUEUE_DISCIPLINE",
        "QSL_NO_PUBLIC_OVERCLAIMING",
    ]
    if not path.exists():
        return {"status": "missing", "path": path.as_posix(), "markers": {}}
    text = path.read_text(encoding="utf-8", errors="replace")
    return {
        "status": "present",
        "path": "docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md",
        "markers": {marker: (marker in text) for marker in markers},
    }


def build_index(
    *,
    repo_root: Path,
    tmp_dir: Path,
    origin_main_sha: str,
    public_safety_status: str,
) -> dict[str, Any]:
    active_ready, queue_state = parse_queue(repo_root)
    latest_decision, duplicate_count, decision_counts = parse_decisions(repo_root)
    index_json_path = tmp_dir / "director_state_index.json"
    return {
        "schema": SCHEMA,
        "generated_at_utc": utc_now(),
        "generator": GENERATOR,
        "generator_directive_id": GENERATOR_DIRECTIVE_ID,
        "repo_root": repo_root.resolve(strict=False).as_posix(),
        "origin_main_sha": origin_main_sha,
        "active_ready": active_ready,
        "last_done_na": {
            "id": queue_state["id"],
            "title": queue_state["title"],
            "status": queue_state["status"],
        },
        "latest_decision_id": latest_decision,
        "duplicate_decision_count": duplicate_count,
        "recent_prs": [
            {"repo": "qsl-protocol", "number": 1068, "state": "merged", "merge_sha12": "779e1cf2edbb"},
            {"repo": "qsl-protocol", "number": 1067, "state": "merged", "merge_sha12": "4506549ec4ef"},
        ],
        "public_safety_status": {
            "status": public_safety_status,
            "required": True,
            "source": "caller_supplied_read_only_evidence",
        },
        "branch_protection_summary": {
            "status": "caller_evidence_required",
            "evidence_incomplete": True,
            "required_context": "public-safety",
            "mutation_allowed": False,
        },
        "dependency_advisory_summary": {
            "status": "caller_evidence_required",
            "cargo_audit_required": True,
            "rustls_webpki_version": rustls_webpki_version(repo_root),
            "mutation_allowed": False,
        },
        "qsl_server_boundary": {
            "status": "evidence_incomplete",
            "evidence_incomplete": True,
            "boundary": "bounded harness evidence only",
            "expected_pr": 56,
            "expected_merge_sha12": "d40e6003fdf0",
        },
        "qsl_attachments_boundary": {
            "status": "evidence_incomplete",
            "evidence_incomplete": True,
            "boundary": "service-local prerequisite evidence only",
            "expected_pr": 37,
            "expected_merge_sha12": "96b9352bd63",
        },
        "backup_status_summary": {
            "status": "evidence_incomplete",
            "evidence_incomplete": True,
            "summary": "same-host continuity evidence must be checked live",
            "durable_index_backup_review_required": True,
        },
        "local_history_availability": {
            "status": "not_scanned_by_helper",
            "allowed_read_only_root_labels": [
                "response_archive",
                "request_archive",
                "directive_archive",
                "journal_archive",
                "ops_history",
            ],
        },
        "project_goal_canon_status": project_goal_canon_status(repo_root),
        "active_blockers": [
            "durable Director State Index storage remains future-gated",
            "public technical paper remains future-gated",
            "D132 cleanup remains not authorized",
        ],
        "public_claim_boundaries": {
            "status": "bounded_internal_governance_only",
            "not_public_docs": True,
            "not_external_review": True,
            "not_public_technical_paper": True,
            "not_public_readiness": True,
        },
        "evidence_gaps": [
            "branch protection must be supplied by live read-only evidence",
            "public-safety must be supplied by live read-only evidence",
            "qsl-server and qsl-attachments must be checked live before relying on them",
            "backup continuity must be checked live before relying on it",
        ],
        "future_candidate_lanes": [
            {
                "id": "NA-0404",
                "title": "QSL Director State Index Durable Storage / Backup Impact Authorization Plan",
                "auto_promoted": False,
            }
        ],
        "stale_detection": {
            "origin_main_sha": origin_main_sha,
            "expected_ready": active_ready["id"],
            "latest_decision_id": latest_decision,
            "ready_count": queue_state["ready_count"],
            "ready_ids": queue_state["ready_ids"],
            "decision_counts": {key: decision_counts[key] for key in sorted(decision_counts)},
            "public_safety_status": public_safety_status,
            "output_path": index_json_path.as_posix(),
            "live_evidence_revalidation_required": True,
        },
        "source_references": [
            {"path": "NEXT_ACTIONS.md", "purpose": "queue_state"},
            {"path": "DECISIONS.md", "purpose": "decision_state"},
            {"path": "TRACEABILITY.md", "purpose": "traceability_state"},
            {
                "path": "docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md",
                "purpose": "project_goal_canon",
            },
        ],
        "verification_commands": [
            "python3 scripts/ci/qsl_director_state_index.py generate ...",
            "python3 scripts/ci/qsl_director_state_index.py validate ...",
            "python3 scripts/ci/qsl_evidence_helper.py queue",
            "python3 scripts/ci/qsl_evidence_helper.py decisions",
        ],
        "advisory_only_disclaimer": {
            "status": "advisory_only",
            "live_repo_github_ci_remain_authority": True,
            "may_not_override_live_evidence": True,
            "internal_local_ops_governance_only": True,
        },
        "no_secret_scan_status": {
            "status": "passed",
            "sentinel_findings": [],
            "scanner": "qsl_director_state_index_string_scan",
        },
        "markers": list(REQUIRED_MARKERS),
    }


def deep_merge(target: dict[str, Any], updates: dict[str, Any]) -> None:
    for key, value in updates.items():
        if isinstance(value, dict) and isinstance(target.get(key), dict):
            deep_merge(target[key], value)
        else:
            target[key] = copy.deepcopy(value)


def remove_path(target: dict[str, Any], raw_path: str) -> None:
    parts = raw_path.split(".")
    if not parts or any(not part for part in parts):
        raise IndexErrorClosed(f"invalid fixture remove path: {raw_path}")
    current: Any = target
    for part in parts[:-1]:
        if not isinstance(current, dict) or part not in current:
            return
        current = current[part]
    if isinstance(current, dict):
        current.pop(parts[-1], None)


def base_fixture_index() -> dict[str, Any]:
    return {
        "schema": SCHEMA,
        "generated_at_utc": "2026-06-02T00:00:00Z",
        "generator": GENERATOR,
        "generator_directive_id": GENERATOR_DIRECTIVE_ID,
        "repo_root": "/srv/qbuild/work/NA-0403/qsl-protocol",
        "origin_main_sha": EXPECTED_ORIGIN_MAIN,
        "active_ready": {
            "id": EXPECTED_READY,
            "title": "QSL Director State Index Implementation Harness",
            "status": "READY",
        },
        "last_done_na": {
            "id": "NA-0402",
            "title": "QSL Director State Index Authorization Plan",
            "status": "DONE",
        },
        "latest_decision_id": EXPECTED_LATEST_DECISION,
        "duplicate_decision_count": 0,
        "recent_prs": [
            {"repo": "qsl-protocol", "number": 1068, "state": "merged", "merge_sha12": "779e1cf2edbb"},
            {"repo": "qsl-protocol", "number": 1067, "state": "merged", "merge_sha12": "4506549ec4ef"},
        ],
        "public_safety_status": {
            "status": EXPECTED_PUBLIC_SAFETY,
            "required": True,
            "source": "fixture",
        },
        "branch_protection_summary": {
            "status": "present",
            "evidence_incomplete": False,
            "required_context": "public-safety",
            "mutation_allowed": False,
        },
        "dependency_advisory_summary": {
            "status": "present",
            "cargo_audit_required": True,
            "rustls_webpki_version": "0.103.13",
            "mutation_allowed": False,
        },
        "qsl_server_boundary": {
            "status": "present",
            "evidence_incomplete": False,
            "boundary": "bounded harness evidence only",
            "expected_pr": 56,
            "expected_merge_sha12": "d40e6003fdf0",
        },
        "qsl_attachments_boundary": {
            "status": "present",
            "evidence_incomplete": False,
            "boundary": "service-local prerequisite evidence only",
            "expected_pr": 37,
            "expected_merge_sha12": "96b9352bd63",
        },
        "backup_status_summary": {
            "status": "present",
            "evidence_incomplete": False,
            "summary": "same-host continuity evidence checked by caller",
            "durable_index_backup_review_required": True,
        },
        "local_history_availability": {
            "status": "fixture",
            "response_archive": "present",
            "request_archive": "present",
            "directive_archive": "absent",
            "journal_archive": "absent",
            "ops_history": "present",
        },
        "project_goal_canon_status": {
            "status": "present",
            "path": "docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md",
            "markers": {
                "QSL_PROJECT_GOAL_CANON_INTERNAL_ONLY": True,
                "QSL_ONE_READY_QUEUE_DISCIPLINE": True,
                "QSL_NO_PUBLIC_OVERCLAIMING": True,
            },
        },
        "active_blockers": ["durable Director State Index storage remains future-gated"],
        "public_claim_boundaries": {
            "status": "bounded_internal_governance_only",
            "not_public_docs": True,
            "not_external_review": True,
            "not_public_technical_paper": True,
            "not_public_readiness": True,
        },
        "evidence_gaps": [],
        "future_candidate_lanes": [
            {
                "id": "NA-0404",
                "title": "QSL Director State Index Durable Storage / Backup Impact Authorization Plan",
                "auto_promoted": False,
            }
        ],
        "stale_detection": {
            "origin_main_sha": EXPECTED_ORIGIN_MAIN,
            "expected_ready": EXPECTED_READY,
            "latest_decision_id": EXPECTED_LATEST_DECISION,
            "ready_count": 1,
            "ready_ids": [EXPECTED_READY],
            "decision_counts": {
                "D-0786": 1,
                "D-0787": 1,
                "D-0788": 1,
            },
            "public_safety_status": EXPECTED_PUBLIC_SAFETY,
            "output_path": "/srv/qbuild/tmp/NA0403_director_state_index_fixture/director_state_index.json",
            "live_evidence_revalidation_required": True,
        },
        "source_references": [
            {"path": "NEXT_ACTIONS.md", "purpose": "queue_state"},
            {"path": "DECISIONS.md", "purpose": "decision_state"},
            {"path": "TRACEABILITY.md", "purpose": "traceability_state"},
        ],
        "verification_commands": [
            "python3 scripts/ci/qsl_director_state_index.py fixture ...",
            "python3 scripts/ci/qsl_director_state_index.py validate ...",
        ],
        "advisory_only_disclaimer": {
            "status": "advisory_only",
            "live_repo_github_ci_remain_authority": True,
            "may_not_override_live_evidence": True,
            "internal_local_ops_governance_only": True,
        },
        "no_secret_scan_status": {
            "status": "passed",
            "sentinel_findings": [],
            "scanner": "qsl_director_state_index_string_scan",
        },
        "markers": list(REQUIRED_MARKERS),
    }


def fixture_to_index(raw: dict[str, Any]) -> dict[str, Any]:
    if raw.get("fixture_schema") != FIXTURE_CASE_SCHEMA:
        return raw
    data = base_fixture_index()
    set_values = raw.get("set", {})
    if set_values:
        deep_merge(data, ensure_object(set_values, "fixture set"))
    for raw_path in raw.get("remove", []):
        remove_path(data, str(raw_path))
    return data


def validate_index(
    data: dict[str, Any],
    *,
    origin_main_sha: str,
    expected_ready: str,
    expected_latest_decision: str,
    public_safety_status: str,
) -> list[str]:
    warnings: list[str] = []
    schema = data.get("schema")
    if schema != SCHEMA:
        raise IndexErrorClosed(f"unknown schema rejected: {schema!r}")
    missing = [field for field in REQUIRED_INDEX_FIELDS if field not in data]
    if missing:
        raise IndexErrorClosed(f"required index fields missing: {', '.join(missing)}")

    secret_findings = scan_secret_strings(data)
    if secret_findings:
        raise IndexErrorClosed("secret sentinel/material rejected: " + ", ".join(secret_findings))

    public_claim_findings = scan_public_overclaims(data)
    if data.get("public_claim_overreach") is True or public_claim_findings:
        detail = ", ".join(public_claim_findings) if public_claim_findings else "explicit_overreach_flag"
        raise IndexErrorClosed("public-claim overreach rejected: " + detail)

    disclaimer = ensure_object(data.get("advisory_only_disclaimer"), "advisory_only_disclaimer")
    if disclaimer.get("status") != "advisory_only":
        raise IndexErrorClosed("index must be advisory-only")
    if disclaimer.get("live_repo_github_ci_remain_authority") is not True:
        raise IndexErrorClosed("index cannot override live repo/GitHub/CI evidence")
    if disclaimer.get("may_not_override_live_evidence") is not True:
        raise IndexErrorClosed("index override claim rejected")

    if data.get("origin_main_sha") != origin_main_sha:
        raise IndexErrorClosed("stale origin/main rejected")
    stale = ensure_object(data.get("stale_detection"), "stale_detection")
    if stale.get("origin_main_sha") != origin_main_sha:
        raise IndexErrorClosed("stale detection origin/main mismatch rejected")
    if stale.get("ready_count") != 1:
        raise IndexErrorClosed("more than one READY rejected")

    active_ready = ensure_object(data.get("active_ready"), "active_ready")
    if active_ready.get("id") != expected_ready:
        raise IndexErrorClosed("READY mismatch rejected")
    if stale.get("expected_ready") != expected_ready:
        raise IndexErrorClosed("stale detection READY mismatch rejected")
    if data.get("latest_decision_id") != expected_latest_decision:
        raise IndexErrorClosed("latest decision mismatch rejected")
    if stale.get("latest_decision_id") != expected_latest_decision:
        raise IndexErrorClosed("stale detection latest decision mismatch rejected")
    if data.get("duplicate_decision_count") != 0:
        raise IndexErrorClosed("duplicate decision rejected")
    for decision_id, count in ensure_object(stale.get("decision_counts"), "stale_detection.decision_counts").items():
        if count != 1:
            raise IndexErrorClosed(f"duplicate decision rejected: {decision_id}")

    public_safety = ensure_object(data.get("public_safety_status"), "public_safety_status")
    if public_safety_status != "success":
        raise IndexErrorClosed("public-safety input is not success")
    if public_safety.get("status") != public_safety_status:
        raise IndexErrorClosed("public-safety mismatch rejected")

    markers = ensure_list(data.get("markers"), "markers")
    marker_missing = [marker for marker in REQUIRED_MARKERS if marker not in markers]
    if marker_missing:
        raise IndexErrorClosed("required markers missing: " + ", ".join(marker_missing))

    for path_key in ("requested_output_path",):
        if path_key in data:
            validate_index_output_path(str(data[path_key]))
    for path_key in ("output_path", "requested_output_path"):
        if path_key in stale:
            validate_index_output_path(str(stale[path_key]))

    branch = ensure_object(data.get("branch_protection_summary"), "branch_protection_summary")
    if branch.get("status") in {"missing", "unavailable", "unknown", "caller_evidence_required"}:
        if branch.get("evidence_incomplete") is True:
            warnings.append("branch_protection_evidence_incomplete")
        else:
            raise IndexErrorClosed("branch protection missing without evidence-incomplete marker")

    for key, warning in (
        ("qsl_server_boundary", "qsl_server_evidence_incomplete"),
        ("qsl_attachments_boundary", "qsl_attachments_evidence_incomplete"),
        ("backup_status_summary", "backup_status_evidence_incomplete"),
    ):
        value = ensure_object(data.get(key), key)
        if value.get("status") in {"missing", "unavailable", "unknown", "evidence_incomplete"}:
            if value.get("evidence_incomplete") is True:
                warnings.append(warning)
            else:
                raise IndexErrorClosed(f"{key} unavailable without evidence-incomplete marker")

    no_secret = ensure_object(data.get("no_secret_scan_status"), "no_secret_scan_status")
    if no_secret.get("status") != "passed":
        raise IndexErrorClosed("no-secret scan status rejected")
    return sorted(set(warnings))


def validation_summary(data: dict[str, Any], warnings: list[str], *, index_path: str | None = None) -> dict[str, Any]:
    return {
        "schema": VALIDATION_SUMMARY_SCHEMA,
        "result": "pass",
        "index_path": index_path,
        "warning_count": len(warnings),
        "warnings": warnings,
        "markers": list(REQUIRED_MARKERS),
    }


def print_human_validation(summary: dict[str, Any]) -> None:
    print("NA-0403 Director State Index validation: PASS")
    print(f"warnings={summary['warning_count']}")
    for warning in summary["warnings"]:
        print(f"WARNING {warning}")
    for marker in summary["markers"]:
        print(f"MARKER {marker}")


def command_validate(args: argparse.Namespace) -> int:
    data = load_json_file(Path(args.index_json))
    warnings = validate_index(
        data,
        origin_main_sha=args.origin_main_sha,
        expected_ready=args.expected_ready,
        expected_latest_decision=args.expected_latest_decision,
        public_safety_status=args.public_safety_status,
    )
    summary = validation_summary(data, warnings, index_path=args.index_json)
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print_human_validation(summary)
    return EXIT_OK


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_human_summary(path: Path, lines: list[str]) -> None:
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def command_generate(args: argparse.Namespace) -> int:
    tmp_dir = validate_temp_output_dir(args.tmp_dir)
    repo_root = Path(args.repo_root).resolve(strict=False)
    if not repo_root.exists():
        raise IndexErrorClosed(f"repo root does not exist: {repo_root}")
    data = build_index(
        repo_root=repo_root,
        tmp_dir=tmp_dir,
        origin_main_sha=args.origin_main_sha,
        public_safety_status=args.public_safety_status,
    )
    warnings = validate_index(
        data,
        origin_main_sha=args.origin_main_sha,
        expected_ready=args.expected_ready,
        expected_latest_decision=args.expected_latest_decision,
        public_safety_status=args.public_safety_status,
    )
    index_path = tmp_dir / "director_state_index.json"
    summary_path = tmp_dir / "director_state_index_summary.txt"
    write_json(index_path, data)
    digest = sha256_file(index_path)
    lines = [
        "NA-0403 Director State Index generation: PASS",
        f"schema={SCHEMA}",
        f"origin_main_sha12={short_sha(args.origin_main_sha)}",
        f"active_ready={data['active_ready']['id']}",
        f"latest_decision_id={data['latest_decision_id']}",
        f"public_safety_status={args.public_safety_status}",
        f"warning_count={len(warnings)}",
        f"index_path={index_path.as_posix()}",
        f"index_sha256={digest}",
        "advisory_only=true",
        "live_evidence_revalidation_required=true",
        *[f"WARNING {warning}" for warning in warnings],
        *[f"MARKER {marker}" for marker in REQUIRED_MARKERS],
    ]
    write_human_summary(summary_path, lines)
    summary = {
        "schema": VALIDATION_SUMMARY_SCHEMA,
        "result": "pass",
        "index_path": index_path.as_posix(),
        "summary_path": summary_path.as_posix(),
        "index_sha256": digest,
        "warning_count": len(warnings),
        "warnings": warnings,
        "markers": list(REQUIRED_MARKERS),
    }
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print("\n".join(lines))
    return EXIT_OK


def fixture_case_result(path: Path) -> tuple[str, list[str], str]:
    try:
        data = fixture_to_index(load_json_file(path))
        warnings = validate_index(
            data,
            origin_main_sha=EXPECTED_ORIGIN_MAIN,
            expected_ready=EXPECTED_READY,
            expected_latest_decision=EXPECTED_LATEST_DECISION,
            public_safety_status=EXPECTED_PUBLIC_SAFETY,
        )
        return "pass", warnings, ""
    except IndexErrorClosed as exc:
        return "fail", [], str(exc)


def command_fixture(args: argparse.Namespace) -> int:
    fixtures_dir = Path(args.fixtures_dir).resolve(strict=False)
    if not fixtures_dir.exists():
        raise IndexErrorClosed(f"fixtures directory does not exist: {fixtures_dir}")
    tmp_dir = validate_temp_output_dir(args.tmp_dir)
    fixture_names = sorted(path.name for path in fixtures_dir.glob("*.json"))
    missing = [name for name in REQUIRED_FIXTURES if name not in fixture_names]
    extra = [name for name in fixture_names if name not in REQUIRED_FIXTURES]
    if missing:
        raise IndexErrorClosed("required fixtures missing: " + ", ".join(missing))
    if extra:
        raise IndexErrorClosed("unexpected fixtures rejected: " + ", ".join(extra))

    results: list[dict[str, Any]] = []
    failures: list[str] = []
    for name in REQUIRED_FIXTURES:
        path = fixtures_dir / name
        expected_result, minimum_warning_count = EXPECTED_FIXTURE_OUTCOMES[name]
        actual_result, warnings, error = fixture_case_result(path)
        outcome_ok = actual_result == expected_result and len(warnings) >= minimum_warning_count
        if not outcome_ok:
            failures.append(name)
        results.append(
            {
                "name": name,
                "expected_result": expected_result,
                "actual_result": actual_result,
                "expected_minimum_warning_count": minimum_warning_count,
                "warning_count": len(warnings),
                "warnings": warnings,
                "error": error,
                "ok": outcome_ok,
            }
        )

    if failures:
        raise IndexErrorClosed("fixture expectations failed: " + ", ".join(failures))

    summary = {
        "schema": FIXTURE_SUMMARY_SCHEMA,
        "result": "pass",
        "fixture_count": len(results),
        "pass_count": sum(1 for item in results if item["ok"]),
        "fail_count": 0,
        "results": results,
        "markers": list(REQUIRED_MARKERS),
        "tmp_dir": tmp_dir.as_posix(),
    }
    summary_json = tmp_dir / "fixture_summary.json"
    summary_txt = tmp_dir / "fixture_summary.txt"
    write_json(summary_json, summary)
    lines = [
        "NA-0403 Director State Index fixture matrix: PASS",
        f"fixture_count={summary['fixture_count']}",
        f"pass_count={summary['pass_count']}",
        f"summary_json={summary_json.as_posix()}",
        f"summary_txt={summary_txt.as_posix()}",
    ]
    for item in results:
        lines.append(
            "CASE "
            + item["name"]
            + f" expected={item['expected_result']} actual={item['actual_result']}"
            + f" warnings={item['warning_count']} ok={str(item['ok']).lower()}"
        )
    lines.extend(f"MARKER {marker}" for marker in REQUIRED_MARKERS)
    write_human_summary(summary_txt, lines)
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print("\n".join(lines))
    return EXIT_OK


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="QSL Director State Index temp-output harness.")
    sub = parser.add_subparsers(dest="command", required=True)

    fixture = sub.add_parser("fixture", help="Run deterministic fixture matrix.")
    fixture.add_argument("--fixtures-dir", required=True)
    fixture.add_argument("--tmp-dir", required=True)
    fixture.add_argument("--json", action="store_true")
    fixture.set_defaults(func=command_fixture)

    generate = sub.add_parser("generate", help="Generate a temp-only Director State Index.")
    generate.add_argument("--repo-root", required=True)
    generate.add_argument("--tmp-dir", required=True)
    generate.add_argument("--origin-main-sha", required=True)
    generate.add_argument("--expected-ready", required=True)
    generate.add_argument("--expected-latest-decision", required=True)
    generate.add_argument("--public-safety-status", choices=("success", "failure", "missing", "unknown"), required=True)
    generate.add_argument("--json", action="store_true")
    generate.set_defaults(func=command_generate)

    validate = sub.add_parser("validate", help="Validate one Director State Index JSON file.")
    validate.add_argument("--index-json", required=True)
    validate.add_argument("--origin-main-sha", required=True)
    validate.add_argument("--expected-ready", required=True)
    validate.add_argument("--expected-latest-decision", required=True)
    validate.add_argument("--public-safety-status", choices=("success", "failure", "missing", "unknown"), required=True)
    validate.add_argument("--json", action="store_true")
    validate.set_defaults(func=command_validate)
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return args.func(args)
    except IndexErrorClosed as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return EXIT_VALIDATION
    except OSError as exc:
        print(f"ERROR: IO failure: {exc}", file=sys.stderr)
        return EXIT_INTERNAL


if __name__ == "__main__":
    raise SystemExit(main())
