#!/usr/bin/env python3
"""Standalone QSL Codex response writer harness.

The helper is intentionally local-only. It uses the Python standard library,
does not call the network or GitHub, and writes only new response-format files
under explicitly supplied /srv/qbuild/tmp output directories in write/fixture
mode.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

try:
    from zoneinfo import ZoneInfo
except ImportError:  # pragma: no cover - Python 3.9+ in supported qbuild lanes.
    ZoneInfo = None  # type: ignore[assignment]


SCHEMA_VERSION = "qsl.codex_response_writer.metadata.v1"
FIXTURE_SCHEMA_VERSION = "qsl.codex_response_writer.fixture_matrix.v1"
TEMP_ROOT = Path("/srv/qbuild/tmp")
REAL_RESPONSE_ARCHIVE = Path("/home/victor/work/qsl/codex/responses")
TEMP_OUTPUT_MODE = "temp-output"
REAL_ARCHIVE_SMOKE_MODE = "real_archive_smoke"
EXIT_OK = 0
EXIT_INTERNAL = 1
EXIT_VALIDATION = 2

REQUIRED_METADATA_FIELDS = {
    "schema_version",
    "target_na",
    "directive_suffix",
    "directive_id",
    "response_start_local",
    "response_start_utc",
    "directive_begin_local",
    "directive_begin_utc",
    "directive_end_local",
    "directive_end_utc",
    "timezone",
    "timezone_offset",
    "output_mode",
    "required_sections",
    "allow_real_archive_output",
    "no_secret_required",
}

OPTIONAL_METADATA_FIELDS = {
    "response_end_local",
    "response_end_utc",
}

FORBIDDEN_METADATA_FIELDS = {
    "cleanup_after_write",
    "create_directive_index",
    "create_journal_index",
    "create_response_index",
    "delete_after_write",
    "delete_existing",
    "index_output",
    "mutate_existing",
    "response_index",
}

REQUIRED_NA_SECTIONS = [
    "0. Directive / Response Identity Check",
    "1. Layman Summary",
    "2. Summary",
    "3. Branch / Commit / PR",
    "4. Queue Proof",
    "Stop Reason, If Stopped",
]

SECRET_PATTERNS = [
    ("private_key_block", re.compile(r"-----BEGIN (?:[A-Z0-9]+ )?PRIVATE KEY-----")),
    ("github_token", re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{30,}\b")),
    ("github_pat", re.compile(r"\bgithub_pat_[A-Za-z0-9_]{30,}\b")),
    ("slack_token", re.compile(r"\bxox[baprs]-[A-Za-z0-9-]{20,}\b")),
    ("aws_access_key_id", re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b")),
    ("google_api_key", re.compile(r"\bAIza[0-9A-Za-z_-]{35}\b")),
    ("openai_key", re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b")),
    ("jwt", re.compile(r"\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}\b")),
    (
        "credential_label",
        re.compile(r"(?i)\b(password|passphrase|secret|token|credential)\s*[:=]\s*\S+"),
    ),
    (
        "recovery_envelope_marker",
        re.compile(r"(?i)\b(BEGIN QSL RECOVERY ENVELOPE|recovery[-_ ]?envelope\s*[:=])"),
    ),
    ("raw_credential_marker", re.compile(r"(?i)\braw[-_ ]?credential\s*[:=]")),
    ("qsl_test_secret_sentinel", re.compile(r"\bQSL_TEST_FORBIDDEN_SECRET_SENTINEL\b")),
]


class ResponseWriterError(RuntimeError):
    """Expected fail-closed helper error."""


@dataclass
class ValidationResult:
    command: str
    ok: bool
    wrote: bool = False
    output_path: str | None = None
    candidate_path: str | None = None
    filename: str | None = None
    sha256: str | None = None
    markers: list[str] = field(default_factory=list)
    errors: list[str] = field(default_factory=list)
    extra: dict[str, Any] | None = None

    def as_dict(self) -> dict[str, Any]:
        return {
            "schema_version": "qsl.codex_response_writer.summary.v1",
            "command": self.command,
            "ok": self.ok,
            "wrote": self.wrote,
            "output_path": self.output_path,
            "candidate_path": self.candidate_path,
            "filename": self.filename,
            "sha256": self.sha256,
            "markers": self.markers,
            "errors": self.errors,
            "extra": self.extra,
        }


@dataclass(frozen=True)
class ResponseInputs:
    metadata_path: Path
    body_path: Path
    metadata: dict[str, Any]
    body: str
    target_prefix: str
    response_start: datetime
    timezone_offset_compact: str
    required_sections: list[str]
    output_mode: str
    real_archive_output: bool


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ResponseWriterError(f"malformed JSON in {path}: line {exc.lineno} column {exc.colno}") from exc
    except OSError as exc:
        raise ResponseWriterError(f"cannot read {path}: {exc}") from exc


def read_text(path: Path, label: str) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as exc:
        raise ResponseWriterError(f"cannot read {label} {path}: {exc}") from exc


def ensure_mapping(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ResponseWriterError(f"{label} must be a JSON object")
    return value


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise ResponseWriterError(f"{label} must be a boolean")
    return value


def ensure_string(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise ResponseWriterError(f"{label} must be a non-empty string")
    return value


def parse_timestamp(value: str, label: str) -> datetime:
    normalized = value[:-1] + "+00:00" if value.endswith("Z") else value
    try:
        parsed = datetime.fromisoformat(normalized)
    except ValueError as exc:
        raise ResponseWriterError(f"{label} must be an ISO-8601 timestamp with timezone offset") from exc
    if parsed.tzinfo is None or parsed.utcoffset() is None:
        raise ResponseWriterError(f"{label} must include a timezone offset")
    return parsed


def validate_utc_pair(local_value: str, utc_value: str, local_label: str, utc_label: str) -> tuple[datetime, datetime]:
    local_dt = parse_timestamp(local_value, local_label)
    utc_dt = parse_timestamp(utc_value, utc_label)
    if utc_dt.utcoffset() != timezone.utc.utcoffset(utc_dt):
        raise ResponseWriterError(f"{utc_label} must use UTC offset")
    if local_dt.astimezone(timezone.utc) != utc_dt.astimezone(timezone.utc):
        raise ResponseWriterError(f"{local_label} and {utc_label} must describe the same instant")
    return local_dt, utc_dt


def validate_timezone_offset(offset: str) -> str:
    if not re.fullmatch(r"[+-]\d{2}:\d{2}", offset):
        raise ResponseWriterError("timezone_offset must match +/-HH:MM")
    hours = int(offset[1:3])
    minutes = int(offset[4:6])
    if hours > 14 or minutes > 59:
        raise ResponseWriterError("timezone_offset is out of range")
    return offset


def format_offset(dt: datetime) -> str:
    offset = dt.utcoffset()
    if offset is None:
        raise ResponseWriterError("timestamp lacks offset")
    total_minutes = int(offset.total_seconds() // 60)
    sign = "+" if total_minutes >= 0 else "-"
    total_minutes = abs(total_minutes)
    return f"{sign}{total_minutes // 60:02d}:{total_minutes % 60:02d}"


def compact_offset(offset: str) -> str:
    return offset.replace(":", "")


def canonical_target_na(value: str) -> str:
    match = re.fullmatch(r"NA-?(\d{4})", value.strip())
    if not match:
        raise ResponseWriterError("target_na must match NA-0000 or NA0000")
    return f"NA{match.group(1)}"


def validate_directive_suffix(value: str) -> str:
    if not re.fullmatch(r"D\d{3}", value.strip()):
        raise ResponseWriterError("directive_suffix must match Dnnn")
    return value.strip()


def validate_directive_id(value: str) -> str:
    if not re.fullmatch(r"QSL-DIR-\d{4}-\d{2}-\d{2}-\d{3}", value.strip()):
        raise ResponseWriterError("directive_id must match QSL-DIR-YYYY-MM-DD-NNN")
    return value.strip()


def iter_strings(value: Any, path: str = "$") -> list[tuple[str, str]]:
    found: list[tuple[str, str]] = []
    if isinstance(value, dict):
        for key, child in value.items():
            found.extend(iter_strings(child, f"{path}.{key}"))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            found.extend(iter_strings(child, f"{path}[{index}]"))
    elif isinstance(value, str):
        found.append((path, value))
    return found


def scan_no_secret(metadata: dict[str, Any], body: str) -> None:
    findings: list[str] = []
    for path, value in iter_strings(metadata):
        for name, pattern in SECRET_PATTERNS:
            if pattern.search(value):
                findings.append(f"metadata {path}: {name}")
    for name, pattern in SECRET_PATTERNS:
        if pattern.search(body):
            findings.append(f"body: {name}")
    if findings:
        raise ResponseWriterError("secret pattern rejected before write: " + "; ".join(findings))


def validate_required_sections(value: Any, body: str) -> list[str]:
    if not isinstance(value, list) or not value:
        raise ResponseWriterError("required_sections must be a non-empty list")
    sections = [ensure_string(item, "required_sections item").strip() for item in value]
    missing_required: list[str] = []
    for section in REQUIRED_NA_SECTIONS:
        if section == "Stop Reason, If Stopped":
            if section not in sections and not any(item.endswith(". Stop Reason, If Stopped") for item in sections):
                missing_required.append(section)
            continue
        if section not in sections:
            missing_required.append(section)
    if missing_required:
        raise ResponseWriterError("required_sections is missing baseline headings: " + ", ".join(missing_required))
    body_lines = {line.strip().lstrip("#").strip() for line in body.splitlines()}
    missing_body = [section for section in sections if section not in body_lines]
    if missing_body:
        raise ResponseWriterError("body is missing required sections: " + ", ".join(missing_body))
    return sections


def validate_metadata(
    metadata: dict[str, Any],
    body: str,
    *,
    cli_allow_real_archive_output: bool,
) -> tuple[str, datetime, str, list[str], str, bool]:
    forbidden = set(metadata) & FORBIDDEN_METADATA_FIELDS
    if forbidden:
        raise ResponseWriterError(
            "forbidden metadata fields request index/delete/mutation behavior: "
            + ", ".join(sorted(forbidden))
        )
    unknown = set(metadata) - REQUIRED_METADATA_FIELDS - OPTIONAL_METADATA_FIELDS
    if unknown:
        raise ResponseWriterError("unknown metadata fields: " + ", ".join(sorted(unknown)))
    missing = REQUIRED_METADATA_FIELDS - set(metadata)
    if missing:
        raise ResponseWriterError("missing required metadata fields: " + ", ".join(sorted(missing)))
    if metadata["schema_version"] != SCHEMA_VERSION:
        raise ResponseWriterError(f"schema_version must be {SCHEMA_VERSION}")

    target_prefix = canonical_target_na(ensure_string(metadata["target_na"], "target_na"))
    directive_suffix = validate_directive_suffix(ensure_string(metadata["directive_suffix"], "directive_suffix"))
    validate_directive_id(ensure_string(metadata["directive_id"], "directive_id"))
    timezone_name = ensure_string(metadata["timezone"], "timezone")
    if timezone_name != "America/Chicago":
        raise ResponseWriterError("timezone must be America/Chicago for this harness")
    offset = validate_timezone_offset(ensure_string(metadata["timezone_offset"], "timezone_offset"))
    response_start, _ = validate_utc_pair(
        ensure_string(metadata["response_start_local"], "response_start_local"),
        ensure_string(metadata["response_start_utc"], "response_start_utc"),
        "response_start_local",
        "response_start_utc",
    )
    validate_utc_pair(
        ensure_string(metadata["directive_begin_local"], "directive_begin_local"),
        ensure_string(metadata["directive_begin_utc"], "directive_begin_utc"),
        "directive_begin_local",
        "directive_begin_utc",
    )
    validate_utc_pair(
        ensure_string(metadata["directive_end_local"], "directive_end_local"),
        ensure_string(metadata["directive_end_utc"], "directive_end_utc"),
        "directive_end_local",
        "directive_end_utc",
    )
    if "response_end_local" in metadata or "response_end_utc" in metadata:
        if "response_end_local" not in metadata or "response_end_utc" not in metadata:
            raise ResponseWriterError("response_end_local and response_end_utc must be supplied together")
        validate_utc_pair(
            ensure_string(metadata["response_end_local"], "response_end_local"),
            ensure_string(metadata["response_end_utc"], "response_end_utc"),
            "response_end_local",
            "response_end_utc",
        )
    if format_offset(response_start) != offset:
        raise ResponseWriterError("response_start_local offset must match timezone_offset")
    output_mode = ensure_string(metadata["output_mode"], "output_mode")
    metadata_allows_real_archive = ensure_bool(metadata["allow_real_archive_output"], "allow_real_archive_output")
    if output_mode == TEMP_OUTPUT_MODE:
        if metadata_allows_real_archive:
            raise ResponseWriterError("allow_real_archive_output must be false for temp-output")
        real_archive_output = False
    elif output_mode == REAL_ARCHIVE_SMOKE_MODE:
        if not cli_allow_real_archive_output:
            raise ResponseWriterError("real archive output requires --allow-real-archive-output")
        if not metadata_allows_real_archive:
            raise ResponseWriterError("real archive output requires metadata allow_real_archive_output true")
        if target_prefix != "NA0386":
            raise ResponseWriterError("real archive smoke target_na must be NA-0386")
        if directive_suffix != "D205":
            raise ResponseWriterError("real archive smoke directive_suffix must be D205")
        real_archive_output = True
    else:
        raise ResponseWriterError(f"output_mode must be {TEMP_OUTPUT_MODE} or {REAL_ARCHIVE_SMOKE_MODE}")
    if not ensure_bool(metadata["no_secret_required"], "no_secret_required"):
        raise ResponseWriterError("no_secret_required must be true")
    sections = validate_required_sections(metadata["required_sections"], body)
    return target_prefix, response_start, compact_offset(offset), sections, output_mode, real_archive_output


def load_inputs(
    metadata_path: Path,
    body_path: Path,
    *,
    cli_allow_real_archive_output: bool = False,
) -> ResponseInputs:
    metadata = ensure_mapping(load_json(metadata_path), "metadata")
    body = read_text(body_path, "body")
    target_prefix, response_start, offset_compact, sections, output_mode, real_archive_output = validate_metadata(
        metadata,
        body,
        cli_allow_real_archive_output=cli_allow_real_archive_output,
    )
    scan_no_secret(metadata, body)
    return ResponseInputs(
        metadata_path=metadata_path,
        body_path=body_path,
        metadata=metadata,
        body=body,
        target_prefix=target_prefix,
        response_start=response_start,
        timezone_offset_compact=offset_compact,
        required_sections=sections,
        output_mode=output_mode,
        real_archive_output=real_archive_output,
    )


def validate_out_dir(path: Path, inputs: ResponseInputs | None = None) -> Path:
    raw = str(path)
    if "\x00" in raw:
        raise ResponseWriterError("out-dir contains NUL byte")
    if not path.is_absolute():
        raise ResponseWriterError("out-dir must be absolute")
    if any(part in {"", ".", ".."} for part in path.parts[1:]):
        raise ResponseWriterError("out-dir must not contain empty/current/parent traversal segments")
    resolved = path.resolve(strict=False)
    temp_root = TEMP_ROOT.resolve(strict=True)
    real_root = REAL_RESPONSE_ARCHIVE.resolve(strict=False)
    if inputs is not None and inputs.real_archive_output:
        if resolved != real_root:
            raise ResponseWriterError(f"real archive out-dir must be exactly {REAL_RESPONSE_ARCHIVE}")
        if not REAL_RESPONSE_ARCHIVE.is_dir():
            raise ResponseWriterError(f"real archive directory is unavailable: {REAL_RESPONSE_ARCHIVE}")
        return real_root
    if resolved == real_root or real_root in resolved.parents:
        raise ResponseWriterError("real response archive output requires explicit real-archive mode")
    if resolved != temp_root and temp_root not in resolved.parents:
        raise ResponseWriterError("out-dir must be under /srv/qbuild/tmp")
    return resolved


def response_filename(inputs: ResponseInputs, suffix: int | None = None) -> str:
    stamp = inputs.response_start.strftime("%Y%m%dT%H%M%S")
    base = f"{inputs.target_prefix}_{stamp}{inputs.timezone_offset_compact}_{inputs.metadata['directive_suffix']}"
    if suffix is None:
        return f"{base}.md"
    return f"{base}_r{suffix}.md"


def choose_output_path(out_dir: Path, inputs: ResponseInputs, *, collision: bool) -> Path:
    base = out_dir / response_filename(inputs)
    if not collision:
        if base.exists():
            raise ResponseWriterError("target response file exists and collision handling is disabled")
        return base
    if not base.exists():
        return base
    suffix = 2
    while suffix < 10000:
        candidate = out_dir / response_filename(inputs, suffix)
        if not candidate.exists():
            return candidate
        suffix += 1
    raise ResponseWriterError("collision suffix limit exceeded")


def wrapped_response(inputs: ResponseInputs) -> str:
    metadata = inputs.metadata
    response_end_local = metadata.get("response_end_local", metadata["directive_end_local"])
    response_end_utc = metadata.get("response_end_utc", metadata["directive_end_utc"])
    body = inputs.body.rstrip()
    return "\n".join(
        [
            "================================================================================",
            "CODEX RESPONSE BEGIN",
            "================================================================================",
            f"Response start timestamp (America/Chicago): {metadata['response_start_local']}",
            f"Response start timestamp (UTC): {metadata['response_start_utc']}",
            f"Directive begin timestamp (America/Chicago): {metadata['directive_begin_local']}",
            f"Directive begin timestamp (UTC): {metadata['directive_begin_utc']}",
            f"Directive ID: {metadata['directive_id']}",
            "",
            body,
            "",
            f"Response end timestamp (America/Chicago): {response_end_local}",
            f"Response end timestamp (UTC): {response_end_utc}",
            f"Directive end timestamp (America/Chicago): {metadata['directive_end_local']}",
            f"Directive end timestamp (UTC): {metadata['directive_end_utc']}",
            "================================================================================",
            "CODEX RESPONSE END",
            "================================================================================",
            "",
        ]
    )


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def execute_validate(args: argparse.Namespace) -> ValidationResult:
    inputs = load_inputs(
        Path(args.metadata),
        Path(args.body),
        cli_allow_real_archive_output=bool(args.allow_real_archive_output),
    )
    markers = ["NA0384_VALIDATE_ONLY_NO_WRITE_OK"]
    if inputs.real_archive_output:
        markers = ["NA0386_ALLOW_REAL_ARCHIVE_EXPLICIT_OK", "NA0386_REAL_ARCHIVE_NO_SECRET_SCAN_OK"]
    return ValidationResult(command="validate", ok=True, markers=markers, extra={"output_mode": inputs.output_mode})


def execute_dry_run(args: argparse.Namespace) -> ValidationResult:
    inputs = load_inputs(
        Path(args.metadata),
        Path(args.body),
        cli_allow_real_archive_output=bool(args.allow_real_archive_output),
    )
    out_dir = validate_out_dir(Path(args.out_dir), inputs)
    candidate = choose_output_path(out_dir, inputs, collision=not args.no_collision)
    markers = ["NA0384_DRY_RUN_NO_WRITE_OK"]
    if inputs.real_archive_output:
        markers = ["NA0386_ALLOW_REAL_ARCHIVE_EXPLICIT_OK", "NA0386_REAL_ARCHIVE_PATH_CHECKSUM_OK"]
    return ValidationResult(
        command="dry-run",
        ok=True,
        wrote=False,
        candidate_path=str(candidate),
        filename=candidate.name,
        markers=markers,
        extra={"output_mode": inputs.output_mode},
    )


def execute_write(args: argparse.Namespace) -> ValidationResult:
    inputs = load_inputs(
        Path(args.metadata),
        Path(args.body),
        cli_allow_real_archive_output=bool(args.allow_real_archive_output),
    )
    out_dir = validate_out_dir(Path(args.out_dir), inputs)
    if inputs.real_archive_output:
        if not out_dir.is_dir():
            raise ResponseWriterError(f"real archive out-dir must already exist: {out_dir}")
    else:
        out_dir.mkdir(parents=True, exist_ok=True)
    candidate = choose_output_path(out_dir, inputs, collision=not args.no_collision)
    content = wrapped_response(inputs)
    try:
        with candidate.open("x", encoding="utf-8") as handle:
            handle.write(content)
    except FileExistsError as exc:
        raise ResponseWriterError("target response file exists; refusing overwrite") from exc
    checksum = sha256_file(candidate)
    markers = ["NA0384_RESPONSE_WRITER_HELPER_OK"]
    if inputs.real_archive_output:
        markers = [
            "NA0386_REAL_ARCHIVE_WRITE_HELPER_OK",
            "NA0386_ALLOW_REAL_ARCHIVE_EXPLICIT_OK",
            "NA0386_REAL_ARCHIVE_NO_SECRET_SCAN_OK",
            "NA0386_REAL_ARCHIVE_PATH_CHECKSUM_OK",
            "NA0386_NO_INDEX_MUTATION_OK",
            "NA0386_NO_DELETE_OK",
        ]
    return ValidationResult(
        command="write",
        ok=True,
        wrote=True,
        output_path=str(candidate),
        candidate_path=str(candidate),
        filename=candidate.name,
        sha256=checksum,
        markers=markers,
        extra={"output_mode": inputs.output_mode},
    )


def now_chicago() -> tuple[str, str, str]:
    if ZoneInfo is None:
        raise ResponseWriterError("zoneinfo module unavailable")
    local = datetime.now(ZoneInfo("America/Chicago")).replace(microsecond=0)
    utc = local.astimezone(timezone.utc)
    return local.isoformat(), utc.isoformat().replace("+00:00", "Z"), format_offset(local)


def execute_template(args: argparse.Namespace) -> ValidationResult:
    target = canonical_target_na(args.target)
    suffix = validate_directive_suffix(args.directive_suffix)
    directive_id = validate_directive_id(args.directive_id)
    local_ts, utc_ts, offset = now_chicago()
    metadata = {
        "schema_version": SCHEMA_VERSION,
        "target_na": target,
        "directive_suffix": suffix,
        "directive_id": directive_id,
        "response_start_local": local_ts,
        "response_start_utc": utc_ts,
        "directive_begin_local": local_ts,
        "directive_begin_utc": utc_ts,
        "directive_end_local": local_ts,
        "directive_end_utc": utc_ts,
        "timezone": "America/Chicago",
        "timezone_offset": offset,
        "output_mode": "temp-output",
        "required_sections": REQUIRED_NA_SECTIONS,
        "allow_real_archive_output": False,
        "no_secret_required": True,
    }
    body = "\n\n".join(f"{section}\n- TODO" for section in REQUIRED_NA_SECTIONS)
    return ValidationResult(command="template", ok=True, extra={"metadata": metadata, "body": body})


def _fixture_path(fixture_dir: Path, name: str) -> Path:
    path = fixture_dir / name
    if path.resolve(strict=False).parent != fixture_dir.resolve(strict=True):
        raise ResponseWriterError(f"fixture path traversal rejected: {name}")
    return path


def _case_args(case: dict[str, Any], fixture_dir: Path, tmp_dir: Path) -> argparse.Namespace:
    operation = ensure_string(case.get("operation"), "case.operation")
    metadata = _fixture_path(fixture_dir, ensure_string(case.get("metadata"), "case.metadata"))
    body = _fixture_path(fixture_dir, ensure_string(case.get("body"), "case.body"))
    out_dir_value = case.get("out_dir")
    if isinstance(out_dir_value, str) and out_dir_value.startswith("/"):
        out_dir = out_dir_value
    elif isinstance(out_dir_value, str):
        out_dir = str(tmp_dir / out_dir_value)
    else:
        out_dir = str(tmp_dir / ensure_string(case.get("name"), "case.name") / "out")
    return argparse.Namespace(
        metadata=str(metadata),
        body=str(body),
        out_dir=out_dir,
        no_collision=bool(case.get("no_collision", False)),
        allow_real_archive_output=bool(case.get("cli_allow_real_archive_output", False)),
        operation=operation,
    )


def _precreate_collisions(case: dict[str, Any], fixture_args: argparse.Namespace) -> None:
    precreate = case.get("precreate", [])
    if not precreate:
        return
    inputs = load_inputs(
        Path(fixture_args.metadata),
        Path(fixture_args.body),
        cli_allow_real_archive_output=bool(fixture_args.allow_real_archive_output),
    )
    out_dir = validate_out_dir(Path(fixture_args.out_dir), inputs)
    out_dir.mkdir(parents=True, exist_ok=True)
    for item in precreate:
        if item == "base":
            name = response_filename(inputs)
        elif item == "r2":
            name = response_filename(inputs, 2)
        else:
            raise ResponseWriterError(f"unsupported precreate collision target: {item}")
        path = out_dir / name
        with path.open("x", encoding="utf-8") as handle:
            handle.write(
                "================================================================================\n"
                "CODEX RESPONSE BEGIN\n"
                "================================================================================\n"
                "preexisting collision fixture\n"
                "================================================================================\n"
                "CODEX RESPONSE END\n"
                "================================================================================\n"
            )


def execute_fixture(args: argparse.Namespace) -> ValidationResult:
    fixture_dir = Path(args.fixture_dir).resolve(strict=True)
    tmp_dir = validate_out_dir(Path(args.tmp_dir))
    tmp_dir.mkdir(parents=True, exist_ok=True)
    log_path = tmp_dir / "fixture_matrix.log"
    matrix = ensure_mapping(load_json(fixture_dir / "fixture_cases.json"), "fixture_cases")
    if matrix.get("schema_version") != FIXTURE_SCHEMA_VERSION:
        raise ResponseWriterError(f"fixture matrix schema_version must be {FIXTURE_SCHEMA_VERSION}")
    cases = matrix.get("cases")
    if not isinstance(cases, list) or not cases:
        raise ResponseWriterError("fixture matrix cases must be a non-empty list")

    all_markers: list[str] = []
    lines: list[str] = []
    pass_count = 0
    for case in cases:
        case_map = ensure_mapping(case, "fixture case")
        name = ensure_string(case_map.get("name"), "case.name")
        expect_success = bool(case_map.get("expect_success", False))
        fixture_args = _case_args(case_map, fixture_dir, tmp_dir)
        try:
            _precreate_collisions(case_map, fixture_args)
            if fixture_args.operation == "write":
                result = execute_write(fixture_args)
            elif fixture_args.operation == "dry-run":
                result = execute_dry_run(fixture_args)
            elif fixture_args.operation == "validate":
                result = execute_validate(fixture_args)
            else:
                raise ResponseWriterError(f"unsupported fixture operation: {fixture_args.operation}")
            if not expect_success:
                raise ResponseWriterError("case unexpectedly succeeded")
            expected_suffix = case_map.get("expected_filename_suffix")
            if expected_suffix and not (result.filename or "").endswith(str(expected_suffix)):
                raise ResponseWriterError(f"filename {result.filename} did not end with {expected_suffix}")
            if case_map.get("expect_write", False) and not result.output_path:
                raise ResponseWriterError("expected write did not produce output_path")
            if case_map.get("expect_no_write", False) and result.output_path:
                raise ResponseWriterError("expected no-write produced output_path")
            if case_map.get("assert_wrapper", False) and result.output_path:
                text = Path(result.output_path).read_text(encoding="utf-8")
                if "CODEX RESPONSE BEGIN" not in text or "CODEX RESPONSE END" not in text:
                    raise ResponseWriterError("wrapper missing from written response")
            if case_map.get("assert_json_summary", False):
                json.loads(json.dumps(result.as_dict(), sort_keys=True))
            if case_map.get("assert_sha256", False) and not result.sha256:
                raise ResponseWriterError("expected sha256 summary was absent")
            markers = [str(item) for item in case_map.get("markers", [])]
            all_markers.extend(markers)
            lines.append(f"CASE {name} PASS expected_success markers={','.join(markers)}")
            pass_count += 1
        except ResponseWriterError as exc:
            if expect_success:
                lines.append(f"CASE {name} FAIL {exc}")
                raise ResponseWriterError(f"fixture case failed: {name}: {exc}") from exc
            markers = [str(item) for item in case_map.get("markers", [])]
            all_markers.extend(markers)
            lines.append(f"CASE {name} PASS expected_failure error={exc} markers={','.join(markers)}")
            pass_count += 1
    unique_markers = sorted(set(all_markers))
    lines.append(f"FIXTURE_SUMMARY pass={pass_count} total={len(cases)}")
    for marker in unique_markers:
        lines.append(f"MARKER {marker}")
    try:
        with log_path.open("x", encoding="utf-8") as handle:
            handle.write("\n".join(lines) + "\n")
    except FileExistsError as exc:
        raise ResponseWriterError(f"fixture log already exists: {log_path}") from exc
    print("\n".join(lines))
    return ValidationResult(
        command="fixture",
        ok=True,
        wrote=True,
        output_path=str(log_path),
        candidate_path=str(log_path),
        filename=log_path.name,
        markers=unique_markers,
    )


def emit_result(result: ValidationResult, *, json_mode: bool) -> None:
    if json_mode:
        print(json.dumps(result.as_dict(), indent=2, sort_keys=True))
        return
    if result.ok:
        detail = f" path={result.output_path}" if result.output_path else ""
        candidate = f" candidate={result.candidate_path}" if result.candidate_path and not result.output_path else ""
        checksum = f" sha256={result.sha256}" if result.sha256 else ""
        mode = ""
        if result.extra and result.extra.get("output_mode") == REAL_ARCHIVE_SMOKE_MODE:
            mode = " mode=real_archive_smoke"
        print(f"OK command={result.command} wrote={str(result.wrote).lower()}{mode}{detail}{candidate}{checksum}")
        for marker in result.markers:
            print(marker)
    else:
        print(f"ERROR command={result.command} errors={'; '.join(result.errors)}", file=sys.stderr)


def run_command(args: argparse.Namespace) -> ValidationResult:
    if args.command == "write":
        return execute_write(args)
    if args.command == "dry-run":
        return execute_dry_run(args)
    if args.command == "validate":
        return execute_validate(args)
    if args.command == "template":
        return execute_template(args)
    if args.command == "fixture":
        return execute_fixture(args)
    raise ResponseWriterError(f"unknown command: {args.command}")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="QSL Codex response writer harness.")
    sub = parser.add_subparsers(dest="command", required=True)

    def add_common_io(p: argparse.ArgumentParser, *, out_dir: bool) -> None:
        p.add_argument("--metadata", required=True, help="Response metadata JSON path.")
        p.add_argument("--body", required=True, help="Response body markdown path.")
        if out_dir:
            p.add_argument(
                "--out-dir",
                required=True,
                help="Authorized temp output directory under /srv/qbuild/tmp or exact real response archive when gated.",
            )
            p.add_argument("--no-collision", action="store_true", help="Reject instead of choosing _r2/_r3 on collision.")
        p.add_argument(
            "--allow-real-archive-output",
            action="store_true",
            help="Explicit gate required for output_mode=real_archive_smoke.",
        )
        p.add_argument("--json", action="store_true", help="Emit JSON summary.")

    write = sub.add_parser(
        "write",
        help="Validate and write one response file under /srv/qbuild/tmp, or the exact real archive with dual gates.",
    )
    add_common_io(write, out_dir=True)

    dry_run = sub.add_parser("dry-run", help="Validate and compute output path without writing.")
    add_common_io(dry_run, out_dir=True)

    validate = sub.add_parser("validate", help="Validate metadata/body without writing.")
    add_common_io(validate, out_dir=False)

    template = sub.add_parser("template", help="Emit a response metadata/body template.")
    template.add_argument("--target", required=True)
    template.add_argument("--directive-suffix", required=True)
    template.add_argument("--directive-id", required=True)
    template.add_argument("--json", action="store_true", help="Accepted for CLI symmetry; template output is JSON.")

    fixture = sub.add_parser("fixture", help="Run deterministic no-network fixture matrix.")
    fixture.add_argument("--fixture-dir", required=True)
    fixture.add_argument("--tmp-dir", required=True)
    fixture.add_argument("--json", action="store_true", help="Emit JSON summary after fixture log.")

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    json_mode = bool(getattr(args, "json", False))
    try:
        result = run_command(args)
        emit_result(result, json_mode=json_mode)
        return EXIT_OK
    except ResponseWriterError as exc:
        result = ValidationResult(command=getattr(args, "command", "unknown"), ok=False, errors=[str(exc)])
        emit_result(result, json_mode=json_mode)
        return EXIT_VALIDATION
    except Exception as exc:  # pragma: no cover - last-resort fail-closed guard.
        result = ValidationResult(command=getattr(args, "command", "unknown"), ok=False, errors=[f"internal error: {exc}"])
        emit_result(result, json_mode=json_mode)
        return EXIT_INTERNAL


if __name__ == "__main__":
    raise SystemExit(main())
