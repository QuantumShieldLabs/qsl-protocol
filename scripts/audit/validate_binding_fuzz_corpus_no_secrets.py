#!/usr/bin/env python3
"""Validate qsc fuzz corpora for checked-in secret material.

The validator is intentionally dependency-free and report-only. It reads the
requested corpus paths, emits deterministic redacted findings, and exits
non-zero when disallowed material is detected.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


VALIDATOR_NAME = "validate_binding_fuzz_corpus_no_secrets"
SCHEMA_VERSION = "1"
DEFAULT_CORPUS_PATH = Path("qsl/qsl-client/qsc/fuzz/corpus")

MARKERS = [
    "NA0491_VALIDATOR_SCOPE_CONSUMED_OK",
    "NA0491_SECRET_PATTERN_MATRIX_READY_OK",
    "NA0491_VALIDATOR_SCRIPT_IMPLEMENTED_OK",
    "NA0491_VALIDATOR_REJECTS_PRIVATE_KEY_MARKERS_OK",
    "NA0491_VALIDATOR_REJECTS_SECRET_LABELS_OK",
    "NA0491_VALIDATOR_REJECTS_HIGH_ENTROPY_UNALLOWLISTED_OK",
    "NA0491_VALIDATOR_ALLOWS_SYNTHETIC_PUBLIC_BYTES_OK",
    "NA0491_VALIDATOR_REPORTS_DETERMINISTIC_JSON_OK",
    "NA0491_NO_DEPENDENCY_CHANGE_OK",
    "NA0491_NO_WORKFLOW_CHANGE_OK",
    "NA0491_NO_CORPUS_MUTATION_OK",
    "NA0491_NO_PUBLIC_READINESS_CLAIM_OK",
    "NA0491_NO_CRYPTO_COMPLETE_CLAIM_OK",
    "NA0491_NO_FUZZ_COMPLETE_CLAIM_OK",
    "NA0491_NO_CORPUS_COMPLETE_CLAIM_OK",
    "NA0491_NO_VECTOR_COMPLETE_CLAIM_OK",
]

TEXT_CHARS = set(range(32, 127)) | {9, 10, 13}
BASE64_MIN_LENGTH = 96
BASE64_MIN_ENTROPY = 4.70
BASE64_MIN_UNIQUE = 24
HEX_MIN_LENGTH = 96
HEX_MIN_ENTROPY = 3.55
HEX_MIN_UNIQUE = 12

BASE64_SPAN_RE = re.compile(rb"(?<![A-Za-z0-9+/=_-])([A-Za-z0-9+/=_-]{96,})(?![A-Za-z0-9+/=_-])")
HEX_SPAN_RE = re.compile(rb"(?<![A-Fa-f0-9])([A-Fa-f0-9]{96,})(?![A-Fa-f0-9])")


@dataclass(frozen=True)
class Rule:
    kind: str
    severity: str
    pattern: re.Pattern[bytes]


@dataclass(frozen=True)
class Finding:
    path: str
    kind: str
    severity: str
    byte_offset: int | None
    line: int | None
    context_hash: str

    def as_dict(self) -> dict[str, object]:
        return {
            "path": self.path,
            "kind": self.kind,
            "severity": self.severity,
            "byte_offset": self.byte_offset,
            "line": self.line,
            "context_hash": self.context_hash,
            "redaction": "[redacted]",
        }


@dataclass(frozen=True)
class PathStatus:
    path: str
    status: str

    def as_dict(self) -> dict[str, str]:
        return {"path": self.path, "status": self.status}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def private_key_rules() -> list[Rule]:
    dash5 = b"-" * 5
    begin = b"BEGIN "
    private = b"PRIVATE"
    key = b"KEY"
    header = re.escape(dash5 + begin) + rb"(?:[A-Z0-9 ]{0,32})" + re.escape(private + b" " + key + dash5)
    openssh_magic = re.escape(b"openssh-" + b"key-v1")
    age_marker = re.escape(b"AGE-" + b"SECRET-" + b"KEY-1")
    minisign_marker = re.escape(b"minisign " + b"secret " + b"key")
    return [
        Rule("private_key_marker", "critical", re.compile(header, re.IGNORECASE)),
        Rule("openssh_private_key_marker", "critical", re.compile(openssh_magic, re.IGNORECASE)),
        Rule("age_private_key_marker", "critical", re.compile(age_marker, re.IGNORECASE)),
        Rule("minisign_private_key_marker", "critical", re.compile(minisign_marker, re.IGNORECASE)),
    ]


def label_rules() -> list[Rule]:
    def rule(kind: str, pattern: bytes, severity: str = "high") -> Rule:
        return Rule(kind, severity, re.compile(pattern, re.IGNORECASE))

    boundary = rb"(?<![A-Za-z0-9])"
    end = rb"(?![A-Za-z0-9])"
    sep = rb"[_ -]?"
    return [
        rule(
            "api_token_label",
            boundary
            + rb"(?:api"
            + sep
            + rb"(?:key|token)|access"
            + sep
            + rb"token|auth"
            + sep
            + rb"token|secret"
            + sep
            + rb"token|client"
            + sep
            + rb"secret|bearer"
            + sep
            + rb"token|github"
            + sep
            + rb"token|slack"
            + sep
            + rb"token|openai"
            + sep
            + rb"api"
            + sep
            + rb"key|aws"
            + sep
            + rb"secret"
            + sep
            + rb"access"
            + sep
            + rb"key)"
            + end,
        ),
        rule("github_token_pattern", rb"\bgh[pousr]_[A-Za-z0-9_]{30,}\b", "critical"),
        rule("slack_token_pattern", rb"\bxox[baprs]-[A-Za-z0-9-]{20,}\b", "critical"),
        rule("aws_access_key_id_pattern", rb"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b", "critical"),
        rule("google_api_key_pattern", rb"\bAIza[0-9A-Za-z_-]{35}\b", "critical"),
        rule("openai_key_pattern", rb"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b", "critical"),
        rule("jwt_pattern", rb"\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}\b", "critical"),
        rule("passphrase_label", boundary + rb"(?:pass" + sep + rb"phrase|password|passwd)" + end),
        rule(
            "kem_secret_key_label",
            boundary
            + rb"(?:(?:kem|ml"
            + sep
            + rb"kem|mlkem|kyber)"
            + sep
            + rb"(?:secret"
            + sep
            + rb"key|private"
            + sep
            + rb"key|sk)|kem"
            + sep
            + rb"sk|decapsulation"
            + sep
            + rb"key)"
            + end,
        ),
        rule(
            "signature_secret_key_label",
            boundary
            + rb"(?:signature|sig|signing)"
            + sep
            + rb"(?:secret"
            + sep
            + rb"key|private"
            + sep
            + rb"key|sk|seed)"
            + end,
        ),
        rule(
            "identity_secret_key_label",
            boundary
            + rb"(?:identity|device)"
            + sep
            + rb"(?:secret"
            + sep
            + rb"key|private"
            + sep
            + rb"key|seed)"
            + end,
        ),
        rule(
            "backup_recovery_key_label",
            boundary
            + rb"(?:backup"
            + sep
            + rb"(?:key|seed)|recovery"
            + sep
            + rb"(?:key|phrase|seed)|restore"
            + sep
            + rb"key)"
            + end,
        ),
        rule(
            "runtime_service_secret_label",
            boundary
            + rb"(?:runtime"
            + sep
            + rb"secret|service"
            + sep
            + rb"secret|jwt"
            + sep
            + rb"secret|session"
            + sep
            + rb"secret|encryption"
            + sep
            + rb"key|master"
            + sep
            + rb"key|database"
            + sep
            + rb"password|db"
            + sep
            + rb"password)"
            + end,
        ),
        rule(
            "private_endpoint_marker",
            boundary
            + rb"(?:private"
            + sep
            + rb"endpoint|production"
            + sep
            + rb"endpoint|prod"
            + sep
            + rb"endpoint|internal"
            + sep
            + rb"endpoint|prod"
            + sep
            + rb"api|live"
            + sep
            + rb"service"
            + sep
            + rb"url)"
            + end,
        ),
        rule(
            "operator_user_data_marker",
            boundary
            + rb"(?:(?:operator|user|customer)"
            + sep
            + rb"(?:private"
            + sep
            + rb"data|secret"
            + sep
            + rb"data|real"
            + sep
            + rb"data)|real"
            + sep
            + rb"user"
            + sep
            + rb"data)"
            + end,
        ),
        rule(
            "qsc_secret_filename_marker",
            boundary
            + rb"(?:qsc"
            + sep
            + rb"vault"
            + sep
            + rb"(?:secret|key|seed|passphrase)|vault"
            + sep
            + rb"(?:secret|key|seed|passphrase)|qsc"
            + sep
            + rb"(?:identity|profile)"
            + sep
            + rb"(?:secret|private|seed))"
            + end,
        ),
    ]


RULES = private_key_rules() + label_rules()


def display_path(path: Path, root: Path) -> str:
    try:
        return path.resolve(strict=False).relative_to(root).as_posix()
    except ValueError:
        return path.resolve(strict=False).as_posix()


def requested_paths(args: argparse.Namespace, root: Path) -> list[Path]:
    raw_paths: list[str] = []
    if args.path:
        raw_paths.extend(args.path)
    if args.paths:
        raw_paths.extend(args.paths)
    if not raw_paths:
        raw_paths = [DEFAULT_CORPUS_PATH.as_posix()]

    paths: list[Path] = []
    for raw in raw_paths:
        p = Path(raw)
        if not p.is_absolute():
            p = root / p
        paths.append(p)
    return paths


def redacted_hash(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()[:16]


def line_number(data: bytes, offset: int) -> int:
    return data.count(b"\n", 0, offset) + 1


def is_mostly_text(data: bytes) -> bool:
    if not data:
        return True
    sample = data[:4096]
    text_count = sum(1 for b in sample if b in TEXT_CHARS)
    return text_count / len(sample) >= 0.85


def shannon_entropy(data: bytes) -> float:
    if not data:
        return 0.0
    total = len(data)
    entropy = 0.0
    for byte in set(data):
        p = data.count(byte) / total
        entropy -= p * math.log2(p)
    return entropy


def make_finding(path: Path, root: Path, kind: str, severity: str, data: bytes, start: int, end: int) -> Finding:
    return Finding(
        path=display_path(path, root),
        kind=kind,
        severity=severity,
        byte_offset=start,
        line=line_number(data, start) if is_mostly_text(data) else None,
        context_hash=redacted_hash(data[start:end]),
    )


def scan_path_label(path: Path, root: Path) -> list[Finding]:
    label = display_path(path, root).encode("utf-8", errors="replace")
    findings: list[Finding] = []
    for rule in RULES:
        if rule.kind == "qsc_secret_filename_marker":
            match = rule.pattern.search(label)
            if match:
                findings.append(
                    Finding(
                        path=display_path(path, root),
                        kind=rule.kind,
                        severity=rule.severity,
                        byte_offset=None,
                        line=None,
                        context_hash=redacted_hash(match.group(0)),
                    )
                )
    return findings


def scan_content(path: Path, root: Path, data: bytes) -> list[Finding]:
    findings: list[Finding] = []
    for rule in RULES:
        for match in rule.pattern.finditer(data):
            findings.append(make_finding(path, root, rule.kind, rule.severity, data, match.start(), match.end()))

    for match in HEX_SPAN_RE.finditer(data):
        span = match.group(1)
        if len(set(span)) >= HEX_MIN_UNIQUE and shannon_entropy(span) >= HEX_MIN_ENTROPY:
            findings.append(
                make_finding(
                    path,
                    root,
                    "high_entropy_hex_span",
                    "high",
                    data,
                    match.start(1),
                    match.end(1),
                )
            )

    for match in BASE64_SPAN_RE.finditer(data):
        span = match.group(1)
        if len(set(span)) >= BASE64_MIN_UNIQUE and shannon_entropy(span) >= BASE64_MIN_ENTROPY:
            findings.append(
                make_finding(
                    path,
                    root,
                    "high_entropy_encoded_span",
                    "high",
                    data,
                    match.start(1),
                    match.end(1),
                )
            )
    return findings


def iter_files(path: Path) -> Iterable[Path]:
    if path.is_file():
        yield path
        return
    for dirpath, dirnames, filenames in os.walk(path, followlinks=False):
        dirnames[:] = sorted(name for name in dirnames if not (Path(dirpath) / name).is_symlink())
        for name in sorted(filenames):
            candidate = Path(dirpath) / name
            if not candidate.is_symlink():
                yield candidate


def scan(paths: list[Path], root: Path, allow_missing: bool) -> tuple[list[PathStatus], list[Finding], int, int]:
    statuses: list[PathStatus] = []
    findings: list[Finding] = []
    files_scanned = 0
    bytes_scanned = 0

    for path in sorted(paths, key=lambda p: display_path(p, root)):
        shown = display_path(path, root)
        if not path.exists():
            if allow_missing:
                statuses.append(PathStatus(shown, "missing_allowed"))
            else:
                statuses.append(PathStatus(shown, "missing_disallowed"))
                findings.append(Finding(shown, "missing_path", "high", None, None, redacted_hash(shown.encode())))
            continue
        if path.is_symlink():
            statuses.append(PathStatus(shown, "symlink_rejected"))
            findings.append(Finding(shown, "symlink_rejected", "high", None, None, redacted_hash(shown.encode())))
            continue

        statuses.append(PathStatus(shown, "scanned"))
        for file_path in iter_files(path):
            findings.extend(scan_path_label(file_path, root))
            try:
                data = file_path.read_bytes()
            except OSError:
                file_shown = display_path(file_path, root)
                findings.append(
                    Finding(file_shown, "read_error", "high", None, None, redacted_hash(file_shown.encode()))
                )
                continue
            files_scanned += 1
            bytes_scanned += len(data)
            findings.extend(scan_content(file_path, root, data))

    findings.sort(key=lambda f: (f.path, f.byte_offset if f.byte_offset is not None else -1, f.kind, f.context_hash))
    statuses.sort(key=lambda s: (s.path, s.status))
    return statuses, findings, files_scanned, bytes_scanned


def build_report(args: argparse.Namespace) -> dict[str, object]:
    root = repo_root()
    paths = requested_paths(args, root)
    statuses, findings, files_scanned, bytes_scanned = scan(paths, root, args.allow_missing)
    result = "fail" if findings else "pass"
    return {
        "schema_version": SCHEMA_VERSION,
        "validator": VALIDATOR_NAME,
        "markers": MARKERS,
        "policy": {
            "allow_missing": args.allow_missing,
            "default_path": DEFAULT_CORPUS_PATH.as_posix(),
            "follow_symlinks": False,
            "high_entropy": {
                "base64_min_length": BASE64_MIN_LENGTH,
                "base64_min_entropy": BASE64_MIN_ENTROPY,
                "hex_min_length": HEX_MIN_LENGTH,
                "hex_min_entropy": HEX_MIN_ENTROPY,
            },
        },
        "summary": {
            "result": result,
            "paths_requested": [display_path(p, root) for p in paths],
            "paths_scanned": sum(1 for status in statuses if status.status == "scanned"),
            "paths_missing_allowed": sum(1 for status in statuses if status.status == "missing_allowed"),
            "paths_missing_disallowed": sum(1 for status in statuses if status.status == "missing_disallowed"),
            "files_scanned": files_scanned,
            "bytes_scanned": bytes_scanned,
            "finding_count": len(findings),
        },
        "paths": [status.as_dict() for status in statuses],
        "findings": [finding.as_dict() for finding in findings],
    }


def print_text(report: dict[str, object]) -> None:
    summary = report["summary"]
    assert isinstance(summary, dict)
    print(f"VALIDATOR {report['validator']}")
    print(f"RESULT {summary['result']}")
    print(f"FILES_SCANNED {summary['files_scanned']}")
    print(f"BYTES_SCANNED {summary['bytes_scanned']}")
    print(f"FINDING_COUNT {summary['finding_count']}")
    print(f"PATHS_MISSING_ALLOWED {summary['paths_missing_allowed']}")
    for finding in report["findings"]:
        assert isinstance(finding, dict)
        print(
            "FINDING "
            f"severity={finding['severity']} "
            f"kind={finding['kind']} "
            f"path={finding['path']} "
            f"byte_offset={finding['byte_offset']} "
            f"line={finding['line']} "
            f"context_hash={finding['context_hash']} "
            "redaction=[redacted]"
        )


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate qsc fuzz corpus paths for disallowed secret material without printing matched payloads."
    )
    parser.add_argument("--path", action="append", help="Corpus path to scan; may be supplied more than once.")
    parser.add_argument("--paths", nargs="+", help="One or more corpus paths to scan.")
    parser.add_argument("--allow-missing", action="store_true", help="Treat missing requested paths as allowed.")
    parser.add_argument("--format", choices=("json", "text"), default="text", help="Report format.")
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    report = build_report(args)
    if args.format == "json":
        print(json.dumps(report, sort_keys=True, separators=(",", ":")))
    else:
        print_text(report)
    summary = report["summary"]
    assert isinstance(summary, dict)
    return 0 if summary["finding_count"] == 0 else 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
