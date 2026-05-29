#!/usr/bin/env python3
"""Validate QSL local-ops directive manifests and scope allow-files.

This helper is intentionally standalone and uses only the Python standard
library. It performs no network calls and does not mutate repository state.
The only write mode is explicit scope-file emission to a caller-supplied
temporary directory.
"""

from __future__ import annotations

import argparse
import fnmatch
import json
import re
import sys
import tempfile
from dataclasses import dataclass, field
from pathlib import Path, PurePosixPath
from typing import Any, Iterable


SCHEMA_VERSION = "qsl.directive_manifest.v1"
EXIT_OK = 0
EXIT_VALIDATION = 2

REQUIRED_MANIFEST_FIELDS = {
    "schema_version",
    "directive_id",
    "target_na",
    "title",
    "expected_origin_main",
    "prior_response_path",
    "mutable_repos",
    "read_only_repos",
    "allowed_paths",
    "forbidden_paths",
    "allowed_local_paths",
    "forbidden_local_paths",
    "required_checks",
    "required_evidence_files",
    "forbidden_operations",
    "public_claim_boundaries",
    "backup_impact_expected",
    "operator_input_required",
    "packet_plan",
    "implementation_paths",
    "temporary_artifact_paths",
    "closeout_successor",
    "stop_conditions",
    "response_file_expected",
    "history_read_only_paths",
}

REQUIRED_FORBIDDEN_OPERATIONS = {
    "admin_bypass",
    "backup_script_mutation",
    "branch_deletion",
    "branch_protection_bypass",
    "dependency_mutation",
    "direct_push",
    "force_push",
    "public_claim_expansion",
    "qsl_bounded_check_poll_mutation",
    "qsl_evidence_helper_mutation",
    "public_safety_gate_mutation",
    "rebase_merge",
    "runtime_mutation",
    "secret_handling",
    "squash_merge",
    "workflow_mutation",
}

REQUIRED_PUBLIC_BOUNDARIES = {
    "no_anonymity_claim",
    "no_external_review_complete_claim",
    "no_metadata_free_claim",
    "no_production_ready_claim",
    "no_public_internet_ready_claim",
    "no_untraceable_claim",
}

REQUIRED_STOP_CONDITIONS = {
    "broad_glob",
    "forbidden_path",
    "manifest_mismatch",
    "missing_public_claim_boundary",
    "parent_traversal",
    "public_safety_red",
    "scope_mismatch",
    "unlisted_path",
}

SECRET_VALUE_PATTERNS = [
    re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----"),
    re.compile(r"\bgh" r"p_[A-Za-z0-9_]{20,}\b"),
    re.compile(r"\bgithub" r"_pat_[A-Za-z0-9_]{20,}\b"),
    re.compile(r"\bs" r"k-[A-Za-z0-9]{20,}\b"),
    re.compile(r"\bAK" r"IA[0-9A-Z]{16}\b"),
    re.compile(r"(?i)\b(password|passphrase|secret|token|credential)\s*=\s*\S+"),
    re.compile(r"\bSECRET_SHAPED_FIXTURE_VALUE\b"),
]

LOCAL_RESPONSE_ROOT = "/home/victor/work/qsl/codex/responses/"
AUTHORIZED_HISTORY_ROOTS = (
    "/home/victor/work/qsl/codex/directives/",
    "/home/victor/work/qsl/codex/responses/",
    "/home/victor/work/qsl/codex/journals/",
    "/home/victor/work/qsl/codex/requests/",
)
TEMP_ROOT = "/srv/qbuild/tmp/"


class ValidationError(RuntimeError):
    """Expected fail-closed validation error."""


@dataclass(frozen=True)
class PathRule:
    path: str
    mode: str
    reason: str = ""
    local: bool = False
    source: str = "manifest"

    def matches(self, changed_path: str) -> bool:
        if self.local:
            return changed_path == self.path
        if self.mode == "exact":
            return changed_path == self.path
        if self.mode == "glob":
            return fnmatch.fnmatch(changed_path, self.path)
        raise ValidationError(f"unsupported path mode: {self.mode}")

    def helper_pattern(self) -> str:
        return self.path


@dataclass
class ManifestValidation:
    manifest_path: str
    manifest: dict[str, Any]
    allowed: list[PathRule] = field(default_factory=list)
    forbidden: list[PathRule] = field(default_factory=list)
    allowed_local: list[PathRule] = field(default_factory=list)
    forbidden_local: list[PathRule] = field(default_factory=list)
    markers: list[str] = field(default_factory=list)


@dataclass
class AllowFileValidation:
    allow_file_path: str
    allowed: list[PathRule] = field(default_factory=list)
    forbidden: list[PathRule] = field(default_factory=list)
    local: list[PathRule] = field(default_factory=list)
    comments: int = 0
    blanks: int = 0
    markers: list[str] = field(default_factory=list)


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ValidationError(f"malformed JSON in {path}: line {exc.lineno} column {exc.colno}") from exc
    except OSError as exc:
        raise ValidationError(f"cannot read {path}: {exc}") from exc


def ensure_mapping(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ValidationError(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise ValidationError(f"{label} must be a list")
    return value


def ensure_string(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise ValidationError(f"{label} must be a non-empty string")
    return value


def check_no_secret_values(value: Any, path: str = "$") -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            check_no_secret_values(child, f"{path}.{key}")
        return
    if isinstance(value, list):
        for index, child in enumerate(value):
            check_no_secret_values(child, f"{path}[{index}]")
        return
    if not isinstance(value, str):
        return
    for pattern in SECRET_VALUE_PATTERNS:
        if pattern.search(value):
            raise ValidationError(f"secret-shaped value rejected at {path}")


def reject_path_ambiguity(raw: str, *, allow_glob: bool, local: bool) -> None:
    if "\x00" in raw:
        raise ValidationError("path contains NUL byte")
    if "\\" in raw:
        raise ValidationError(f"path uses backslash separators: {raw}")
    if raw != raw.strip():
        raise ValidationError(f"path has leading/trailing whitespace: {raw!r}")
    if "//" in raw or raw.startswith("./") or "/./" in raw or raw.endswith("/."):
        raise ValidationError(f"path normalization ambiguity rejected: {raw}")
    if raw.startswith("~"):
        raise ValidationError(f"home-relative path rejected: {raw}")
    if not local and raw.startswith("/"):
        raise ValidationError(f"absolute repo path rejected: {raw}")
    path_for_parts = raw[1:] if local and raw.startswith("/") else raw
    parts = path_for_parts.split("/")
    if any(part in {"", ".", ".."} for part in parts):
        raise ValidationError(f"parent traversal or empty path segment rejected: {raw}")
    if any(part == ".git" for part in parts):
        raise ValidationError(f".git path rejected: {raw}")
    has_glob = any(ch in raw for ch in "*?[")
    if has_glob and not allow_glob:
        raise ValidationError(f"wildcard requires glob mode: {raw}")
    if local:
        if not raw.startswith("/"):
            raise ValidationError(f"local path must be absolute: {raw}")
        if "/../" in raw or raw.endswith("/.."):
            raise ValidationError(f"local parent traversal rejected: {raw}")


def validate_repo_path(raw: str, *, allow_glob: bool = False, for_forbidden: bool = False) -> str:
    reject_path_ambiguity(raw, allow_glob=allow_glob, local=False)
    if allow_glob:
        validate_glob(raw, for_forbidden=for_forbidden)
    else:
        PurePosixPath(raw)
    return raw


def validate_local_path(raw: str, *, allow_glob: bool = False, for_forbidden: bool = False) -> str:
    reject_path_ambiguity(raw, allow_glob=allow_glob, local=True)
    if allow_glob:
        validate_local_glob(raw, for_forbidden=for_forbidden)
    return raw


def validate_glob(pattern: str, *, for_forbidden: bool) -> None:
    if not any(ch in pattern for ch in "*?["):
        return
    if pattern in {"*", "**", "**/*", "*/**", "*/*"}:
        raise ValidationError(f"broad glob rejected: {pattern}")
    parts = pattern.split("/")
    if "**" in parts:
        if for_forbidden and pattern.endswith("/**") and len(parts) >= 2 and not any("*" in p or "?" in p or "[" in p for p in parts[:-1]):
            return
        raise ValidationError(f"recursive broad glob rejected: {pattern}")
    first_wild = next((i for i, part in enumerate(parts) if any(ch in part for ch in "*?[")), None)
    if first_wild is None:
        return
    if first_wild < 2:
        raise ValidationError(f"glob must be scoped below a fixed directory prefix: {pattern}")
    if parts[first_wild] in {"*", "*.*"} and first_wild < len(parts) - 1:
        raise ValidationError(f"directory-wide wildcard glob rejected: {pattern}")


def validate_local_glob(pattern: str, *, for_forbidden: bool) -> None:
    if not any(ch in pattern for ch in "*?["):
        return
    if not for_forbidden:
        raise ValidationError(f"local glob is not allowed for mutable local paths: {pattern}")
    if "**" in pattern and not pattern.endswith("/**"):
        raise ValidationError(f"unsafe local glob rejected: {pattern}")


def parse_path_rule(value: Any, *, label: str, local: bool, for_forbidden: bool) -> PathRule:
    if isinstance(value, str):
        path = value
        mode = "exact"
        reason = ""
    else:
        item = ensure_mapping(value, label)
        allowed_keys = {"path", "mode", "reason"}
        unknown = sorted(set(item) - allowed_keys)
        if unknown:
            raise ValidationError(f"{label} has unknown keys: {', '.join(unknown)}")
        path = ensure_string(item.get("path"), f"{label}.path")
        mode = ensure_string(item.get("mode", "exact"), f"{label}.mode")
        reason = str(item.get("reason", ""))
    if mode not in {"exact", "glob", "local_exact", "local_glob"}:
        raise ValidationError(f"{label}.mode unsupported: {mode}")
    if local:
        if mode == "local_exact":
            mode = "exact"
        if mode == "local_glob":
            mode = "glob"
        if mode not in {"exact", "glob"}:
            raise ValidationError(f"{label}.mode must be local_exact/local_glob for local paths")
        normalized = validate_local_path(path, allow_glob=(mode == "glob"), for_forbidden=for_forbidden)
    else:
        if mode.startswith("local_"):
            raise ValidationError(f"{label}.mode local mode used for repo path")
        normalized = validate_repo_path(path, allow_glob=(mode == "glob"), for_forbidden=for_forbidden)
    return PathRule(path=normalized, mode=mode, reason=reason, local=local)


def parse_path_rules(values: Any, *, label: str, local: bool, for_forbidden: bool) -> list[PathRule]:
    rules: list[PathRule] = []
    for index, value in enumerate(ensure_list(values, label)):
        rule = parse_path_rule(value, label=f"{label}[{index}]", local=local, for_forbidden=for_forbidden)
        rules.append(rule)
    return rules


def require_subset(actual: Iterable[str], required: set[str], label: str) -> None:
    actual_set = {str(item) for item in actual}
    missing = sorted(required - actual_set)
    if missing:
        raise ValidationError(f"{label} missing required values: {', '.join(missing)}")


def validate_manifest(
    manifest_path: Path,
    *,
    expect_directive_id: str | None = None,
    expect_target_na: str | None = None,
    expect_origin_main: str | None = None,
    expect_successor: str | None = None,
    expect_operator_input_required: bool | None = None,
) -> ManifestValidation:
    data = ensure_mapping(load_json(manifest_path), "manifest")
    unknown = sorted(set(data) - REQUIRED_MANIFEST_FIELDS)
    if unknown:
        raise ValidationError(f"manifest has unknown top-level keys: {', '.join(unknown)}")
    missing = sorted(REQUIRED_MANIFEST_FIELDS - set(data))
    if missing:
        raise ValidationError(f"manifest missing required fields: {', '.join(missing)}")
    check_no_secret_values(data)
    if data["schema_version"] != SCHEMA_VERSION:
        raise ValidationError(f"schema_version must be {SCHEMA_VERSION}")
    directive_id = ensure_string(data["directive_id"], "directive_id")
    target_na = ensure_string(data["target_na"], "target_na")
    expected_main = ensure_string(data["expected_origin_main"], "expected_origin_main")
    successor = ensure_string(data["closeout_successor"], "closeout_successor")
    if expect_directive_id and directive_id != expect_directive_id:
        raise ValidationError(f"directive_id mismatch: {directive_id} != {expect_directive_id}")
    if expect_target_na and target_na != expect_target_na:
        raise ValidationError(f"target_na mismatch: {target_na} != {expect_target_na}")
    if expect_origin_main and expected_main != expect_origin_main:
        raise ValidationError("expected_origin_main mismatch")
    if expect_successor and successor != expect_successor:
        raise ValidationError("closeout_successor mismatch")
    if expect_operator_input_required is not None and data["operator_input_required"] is not expect_operator_input_required:
        raise ValidationError("operator_input_required mismatch")
    if not re.fullmatch(r"[0-9a-f]{40}", expected_main):
        raise ValidationError("expected_origin_main must be a full lowercase git SHA")
    prior_response = ensure_string(data["prior_response_path"], "prior_response_path")
    if not prior_response.startswith(LOCAL_RESPONSE_ROOT) or not prior_response.endswith("_D200.md"):
        raise ValidationError("prior_response_path outside expected D200 response archive boundary")
    response_expected = ensure_string(data["response_file_expected"], "response_file_expected")
    if not response_expected.startswith(LOCAL_RESPONSE_ROOT) or not response_expected.endswith("_D201.md"):
        raise ValidationError("response_file_expected outside expected D201 response archive boundary")
    mutable_repos = [ensure_string(item, "mutable_repos[]") for item in ensure_list(data["mutable_repos"], "mutable_repos")]
    if mutable_repos != ["qsl-protocol"]:
        raise ValidationError("mutable_repos must be exactly ['qsl-protocol']")
    read_only_repos = {ensure_string(item, "read_only_repos[]") for item in ensure_list(data["read_only_repos"], "read_only_repos")}
    if not {"qsl-server", "qsl-attachments"}.issubset(read_only_repos):
        raise ValidationError("read_only_repos must include qsl-server and qsl-attachments")
    required_checks = [ensure_string(item, "required_checks[]") for item in ensure_list(data["required_checks"], "required_checks")]
    if "public-safety" not in required_checks:
        raise ValidationError("required_checks must include public-safety")
    required_evidence = [ensure_string(item, "required_evidence_files[]") for item in ensure_list(data["required_evidence_files"], "required_evidence_files")]
    for item in required_evidence:
        validate_repo_path(item)
    forbidden_operations = [ensure_string(item, "forbidden_operations[]") for item in ensure_list(data["forbidden_operations"], "forbidden_operations")]
    require_subset(forbidden_operations, REQUIRED_FORBIDDEN_OPERATIONS, "forbidden_operations")
    public_boundaries = [ensure_string(item, "public_claim_boundaries[]") for item in ensure_list(data["public_claim_boundaries"], "public_claim_boundaries")]
    require_subset(public_boundaries, REQUIRED_PUBLIC_BOUNDARIES, "public_claim_boundaries")
    stop_conditions = [ensure_string(item, "stop_conditions[]") for item in ensure_list(data["stop_conditions"], "stop_conditions")]
    require_subset(stop_conditions, REQUIRED_STOP_CONDITIONS, "stop_conditions")
    if data["backup_impact_expected"] != "no_backup_plan_update_required":
        raise ValidationError("backup_impact_expected must be no_backup_plan_update_required")
    if not isinstance(data["operator_input_required"], bool):
        raise ValidationError("operator_input_required must be boolean")
    for label in ("packet_plan", "implementation_paths"):
        for item in ensure_list(data[label], label):
            if label == "implementation_paths":
                validate_repo_path(ensure_string(item, f"{label}[]"))
            else:
                ensure_string(item, f"{label}[]")
    for item in ensure_list(data["temporary_artifact_paths"], "temporary_artifact_paths"):
        temp_path = ensure_string(item, "temporary_artifact_paths[]")
        if not temp_path.startswith(TEMP_ROOT) or "/../" in temp_path:
            raise ValidationError("temporary_artifact_paths must stay under /srv/qbuild/tmp")
    for item in ensure_list(data["history_read_only_paths"], "history_read_only_paths"):
        history_path = ensure_string(item, "history_read_only_paths[]")
        if not history_path.startswith(AUTHORIZED_HISTORY_ROOTS):
            raise ValidationError(f"history_read_only_paths entry outside authorized roots: {history_path}")
    allowed = parse_path_rules(data["allowed_paths"], label="allowed_paths", local=False, for_forbidden=False)
    forbidden = parse_path_rules(data["forbidden_paths"], label="forbidden_paths", local=False, for_forbidden=True)
    allowed_local = parse_path_rules(data["allowed_local_paths"], label="allowed_local_paths", local=True, for_forbidden=False)
    forbidden_local = parse_path_rules(data["forbidden_local_paths"], label="forbidden_local_paths", local=True, for_forbidden=True)
    if not allowed:
        raise ValidationError("allowed_paths must not be empty")
    return ManifestValidation(
        manifest_path=str(manifest_path),
        manifest=data,
        allowed=allowed,
        forbidden=forbidden,
        allowed_local=allowed_local,
        forbidden_local=forbidden_local,
        markers=["NA0382_MANIFEST_VALIDATION_OK"],
    )


def parse_allow_file(path: Path, *, manifest: ManifestValidation | None = None) -> AllowFileValidation:
    allowed: list[PathRule] = []
    forbidden: list[PathRule] = []
    local_rules: list[PathRule] = []
    comments = 0
    blanks = 0
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as exc:
        raise ValidationError(f"cannot read allow-file {path}: {exc}") from exc
    check_no_secret_values("\n".join(lines), "allow_file")
    for line_no, raw in enumerate(lines, start=1):
        stripped = raw.strip()
        if not stripped:
            blanks += 1
            continue
        if stripped.startswith("#"):
            comments += 1
            continue
        if ":" in stripped:
            prefix, body = stripped.split(":", 1)
            if prefix == "glob":
                repo_path = validate_repo_path(body, allow_glob=True, for_forbidden=False)
                allowed.append(PathRule(repo_path, "glob", source=f"{path}:{line_no}"))
                continue
            if prefix == "local":
                local_path = validate_local_path(body, allow_glob=False, for_forbidden=False)
                local_rule = PathRule(local_path, "exact", local=True, source=f"{path}:{line_no}")
                if manifest is None or not any(rule.path == local_rule.path for rule in manifest.allowed_local):
                    raise ValidationError(f"local path not authorized by manifest at {path}:{line_no}")
                local_rules.append(local_rule)
                continue
            raise ValidationError(f"malformed allow-file line {path}:{line_no}: unknown prefix {prefix}")
        repo_path = validate_repo_path(stripped, allow_glob=False, for_forbidden=False)
        allowed.append(PathRule(repo_path, "exact", source=f"{path}:{line_no}"))
    if not allowed and not local_rules:
        raise ValidationError("allow-file has no allowed entries")
    return AllowFileValidation(
        allow_file_path=str(path),
        allowed=allowed,
        forbidden=forbidden,
        local=local_rules,
        comments=comments,
        blanks=blanks,
        markers=["NA0382_ALLOW_FILE_VALIDATION_OK"],
    )


def load_changed_paths(path: Path) -> list[str]:
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as exc:
        raise ValidationError(f"cannot read changed paths {path}: {exc}") from exc
    changed: list[str] = []
    for line_no, raw in enumerate(lines, start=1):
        stripped = raw.strip()
        if not stripped or stripped.startswith("#"):
            continue
        changed.append(validate_repo_path(stripped, allow_glob=False))
    if not changed:
        raise ValidationError("changed-paths file has no paths")
    return changed


def any_match(path: str, rules: Iterable[PathRule]) -> bool:
    return any(rule.matches(path) for rule in rules)


def validate_changed_paths(manifest: ManifestValidation, allow_file: AllowFileValidation, changed_paths: list[str]) -> dict[str, Any]:
    allowed_rules = list(manifest.allowed) + list(allow_file.allowed)
    forbidden_rules = list(manifest.forbidden) + list(allow_file.forbidden)
    path_results: list[dict[str, Any]] = []
    failures: list[str] = []
    for path in changed_paths:
        forbidden = any_match(path, forbidden_rules)
        allowed = any_match(path, allowed_rules)
        if forbidden:
            classification = "forbidden"
            failures.append(path)
        elif not allowed:
            classification = "unlisted"
            failures.append(path)
        else:
            classification = "allowed"
        path_results.append({"path": path, "classification": classification})
    if failures:
        raise ValidationError("changed paths rejected: " + ", ".join(failures))
    return {"changed_paths": path_results, "changed_path_count": len(changed_paths)}


def manifest_summary(result: ManifestValidation) -> dict[str, Any]:
    return {
        "kind": "manifest",
        "manifest": result.manifest_path,
        "schema_version": result.manifest["schema_version"],
        "directive_id": result.manifest["directive_id"],
        "target_na": result.manifest["target_na"],
        "expected_origin_main": result.manifest["expected_origin_main"],
        "closeout_successor": result.manifest["closeout_successor"],
        "allowed_path_count": len(result.allowed),
        "forbidden_path_count": len(result.forbidden),
        "allowed_local_path_count": len(result.allowed_local),
        "markers": result.markers,
    }


def allow_summary(result: AllowFileValidation) -> dict[str, Any]:
    return {
        "kind": "allow-file",
        "allow_file": result.allow_file_path,
        "allowed_entry_count": len(result.allowed),
        "local_entry_count": len(result.local),
        "comment_count": result.comments,
        "blank_count": result.blanks,
        "markers": result.markers,
    }


def emit_summary(summary: dict[str, Any], *, json_output: bool) -> None:
    if json_output:
        print(json.dumps(summary, indent=2, sort_keys=True))
        return
    status = summary.get("status", "ok").upper()
    kind = summary.get("kind", "validation")
    print(f"RESULT {status} {kind}")
    for key in sorted(k for k in summary if k not in {"markers", "errors", "changed_paths"}):
        print(f"{key.upper()} {summary[key]}")
    for item in summary.get("changed_paths", []):
        print(f"PATH {item['classification']} {item['path']}")
    for marker in summary.get("markers", []):
        print(marker)


def expectation_kwargs(args: argparse.Namespace) -> dict[str, Any]:
    return {
        "expect_directive_id": getattr(args, "expect_directive_id", None),
        "expect_target_na": getattr(args, "expect_target_na", None),
        "expect_origin_main": getattr(args, "expect_origin_main", None),
        "expect_successor": getattr(args, "expect_successor", None),
        "expect_operator_input_required": getattr(args, "expect_operator_input_required", None),
    }


def command_validate_manifest(args: argparse.Namespace) -> int:
    result = validate_manifest(Path(args.manifest), **expectation_kwargs(args))
    summary = manifest_summary(result)
    summary["status"] = "ok"
    emit_summary(summary, json_output=args.json)
    return EXIT_OK


def command_validate_allow_file(args: argparse.Namespace) -> int:
    manifest = validate_manifest(Path(args.manifest), **expectation_kwargs(args)) if args.manifest else None
    result = parse_allow_file(Path(args.allow_file), manifest=manifest)
    summary = allow_summary(result)
    summary["status"] = "ok"
    emit_summary(summary, json_output=args.json)
    return EXIT_OK


def command_validate(args: argparse.Namespace) -> int:
    manifest = validate_manifest(Path(args.manifest), **expectation_kwargs(args))
    allow_file = parse_allow_file(Path(args.allow_file), manifest=manifest)
    changed_paths = load_changed_paths(Path(args.changed_paths_file))
    changed_summary = validate_changed_paths(manifest, allow_file, changed_paths)
    summary = {
        "kind": "manifest-allow-file-changed-paths",
        "status": "ok",
        "manifest": args.manifest,
        "allow_file": args.allow_file,
        "changed_paths_file": args.changed_paths_file,
        "markers": sorted(set(manifest.markers + allow_file.markers)),
    }
    summary.update(changed_summary)
    emit_summary(summary, json_output=args.json)
    return EXIT_OK


def safe_out_dir(path: Path) -> Path:
    resolved = path.resolve()
    temp_root = Path(TEMP_ROOT).resolve()
    try:
        resolved.relative_to(temp_root)
    except ValueError as exc:
        raise ValidationError(f"out-dir must be under {TEMP_ROOT}") from exc
    return resolved


def command_emit_scope_files(args: argparse.Namespace) -> int:
    manifest = validate_manifest(Path(args.manifest), **expectation_kwargs(args))
    allow_file = parse_allow_file(Path(args.allow_file), manifest=manifest)
    out_dir = safe_out_dir(Path(args.out_dir))
    out_dir.mkdir(parents=True, exist_ok=True)
    allowed_path = out_dir / "allowed_paths.txt"
    forbidden_path = out_dir / "forbidden_paths.txt"
    summary_path = out_dir / "scope_summary.json"
    allowed_patterns = [rule.helper_pattern() for rule in manifest.allowed + allow_file.allowed]
    forbidden_patterns = [rule.helper_pattern() for rule in manifest.forbidden + allow_file.forbidden]
    allowed_path.write_text("\n".join(allowed_patterns) + "\n", encoding="utf-8")
    forbidden_path.write_text("\n".join(forbidden_patterns) + "\n", encoding="utf-8")
    summary = {
        "kind": "emit-scope-files",
        "status": "ok",
        "out_dir": str(out_dir),
        "allowed_file": str(allowed_path),
        "forbidden_file": str(forbidden_path),
        "summary_file": str(summary_path),
        "allowed_count": len(allowed_patterns),
        "forbidden_count": len(forbidden_patterns),
        "markers": ["NA0382_MANIFEST_VALIDATION_OK", "NA0382_ALLOW_FILE_VALIDATION_OK"],
    }
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    emit_summary(summary, json_output=args.json)
    return EXIT_OK


def run_fixture_case(case: dict[str, Any], fixture_dir: Path, allow_fixture_dir: Path) -> tuple[bool, str, list[str]]:
    kind = ensure_string(case.get("kind"), "case.kind")
    name = ensure_string(case.get("name"), "case.name")
    expected_exit = int(case.get("expect_exit", 0))
    markers = [str(marker) for marker in case.get("markers", [])]
    expectations = dict(case.get("expect", {}))
    try:
        if kind == "manifest":
            validate_manifest(fixture_dir / ensure_string(case.get("manifest"), "case.manifest"), **expectations)
        elif kind == "allow-file":
            manifest = None
            if case.get("manifest"):
                manifest = validate_manifest(fixture_dir / ensure_string(case.get("manifest"), "case.manifest"), **expectations)
            parse_allow_file(allow_fixture_dir / ensure_string(case.get("allow_file"), "case.allow_file"), manifest=manifest)
        elif kind == "validate":
            manifest = validate_manifest(fixture_dir / ensure_string(case.get("manifest"), "case.manifest"), **expectations)
            allow_file = parse_allow_file(allow_fixture_dir / ensure_string(case.get("allow_file"), "case.allow_file"), manifest=manifest)
            changed = load_changed_paths(allow_fixture_dir / ensure_string(case.get("changed_paths"), "case.changed_paths"))
            validate_changed_paths(manifest, allow_file, changed)
        elif kind == "emit-scope-files":
            manifest = validate_manifest(fixture_dir / ensure_string(case.get("manifest"), "case.manifest"), **expectations)
            allow_file = parse_allow_file(allow_fixture_dir / ensure_string(case.get("allow_file"), "case.allow_file"), manifest=manifest)
            out_dir = Path(tempfile.mkdtemp(prefix="NA0382_fixture_emit_", dir=TEMP_ROOT))
            allowed_path = out_dir / "allowed_paths.txt"
            forbidden_path = out_dir / "forbidden_paths.txt"
            allowed_path.write_text("\n".join(rule.helper_pattern() for rule in manifest.allowed + allow_file.allowed) + "\n", encoding="utf-8")
            forbidden_path.write_text("\n".join(rule.helper_pattern() for rule in manifest.forbidden + allow_file.forbidden) + "\n", encoding="utf-8")
            if not allowed_path.exists() or not forbidden_path.exists():
                raise ValidationError("emit-scope-files did not create expected files")
        else:
            raise ValidationError(f"unknown fixture case kind: {kind}")
        actual_exit = 0
        detail = "ok"
    except ValidationError as exc:
        actual_exit = EXIT_VALIDATION
        detail = str(exc)
    passed = actual_exit == expected_exit
    return passed, f"{name}: expected={expected_exit} actual={actual_exit} detail={detail}", markers if passed else []


def command_fixture(args: argparse.Namespace) -> int:
    fixture_dir = Path(args.fixture_dir)
    allow_fixture_dir = Path(args.allow_fixture_dir)
    matrix = ensure_mapping(load_json(fixture_dir / "fixture_cases.json"), "fixture_cases")
    cases = ensure_list(matrix.get("cases"), "cases")
    passed = 0
    failed = 0
    emitted_markers: list[str] = []
    records: list[dict[str, Any]] = []
    for raw_case in cases:
        case = ensure_mapping(raw_case, "case")
        ok, detail, markers = run_fixture_case(case, fixture_dir, allow_fixture_dir)
        if ok:
            passed += 1
            emitted_markers.extend(markers)
        else:
            failed += 1
        records.append({"name": case.get("name"), "passed": ok, "detail": detail, "markers": markers})
    summary = {
        "kind": "fixture",
        "status": "ok" if failed == 0 else "fail",
        "case_count": len(cases),
        "passed": passed,
        "failed": failed,
        "markers": sorted(set(emitted_markers)),
    }
    if args.json:
        summary["cases"] = records
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(f"FIXTURE_CASE_COUNT {len(cases)}")
        print(f"FIXTURE_PASS_COUNT {passed}")
        print(f"FIXTURE_FAIL_COUNT {failed}")
        for record in records:
            state = "PASS" if record["passed"] else "FAIL"
            print(f"CASE {state} {record['detail']}")
        for marker in summary["markers"]:
            print(marker)
    return EXIT_OK if failed == 0 else EXIT_VALIDATION


def add_expectation_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--expect-directive-id")
    parser.add_argument("--expect-target-na")
    parser.add_argument("--expect-origin-main")
    parser.add_argument("--expect-successor")
    parser.add_argument("--expect-operator-input-required", action=argparse.BooleanOptionalAction, default=None)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Validate QSL directive manifest and scope allow-file fixtures.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    manifest = subparsers.add_parser("validate-manifest", help="Validate one directive manifest JSON file.")
    manifest.add_argument("--manifest", required=True)
    manifest.add_argument("--repo-root", default=".")
    manifest.add_argument("--json", action="store_true")
    add_expectation_args(manifest)
    manifest.set_defaults(func=command_validate_manifest)

    allow_file = subparsers.add_parser("validate-allow-file", help="Validate one scope allow-file.")
    allow_file.add_argument("--allow-file", required=True)
    allow_file.add_argument("--manifest")
    allow_file.add_argument("--repo-root", default=".")
    allow_file.add_argument("--json", action="store_true")
    add_expectation_args(allow_file)
    allow_file.set_defaults(func=command_validate_allow_file)

    validate = subparsers.add_parser("validate", help="Validate manifest, allow-file, and changed paths.")
    validate.add_argument("--manifest", required=True)
    validate.add_argument("--allow-file", required=True)
    validate.add_argument("--changed-paths-file", required=True)
    validate.add_argument("--repo-root", default=".")
    validate.add_argument("--json", action="store_true")
    add_expectation_args(validate)
    validate.set_defaults(func=command_validate)

    emit = subparsers.add_parser("emit-scope-files", help="Emit helper-compatible scope files under a temp directory.")
    emit.add_argument("--manifest", required=True)
    emit.add_argument("--allow-file", required=True)
    emit.add_argument("--out-dir", required=True)
    emit.add_argument("--json", action="store_true")
    add_expectation_args(emit)
    emit.set_defaults(func=command_emit_scope_files)

    fixture = subparsers.add_parser("fixture", help="Run the no-network fixture matrix.")
    fixture.add_argument("--fixture-dir", required=True)
    fixture.add_argument("--allow-fixture-dir", required=True)
    fixture.add_argument("--json", action="store_true")
    fixture.set_defaults(func=command_fixture)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return args.func(args)
    except ValidationError as exc:
        summary = {"kind": getattr(args, "command", "validation"), "status": "fail", "errors": [str(exc)]}
        if getattr(args, "json", False):
            print(json.dumps(summary, indent=2, sort_keys=True))
        else:
            print(f"RESULT FAIL {summary['kind']}")
            print(f"ERROR {exc}")
        return EXIT_VALIDATION


if __name__ == "__main__":
    sys.exit(main())
