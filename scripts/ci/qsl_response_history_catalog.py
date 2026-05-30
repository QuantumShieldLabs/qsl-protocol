#!/usr/bin/env python3
"""QSL local response/history metadata catalog harness.

This helper is intentionally local-only and standard-library-only. It scans
explicitly supplied allow-listed roots read-only, writes metadata-only catalog
artifacts under /srv/qbuild/tmp, and never writes into the scanned history roots.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


SCHEMA_VERSION = "qsl.response_history_catalog.v1"
SUMMARY_SCHEMA_VERSION = "qsl.response_history_catalog.summary.v1"
FIXTURE_SCHEMA_VERSION = "qsl.response_history_catalog.fixture_matrix.v1"
TEMP_ROOT = Path("/srv/qbuild/tmp")
MAX_TEXT_SCAN_BYTES = 64 * 1024
EXIT_OK = 0
EXIT_INTERNAL = 1
EXIT_VALIDATION = 2

ROOT_LABELS = ("responses", "requests", "directives", "journals", "ops")
LIVE_ROOTS = {
    "responses": Path("/home/victor/work/qsl/codex/responses"),
    "requests": Path("/home/victor/work/qsl/codex/requests"),
    "directives": Path("/home/victor/work/qsl/codex/directives"),
    "journals": Path("/home/victor/work/qsl/codex/journals"),
    "ops": Path("/home/victor/work/qsl/codex/ops"),
}

SECRET_PATTERNS = [
    ("private_key_block", re.compile(r"-----BEGIN (?:[A-Z0-9]+ )?PRIVATE KEY-----")),
    ("github_token", re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{30,}\b")),
    ("github_pat", re.compile(r"\bgithub_pat_[A-Za-z0-9_]{30,}\b")),
    ("aws_access_key_id", re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b")),
    ("openai_key", re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b")),
    ("jwt", re.compile(r"\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}\b")),
    ("qsl_test_secret_sentinel", re.compile(r"\bQSL_TEST_FORBIDDEN_SECRET_SENTINEL\b")),
    ("secret_shaped_fixture_value", re.compile(r"\bSECRET_SHAPED_FIXTURE_VALUE\b")),
    ("recovery_envelope_marker", re.compile(r"(?i)\bBEGIN QSL RECOVERY ENVELOPE\b")),
]

FORBIDDEN_BODY_KEYS = {
    "body",
    "body_text",
    "content",
    "contents",
    "full_body",
    "full_body_text",
    "raw_body",
    "raw_content",
    "raw_text",
    "response_body",
    "text",
}


class CatalogError(RuntimeError):
    """Expected fail-closed helper error."""


@dataclass(frozen=True)
class RootSpec:
    label: str
    path: Path
    resolved: Path


@dataclass
class FileEntry:
    source_root_label: str
    relative_path: str
    classification: str
    size: int | None = None
    mtime_utc: str | None = None
    sha256: str | None = None
    filename_inferred_target_na: str | None = None
    filename_inferred_directive_suffix: str | None = None
    filename_inferred_timestamp: str | None = None
    response_wrapper_present: bool | None = None
    synthetic_smoke_marker: bool = False
    bounded_headers: dict[str, str] = field(default_factory=dict)
    extraction_status: str = "not_started"
    secret_scan_status: str = "not_started"
    errors: list[str] = field(default_factory=list)

    def as_dict(self) -> dict[str, Any]:
        return {
            "source_root_label": self.source_root_label,
            "relative_path": self.relative_path,
            "classification": self.classification,
            "size": self.size,
            "mtime_utc": self.mtime_utc,
            "sha256": self.sha256,
            "filename_inferred_target_na": self.filename_inferred_target_na,
            "filename_inferred_directive_suffix": self.filename_inferred_directive_suffix,
            "filename_inferred_timestamp": self.filename_inferred_timestamp,
            "response_wrapper_present": self.response_wrapper_present,
            "synthetic_smoke_marker": self.synthetic_smoke_marker,
            "bounded_headers": dict(sorted(self.bounded_headers.items())),
            "extraction_status": self.extraction_status,
            "secret_scan_status": self.secret_scan_status,
            "errors": sorted(set(self.errors)),
        }


def utc_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def is_relative_to(path: Path, parent: Path) -> bool:
    try:
        path.relative_to(parent)
    except ValueError:
        return False
    return True


def reject_ambiguous_path(raw: str, *, allow_absolute: bool) -> None:
    if "\x00" in raw:
        raise CatalogError("path contains NUL byte")
    if "\\" in raw:
        raise CatalogError(f"path uses backslash separators: {raw}")
    if raw != raw.strip():
        raise CatalogError(f"path has leading/trailing whitespace: {raw!r}")
    if raw.startswith("~"):
        raise CatalogError(f"home-relative path rejected: {raw}")
    if not allow_absolute and raw.startswith("/"):
        raise CatalogError(f"absolute path rejected: {raw}")
    parts = Path(raw).parts
    if any(part in {"..", ""} for part in parts):
        raise CatalogError(f"parent traversal or empty path segment rejected: {raw}")


def validate_temp_output_dir(raw: str) -> Path:
    reject_ambiguous_path(raw, allow_absolute=True)
    out_dir = Path(raw)
    resolved = out_dir.resolve(strict=False)
    temp_root = TEMP_ROOT.resolve(strict=True)
    if resolved == temp_root or not is_relative_to(resolved, temp_root):
        raise CatalogError("output directory must be a child of /srv/qbuild/tmp")
    resolved.mkdir(parents=True, exist_ok=True)
    if resolved.is_symlink():
        raise CatalogError("output directory symlink rejected")
    return resolved


def parse_root_spec(raw: str, *, mode: str, fixture_dir: Path | None = None) -> RootSpec:
    if "=" not in raw:
        raise CatalogError(f"root must use label=path form: {raw}")
    label, raw_path = raw.split("=", 1)
    label = label.strip()
    if label not in ROOT_LABELS:
        raise CatalogError(f"unknown root label rejected: {label}")
    reject_ambiguous_path(raw_path, allow_absolute=True)
    path = Path(raw_path)
    resolved = path.resolve(strict=False)
    if mode == "live":
        expected = LIVE_ROOTS[label].resolve(strict=False)
    elif mode == "fixture":
        if fixture_dir is None:
            raise CatalogError("fixture_dir is required for fixture root validation")
        expected = (fixture_dir / label).resolve(strict=False)
    else:
        raise CatalogError(f"unknown root validation mode: {mode}")
    if resolved != expected:
        raise CatalogError(f"root outside allowed path rejected for {label}")
    return RootSpec(label=label, path=path, resolved=resolved)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def mtime_utc(path: Path) -> str:
    return datetime.fromtimestamp(path.stat().st_mtime, tz=timezone.utc).replace(microsecond=0).isoformat()


def infer_from_filename(name: str) -> tuple[str | None, str | None, str | None]:
    target = None
    suffix = None
    timestamp = None
    target_match = re.search(r"(?<![A-Za-z0-9])NA-?(\d{4})(?!\d)", name)
    if target_match:
        target = f"NA-{target_match.group(1)}"
    suffix_match = re.search(r"(?:^|[_-])D(\d{3})(?:\D|$)", name)
    if suffix_match:
        suffix = f"D{suffix_match.group(1)}"
    timestamp_match = re.search(r"\d{8}T\d{6}[+-]\d{4}", name)
    if timestamp_match:
        timestamp = timestamp_match.group(0)
    return target, suffix, timestamp


def classify(label: str) -> str:
    if label == "responses":
        return "response"
    if label == "requests":
        return "request"
    if label == "directives":
        return "directive"
    if label == "journals":
        return "journal"
    return "ops"


def safe_decode_sample(path: Path) -> tuple[str | None, str]:
    with path.open("rb") as handle:
        sample = handle.read(MAX_TEXT_SCAN_BYTES + 1)
    bounded = sample[:MAX_TEXT_SCAN_BYTES]
    if b"\x00" in bounded:
        return None, "binary_or_non_utf8"
    try:
        text = bounded.decode("utf-8")
    except UnicodeDecodeError:
        return None, "binary_or_non_utf8"
    if len(sample) > MAX_TEXT_SCAN_BYTES:
        return text, "utf8_bounded_truncated"
    return text, "utf8_bounded"


def secret_scan(text: str | None, decode_status: str) -> tuple[str, str | None]:
    if text is None:
        return "skipped_binary_or_non_utf8", None
    for pattern_id, pattern in SECRET_PATTERNS:
        if pattern.search(text):
            return "secret_sentinel_rejected", pattern_id
    if decode_status == "utf8_bounded_truncated":
        return "bounded_clean_truncated", None
    return "bounded_clean", None


def extract_bounded_headers(text: str) -> dict[str, str]:
    headers: dict[str, str] = {}
    allowed = {
        "Response start timestamp (America/Chicago)": "response_start_chicago",
        "Response start timestamp (UTC)": "response_start_utc",
        "Directive begin timestamp (America/Chicago)": "directive_begin_chicago",
        "Directive begin timestamp (UTC)": "directive_begin_utc",
        "Directive ID": "directive_id",
    }
    for raw_line in text.splitlines()[:80]:
        if ":" not in raw_line:
            continue
        key, value = raw_line.split(":", 1)
        normalized = allowed.get(key.strip())
        if normalized and value.strip():
            headers[normalized] = value.strip()[:160]
    return headers


def entry_for_symlink(root: RootSpec, candidate: Path) -> FileEntry:
    rel = candidate.relative_to(root.resolved).as_posix()
    entry = FileEntry(
        source_root_label=root.label,
        relative_path=rel,
        classification=f"{classify(root.label)}_symlink_rejected",
        extraction_status="symlink_rejected",
        secret_scan_status="not_scanned_symlink",
        errors=["symlink_rejected"],
    )
    target, suffix, timestamp = infer_from_filename(candidate.name)
    entry.filename_inferred_target_na = target
    entry.filename_inferred_directive_suffix = suffix
    entry.filename_inferred_timestamp = timestamp
    return entry


def scan_file(root: RootSpec, candidate: Path) -> FileEntry:
    rel = candidate.resolve(strict=True).relative_to(root.resolved).as_posix()
    entry = FileEntry(source_root_label=root.label, relative_path=rel, classification=classify(root.label))
    target, suffix, timestamp = infer_from_filename(candidate.name)
    entry.filename_inferred_target_na = target
    entry.filename_inferred_directive_suffix = suffix
    entry.filename_inferred_timestamp = timestamp
    entry.size = candidate.stat().st_size
    entry.mtime_utc = mtime_utc(candidate)
    entry.sha256 = sha256_file(candidate)

    text, decode_status = safe_decode_sample(candidate)
    entry.secret_scan_status, pattern_id = secret_scan(text, decode_status)
    if pattern_id:
        entry.errors.append(f"secret_sentinel_rejected:{pattern_id}")
        entry.extraction_status = "secret_sentinel_rejected"
        return entry
    if text is None:
        entry.errors.append("binary_or_non_utf8")
        entry.extraction_status = "binary_or_non_utf8"
        return entry

    entry.bounded_headers = extract_bounded_headers(text)
    if root.label == "responses":
        entry.response_wrapper_present = "CODEX RESPONSE BEGIN" in text
        if not entry.response_wrapper_present:
            entry.errors.append("response_wrapper_missing")
        entry.synthetic_smoke_marker = (
            "NA0386 SYNTHETIC REAL-ARCHIVE SMOKE FILE" in text
            or "NA0386_REAL_ARCHIVE_WRITE_AUTHORIZATION_OK" in text
            or "NA0388_SYNTHETIC_SMOKE_MARKER_OK" in text
        )
        if not entry.filename_inferred_target_na:
            entry.errors.append("filename_target_na_missing")
        if not entry.filename_inferred_directive_suffix:
            entry.errors.append("filename_directive_suffix_missing")
    entry.extraction_status = "metadata_extracted" if not entry.errors else "metadata_extracted_with_errors"
    return entry


def scan_root(root: RootSpec) -> tuple[dict[str, Any], list[FileEntry]]:
    root_info = {
        "label": root.label,
        "path": str(root.resolved),
        "exists": root.resolved.exists(),
        "file_count": 0,
        "errors": [],
    }
    entries: list[FileEntry] = []
    if not root.resolved.exists():
        return root_info, entries
    if root.resolved.is_symlink():
        root_info["errors"].append("root_symlink_rejected")
        return root_info, entries
    if not root.resolved.is_dir():
        root_info["errors"].append("root_not_directory")
        return root_info, entries

    for dirpath, dirnames, filenames in os.walk(root.resolved, topdown=True, followlinks=False):
        current = Path(dirpath)
        kept_dirs = []
        for dirname in sorted(dirnames):
            child = current / dirname
            if child.is_symlink():
                rel = child.relative_to(root.resolved).as_posix()
                root_info["errors"].append(f"symlink_directory_rejected:{rel}")
            else:
                kept_dirs.append(dirname)
        dirnames[:] = kept_dirs
        for filename in sorted(filenames):
            if filename == ".gitkeep":
                continue
            candidate = current / filename
            if candidate.is_symlink():
                entries.append(entry_for_symlink(root, candidate))
                continue
            try:
                resolved = candidate.resolve(strict=True)
                if not is_relative_to(resolved, root.resolved):
                    entries.append(
                        FileEntry(
                            source_root_label=root.label,
                            relative_path=candidate.relative_to(root.resolved).as_posix(),
                            classification=f"{classify(root.label)}_outside_root_rejected",
                            extraction_status="outside_root_rejected",
                            secret_scan_status="not_scanned_outside_root",
                            errors=["outside_root_rejected"],
                        )
                    )
                    continue
                if not resolved.is_file():
                    continue
                entries.append(scan_file(root, resolved))
            except OSError as exc:
                rel = candidate.relative_to(root.resolved).as_posix()
                entries.append(
                    FileEntry(
                        source_root_label=root.label,
                        relative_path=rel,
                        classification=f"{classify(root.label)}_unreadable",
                        extraction_status="unreadable",
                        secret_scan_status="not_scanned_unreadable",
                        errors=[f"unreadable:{exc.__class__.__name__}"],
                    )
                )
    root_info["file_count"] = len(entries)
    return root_info, entries


def mark_duplicate_directives(entries: list[FileEntry]) -> None:
    groups: dict[tuple[str, str, str], list[FileEntry]] = {}
    for entry in entries:
        if entry.source_root_label != "responses":
            continue
        if not entry.filename_inferred_target_na or not entry.filename_inferred_directive_suffix:
            continue
        key = (
            entry.source_root_label,
            entry.filename_inferred_target_na,
            entry.filename_inferred_directive_suffix,
        )
        groups.setdefault(key, []).append(entry)
    for duplicate_entries in groups.values():
        if len(duplicate_entries) <= 1:
            continue
        for entry in duplicate_entries:
            entry.errors.append("duplicate_inferred_directive")
            if entry.extraction_status == "metadata_extracted":
                entry.extraction_status = "metadata_extracted_with_errors"


def ensure_no_body_fields(value: Any, path: str = "$") -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            if key in FORBIDDEN_BODY_KEYS:
                raise CatalogError(f"full body/content field rejected at {path}.{key}")
            ensure_no_body_fields(child, f"{path}.{key}")
    elif isinstance(value, list):
        for index, child in enumerate(value):
            ensure_no_body_fields(child, f"{path}[{index}]")


def build_markers(catalog: dict[str, Any], *, fixture: bool) -> list[str]:
    labels = {root["label"] for root in catalog["roots"]}
    files = catalog["files"]
    errors = [err for entry in files for err in entry.get("errors", [])]
    markers = [
        "NA0388_RESPONSE_HISTORY_CATALOG_HELPER_OK",
        "NA0388_METADATA_ONLY_OK",
        "NA0388_NO_FULL_BODY_COPY_OK",
        "NA0388_TEMP_OUTPUT_BOUNDARY_OK",
        "NA0388_NO_REAL_CATALOG_WRITE_OK",
        "NA0388_NO_ARCHIVE_MUTATION_OK",
        "NA0388_NO_DELETE_OK",
        "NA0388_NO_SECRET_MATERIAL_OK",
        "NA0388_METADATA_RUNTIME_HISTORY_CATALOG_OK",
    ]
    if "responses" in labels:
        markers.append("NA0388_RESPONSE_ARCHIVE_SCAN_OK")
    if "requests" in labels:
        markers.append("NA0388_REQUESTS_SCAN_OK")
    if "directives" in labels:
        markers.append("NA0388_DIRECTIVES_ROOT_ABSENT_OR_SCANNED_OK")
    if "journals" in labels:
        markers.append("NA0388_JOURNALS_ROOT_ABSENT_OR_SCANNED_OK")
    if "ops" in labels:
        markers.append("NA0388_OPS_SCAN_OK")
    if any("secret_sentinel_rejected" in err for err in errors):
        markers.append("NA0388_SECRET_SENTINEL_REJECT_OK")
    if any("symlink_rejected" in err for err in errors):
        markers.append("NA0388_SYMLINK_OR_TRAVERSAL_REJECT_OK")
    if fixture:
        markers.extend(
            [
                "NA0388_RESPONSE_HISTORY_CATALOG_AUTHORIZATION_OK",
                "NA0388_SYMLINK_OR_TRAVERSAL_REJECT_OK",
                "NA0388_BACKUP_IMPACT_OK",
                "NA0388_NO_WORKFLOW_CHANGE_OK",
                "NA0388_NO_DEPENDENCY_CHANGE_OK",
                "NA0388_NO_RUNTIME_CHANGE_OK",
                "NA0388_NO_METADATA_FREE_CLAIM_OK",
                "NA0388_NO_ANONYMITY_CLAIM_OK",
                "NA0388_NO_UNTRACEABLE_CLAIM_OK",
                "NA0388_NO_PRODUCTION_READY_CLAIM_OK",
                "NA0388_NO_PUBLIC_INTERNET_READY_CLAIM_OK",
            ]
        )
    return sorted(dict.fromkeys(markers))


def build_catalog(roots: list[RootSpec], *, mode: str, fixture: bool = False) -> dict[str, Any]:
    root_infos: list[dict[str, Any]] = []
    entries: list[FileEntry] = []
    for root in sorted(roots, key=lambda item: item.label):
        root_info, root_entries = scan_root(root)
        root_infos.append(root_info)
        entries.extend(root_entries)
    mark_duplicate_directives(entries)
    file_dicts = [entry.as_dict() for entry in sorted(entries, key=lambda item: (item.source_root_label, item.relative_path))]
    catalog = {
        "schema_version": SCHEMA_VERSION,
        "generated_at_utc": utc_now(),
        "mode": mode,
        "roots": root_infos,
        "files": file_dicts,
        "summary": {
            "schema_version": SUMMARY_SCHEMA_VERSION,
            "root_count": len(root_infos),
            "file_count": len(file_dicts),
            "error_count": sum(len(entry.get("errors", [])) for entry in file_dicts)
            + sum(len(root.get("errors", [])) for root in root_infos),
            "secret_sentinel_rejected_count": sum(
                1 for entry in file_dicts if entry.get("secret_scan_status") == "secret_sentinel_rejected"
            ),
            "binary_or_non_utf8_count": sum(
                1 for entry in file_dicts if entry.get("extraction_status") == "binary_or_non_utf8"
            ),
            "symlink_rejected_count": sum(
                1 for entry in file_dicts if entry.get("extraction_status") == "symlink_rejected"
            ),
            "markers": [],
        },
    }
    ensure_no_body_fields(catalog)
    catalog["summary"]["markers"] = build_markers(catalog, fixture=fixture)
    return catalog


def human_summary(catalog: dict[str, Any], *, catalog_path: Path | None = None) -> str:
    summary = catalog["summary"]
    lines = [
        "QSL response/history catalog summary",
        f"schema_version: {catalog['schema_version']}",
        f"generated_at_utc: {catalog['generated_at_utc']}",
        f"mode: {catalog['mode']}",
        f"root_count: {summary['root_count']}",
        f"file_count: {summary['file_count']}",
        f"error_count: {summary['error_count']}",
        f"secret_sentinel_rejected_count: {summary['secret_sentinel_rejected_count']}",
        f"binary_or_non_utf8_count: {summary['binary_or_non_utf8_count']}",
        f"symlink_rejected_count: {summary['symlink_rejected_count']}",
    ]
    if catalog_path is not None:
        lines.append(f"catalog_path: {catalog_path}")
    lines.append("markers:")
    lines.extend(f"- {marker}" for marker in summary["markers"])
    return "\n".join(lines) + "\n"


def write_catalog_outputs(out_dir: Path, catalog: dict[str, Any]) -> tuple[Path, Path]:
    catalog_path = out_dir / "catalog.json"
    summary_path = out_dir / "summary.txt"
    for path in (catalog_path, summary_path):
        if path.exists():
            raise CatalogError(f"refusing to overwrite existing output: {path}")
    catalog_path.write_text(json.dumps(catalog, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    summary_path.write_text(human_summary(catalog, catalog_path=catalog_path), encoding="utf-8")
    return catalog_path, summary_path


def validate_catalog(catalog: dict[str, Any]) -> dict[str, Any]:
    if not isinstance(catalog, dict):
        raise CatalogError("catalog must be a JSON object")
    if catalog.get("schema_version") != SCHEMA_VERSION:
        raise CatalogError(f"catalog schema_version must be {SCHEMA_VERSION}")
    for field_name in ("generated_at_utc", "mode", "roots", "files", "summary"):
        if field_name not in catalog:
            raise CatalogError(f"catalog missing required field: {field_name}")
    if not isinstance(catalog["roots"], list):
        raise CatalogError("catalog roots must be a list")
    if not isinstance(catalog["files"], list):
        raise CatalogError("catalog files must be a list")
    ensure_no_body_fields(catalog)
    return {
        "schema_version": SUMMARY_SCHEMA_VERSION,
        "valid": True,
        "root_count": len(catalog["roots"]),
        "file_count": len(catalog["files"]),
        "markers": catalog.get("summary", {}).get("markers", []),
    }


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise CatalogError(f"malformed JSON in {path}: line {exc.lineno} column {exc.colno}") from exc
    except OSError as exc:
        raise CatalogError(f"cannot read {path}: {exc}") from exc


def expect(condition: bool, name: str, log_lines: list[str]) -> None:
    if not condition:
        log_lines.append(f"FIXTURE {name} FAIL")
        raise CatalogError(f"fixture case failed: {name}")
    log_lines.append(f"FIXTURE {name} PASS")


def find_entry(catalog: dict[str, Any], label: str, relative_path: str) -> dict[str, Any] | None:
    for entry in catalog["files"]:
        if entry.get("source_root_label") == label and entry.get("relative_path") == relative_path:
            return entry
    return None


def run_fixture_cases(fixture_dir: Path, tmp_dir: Path, catalog: dict[str, Any], log_lines: list[str]) -> None:
    matrix = load_json(fixture_dir / "fixture_cases.json")
    if not isinstance(matrix, dict) or matrix.get("schema_version") != FIXTURE_SCHEMA_VERSION:
        raise CatalogError(f"fixture_cases schema_version must be {FIXTURE_SCHEMA_VERSION}")
    cases = matrix.get("cases")
    if not isinstance(cases, list) or not cases:
        raise CatalogError("fixture cases must be a non-empty list")
    for raw_case in cases:
        if not isinstance(raw_case, dict):
            raise CatalogError("fixture case must be an object")
        name = str(raw_case.get("name") or "")
        kind = str(raw_case.get("kind") or "")
        if not name or not kind:
            raise CatalogError("fixture case missing name or kind")
        if kind == "catalog_entry":
            entry = find_entry(catalog, str(raw_case["label"]), str(raw_case["relative_path"]))
            expect(entry is not None, name, log_lines)
            for key, expected_value in raw_case.get("expect", {}).items():
                expect(entry.get(key) == expected_value, f"{name}:{key}", log_lines)
            for expected_error in raw_case.get("errors_include", []):
                expect(expected_error in entry.get("errors", []), f"{name}:error:{expected_error}", log_lines)
        elif kind == "root_entry":
            root = next((item for item in catalog["roots"] if item.get("label") == raw_case.get("label")), None)
            expect(root is not None, name, log_lines)
            for key, expected_value in raw_case.get("expect", {}).items():
                expect(root.get(key) == expected_value, f"{name}:{key}", log_lines)
        elif kind == "catalog_marker":
            marker = str(raw_case["marker"])
            expect(marker in catalog["summary"]["markers"], name, log_lines)
        elif kind == "json_fixture_valid":
            validate_catalog(load_json(fixture_dir / str(raw_case["path"])))
            log_lines.append(f"FIXTURE {name} PASS")
        elif kind == "full_body_catalog_rejected":
            bad_catalog = {
                "schema_version": SCHEMA_VERSION,
                "generated_at_utc": utc_now(),
                "mode": "fixture",
                "roots": [],
                "files": [{"source_root_label": "responses", "relative_path": "bad.md", "full_body_text": "not allowed"}],
                "summary": {"schema_version": SUMMARY_SCHEMA_VERSION, "markers": []},
            }
            try:
                validate_catalog(bad_catalog)
            except CatalogError:
                log_lines.append(f"FIXTURE {name} PASS")
            else:
                raise CatalogError(f"fixture case failed: {name}")
        elif kind == "generated_non_utf8":
            generated_base = tmp_dir / "generated_non_utf8"
            responses = generated_base / "responses"
            responses.mkdir(parents=True, exist_ok=True)
            binary_path = responses / "NA0388_20260530T140500-0500_D240.bin"
            binary_path.write_bytes(b"\xff\xfe\x00QSL")
            root = parse_root_spec(f"responses={responses}", mode="fixture", fixture_dir=generated_base)
            generated_catalog = build_catalog([root], mode="fixture", fixture=True)
            entry = find_entry(generated_catalog, "responses", binary_path.name)
            expect(entry is not None, name, log_lines)
            expect(entry.get("extraction_status") == "binary_or_non_utf8", f"{name}:binary_status", log_lines)
        elif kind == "generated_symlink":
            generated_base = tmp_dir / "generated_symlink"
            responses = generated_base / "responses"
            responses.mkdir(parents=True, exist_ok=True)
            target = tmp_dir / "symlink_target.txt"
            target.write_text("outside symlink target\n", encoding="utf-8")
            link = responses / "NA0388_20260530T140600-0500_D241.md"
            if not link.exists():
                link.symlink_to(target)
            root = parse_root_spec(f"responses={responses}", mode="fixture", fixture_dir=generated_base)
            generated_catalog = build_catalog([root], mode="fixture", fixture=True)
            entry = find_entry(generated_catalog, "responses", link.name)
            expect(entry is not None, name, log_lines)
            expect(entry.get("extraction_status") == "symlink_rejected", f"{name}:symlink_status", log_lines)
            log_lines.append("MARKER NA0388_SYMLINK_OR_TRAVERSAL_REJECT_OK")
        elif kind == "reject_root":
            try:
                parse_root_spec(str(raw_case["root"]), mode="fixture", fixture_dir=fixture_dir)
            except CatalogError:
                log_lines.append(f"FIXTURE {name} PASS")
                log_lines.append("MARKER NA0388_SYMLINK_OR_TRAVERSAL_REJECT_OK")
            else:
                raise CatalogError(f"fixture case failed: {name}")
        elif kind == "reject_output_dir":
            try:
                validate_temp_output_dir(str(raw_case["path"]))
            except CatalogError:
                log_lines.append(f"FIXTURE {name} PASS")
            else:
                raise CatalogError(f"fixture case failed: {name}")
        else:
            raise CatalogError(f"unknown fixture case kind: {kind}")


def command_scan(args: argparse.Namespace) -> int:
    out_dir = validate_temp_output_dir(args.out_dir)
    roots = [parse_root_spec(raw, mode="live") for raw in args.root]
    catalog = build_catalog(roots, mode="live")
    catalog_path, _summary_path = write_catalog_outputs(out_dir, catalog)
    summary = validate_catalog(catalog)
    summary["catalog_path"] = str(catalog_path)
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(human_summary(catalog, catalog_path=catalog_path), end="")
    return EXIT_OK


def command_fixture(args: argparse.Namespace) -> int:
    fixture_dir = Path(args.fixture_dir).resolve(strict=True)
    tmp_dir = validate_temp_output_dir(args.tmp_dir)
    roots = [parse_root_spec(f"{label}={fixture_dir / label}", mode="fixture", fixture_dir=fixture_dir) for label in ROOT_LABELS]
    catalog = build_catalog(roots, mode="fixture", fixture=True)
    catalog_path, _summary_path = write_catalog_outputs(tmp_dir, catalog)
    log_path = tmp_dir / "fixture_matrix.log"
    if log_path.exists():
        raise CatalogError(f"refusing to overwrite fixture log: {log_path}")
    log_lines = [
        "NA-0388 response history catalog fixture matrix",
        f"fixture_dir: {fixture_dir}",
        f"tmp_dir: {tmp_dir}",
        f"catalog_path: {catalog_path}",
    ]
    run_fixture_cases(fixture_dir, tmp_dir, catalog, log_lines)
    for marker in catalog["summary"]["markers"]:
        log_lines.append(f"MARKER {marker}")
    log_path.write_text("\n".join(log_lines) + "\n", encoding="utf-8")
    summary = validate_catalog(catalog)
    summary["catalog_path"] = str(catalog_path)
    summary["fixture_log_path"] = str(log_path)
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(human_summary(catalog, catalog_path=catalog_path), end="")
        print(f"fixture_log_path: {log_path}")
    return EXIT_OK


def command_validate(args: argparse.Namespace) -> int:
    catalog = load_json(Path(args.catalog))
    summary = validate_catalog(catalog)
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(f"catalog valid: {args.catalog}")
        print(f"root_count: {summary['root_count']}")
        print(f"file_count: {summary['file_count']}")
    return EXIT_OK


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Build metadata-only QSL response/history catalogs under /srv/qbuild/tmp.")
    sub = parser.add_subparsers(dest="command", required=True)

    scan = sub.add_parser("scan", help="Scan explicitly supplied live history roots read-only.")
    scan.add_argument("--root", action="append", required=True, help="Allowed root in label=path form.")
    scan.add_argument("--out-dir", required=True, help="Output directory under /srv/qbuild/tmp.")
    scan.add_argument("--json", action="store_true", help="Emit JSON summary to stdout.")
    scan.set_defaults(func=command_scan)

    fixture = sub.add_parser("fixture", help="Run deterministic no-network fixture matrix.")
    fixture.add_argument("--fixture-dir", required=True)
    fixture.add_argument("--tmp-dir", required=True)
    fixture.add_argument("--json", action="store_true", help="Emit JSON summary to stdout.")
    fixture.set_defaults(func=command_fixture)

    validate = sub.add_parser("validate", help="Validate a catalog JSON file.")
    validate.add_argument("--catalog", required=True)
    validate.add_argument("--json", action="store_true", help="Emit JSON validation summary.")
    validate.set_defaults(func=command_validate)
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return args.func(args)
    except CatalogError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return EXIT_VALIDATION
    except Exception as exc:  # pragma: no cover - defensive top-level guard.
        print(f"INTERNAL_ERROR: {exc.__class__.__name__}: {exc}", file=sys.stderr)
        return EXIT_INTERNAL


if __name__ == "__main__":
    raise SystemExit(main())
