#!/usr/bin/env python3
import argparse
import http.client
import json
import os
import re
import socket
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import PurePosixPath


DENYLIST_SUFFIXES = (".pem", ".key", ".p12")
ROOT_DOC_SCAN_NAMES = {
    "README.md",
    "START_HERE.md",
    "LICENSE",
    "SECURITY.md",
    "CONTRIBUTING.md",
    "THIRD_PARTY_NOTICES.md",
    "CODE_OF_CONDUCT.md",
    "SUPPORT.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "NEXT_ACTIONS.md",
}
HIGH_CONF_PATTERN = re.compile(
    r"(-----BEGIN (OPENSSH |RSA |EC |DSA )?PRIVATE KEY-----|"
    r"-----BEGIN PRIVATE KEY-----|BEGIN OPENSSH PRIVATE KEY|AKIA[0-9A-Z]{16}|"
    r"ASIA[0-9A-Z]{16}|ghp_[A-Za-z0-9]{36}|github_pat_[A-Za-z0-9_]{20,}|"
    r"glpat-[A-Za-z0-9_-]{20,}|xox[baprs]-[A-Za-z0-9-]{10,}|"
    r"AIza[0-9A-Za-z_-]{35}|ya29\.[0-9A-Za-z_-]+|"
    r"Authorization:\s*Bearer\s+[A-Za-z0-9_.=\-]{10,}|"
    r"x-api-key\s*[:=]\s*[A-Za-z0-9_-]{10,})"
)
MARKDOWN_LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")
SELF_REPAIR_BOOTSTRAP_ROOT_PATHS = {
    ".github/workflows/public-ci.yml",
    "scripts/ci/public_safety_gate.py",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
}
SELF_REPAIR_BOOTSTRAP_TESTPLAN_RE = re.compile(
    r"^tests/NA-[0-9A-Za-z-]+_public_safety_.*\.md$"
)
ACCEPTED_CHECK_CONCLUSIONS = {"success", "neutral", "skipped"}
REQUIRED_CONTEXT_NEUTRAL_ALLOWED = {"CodeQL"}
TRANSIENT_WAIT_HTTP_CODES = {502, 503, 504}
RED_MAIN_REPAIR_PROFILES = {
    "send_commit_vault_mock_provider_retired": {
        "failure_check": "macos-qsc-full-serial",
        "required_markers": ("send_commit", "vault_mock_provider_retired"),
        "required_paths": ("qsl/qsl-client/qsc/tests/send_commit.rs",),
        "kt_blocked_prefixes": (
            "tools/refimpl/quantumshield_refimpl/src/kt/",
            "tools/refimpl/quantumshield_refimpl/tests/kt_",
            "inputs/suite2/vectors/qshield_suite2_kt_",
        ),
    }
}


class TransientGitHubPollingError(RuntimeError):
    """A bounded wait-only GitHub polling error that is safe to retry."""


ANSI_RE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
RUSTSEC_ID_RE = re.compile(r"\bRUSTSEC-\d{4}-\d{4}\b", re.I)
ADVISORIES_REAL_FINDING_RE = re.compile(
    r"(?im)^\s*(?:error|warning):\s+\d+\s+"
    r"(?:vulnerabilit(?:y|ies)|warnings?|advisories?)\s+found\b"
)
ADVISORIES_REAL_DETAIL_RE = re.compile(
    r"(?im)^\s*(?:Crate|Version|Title|Date|ID|URL|Solution|Dependency tree):\s+"
)
ADVISORIES_TRANSIENT_CONTEXT_MARKERS = (
    "couldn't fetch advisory database",
    "could not fetch advisory database",
    "failed to fetch advisory database",
    "failed fetching advisory database",
    "failed to update advisory database",
    "fetching advisory database from",
    "rustsec/advisory-db.git",
    "advisory-db.git",
)
ADVISORIES_TRANSIENT_NETWORK_MARKERS = (
    "an io error occurred when talking to the server",
    "git operation failed",
    "error sending request",
    "network error",
    "network failure",
    "timed out",
    "timeout",
    "connection timed out",
    "connection reset",
    "connection refused",
    "failed to connect",
    "temporarily unavailable",
    "temporary failure",
    "early eof",
    "http/2",
    "stream error",
    "tls",
    "ssl",
    "502",
    "503",
    "504",
)


def strip_ansi(text: str) -> str:
    return ANSI_RE.sub("", text)


def classify_advisories_output(exit_code: int, output: str) -> tuple[str, str]:
    """Classify cargo-audit output without ever treating failures as success."""
    if exit_code == 0:
        return "clean_success", "cargo_audit_exit_zero"

    normalized = strip_ansi(output).lower()
    plain = strip_ansi(output)

    if RUSTSEC_ID_RE.search(plain):
        return "real_finding", "rustsec_advisory_id_present"
    if ADVISORIES_REAL_FINDING_RE.search(plain):
        return "real_finding", "cargo_audit_finding_summary_present"
    if ADVISORIES_REAL_DETAIL_RE.search(plain):
        return "real_finding", "cargo_audit_finding_detail_present"

    has_advisory_fetch_context = any(
        marker in normalized for marker in ADVISORIES_TRANSIENT_CONTEXT_MARKERS
    )
    has_network_marker = any(
        marker in normalized for marker in ADVISORIES_TRANSIENT_NETWORK_MARKERS
    )
    if has_advisory_fetch_context and has_network_marker:
        return "transient_fetch", "advisory_database_network_fetch_failure"

    return "unknown_failure", "unrecognized_cargo_audit_failure"


def append_advisories_record(output_path: str, text: str) -> None:
    with open(output_path, "a", encoding="utf-8") as handle:
        handle.write(text)
        if text and not text.endswith("\n"):
            handle.write("\n")


def emit_advisories_marker(output_path: str, marker: str) -> None:
    print(marker)
    append_advisories_record(output_path, marker)


def classify_advisories_output_cmd(args: argparse.Namespace) -> int:
    output = open(args.input, "r", encoding="utf-8", errors="replace").read()
    classification, reason = classify_advisories_output(args.exit_code, output)
    print(f"ADVISORIES_CLASSIFICATION={classification}")
    print(f"ADVISORIES_REASON={reason}")
    return 0


def run_cargo_audit_with_resilience(args: argparse.Namespace) -> int:
    if args.max_retries < 0 or args.max_retries > 2:
        print("ERROR: --max-retries must be between 0 and 2", file=sys.stderr)
        return 2
    if args.retry_delay_seconds < 0:
        print("ERROR: --retry-delay-seconds must be non-negative", file=sys.stderr)
        return 2
    command = list(args.command)
    if command and command[0] == "--":
        command = command[1:]
    if not command:
        print("ERROR: cargo audit command is required after --", file=sys.stderr)
        return 2

    os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
    with open(args.output, "w", encoding="utf-8") as handle:
        handle.write("")

    max_attempts = args.max_retries + 1
    saw_transient = False
    last_rc = 1
    for attempt in range(1, max_attempts + 1):
        emit_advisories_marker(
            args.output,
            f"ADVISORIES_AUDIT_ATTEMPT attempt={attempt} max_attempts={max_attempts}",
        )
        proc = subprocess.run(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )
        last_rc = proc.returncode
        if proc.stdout:
            print(proc.stdout, end="")
            append_advisories_record(args.output, proc.stdout)

        classification, reason = classify_advisories_output(last_rc, proc.stdout or "")
        emit_advisories_marker(
            args.output,
            f"ADVISORIES_CLASSIFICATION={classification} reason={reason} attempt={attempt}",
        )

        if classification == "clean_success":
            if saw_transient:
                emit_advisories_marker(
                    args.output,
                    f"ADVISORIES_TRANSIENT_FETCH_RETRY_OK attempt={attempt}",
                )
            return 0
        if classification == "real_finding":
            emit_advisories_marker(
                args.output,
                f"ADVISORIES_REAL_FINDING_FAIL_CLOSED attempt={attempt}",
            )
            return last_rc if last_rc else 1
        if classification == "unknown_failure":
            emit_advisories_marker(
                args.output,
                f"ADVISORIES_UNKNOWN_FAIL_CLOSED attempt={attempt}",
            )
            return last_rc if last_rc else 1

        saw_transient = True
        if attempt < max_attempts:
            emit_advisories_marker(
                args.output,
                f"ADVISORIES_RETRY_TRANSIENT_FETCH attempt={attempt} max_retries={args.max_retries}",
            )
            if args.retry_delay_seconds:
                time.sleep(args.retry_delay_seconds)
            continue
        emit_advisories_marker(
            args.output,
            f"ADVISORIES_TRANSIENT_FETCH_FAIL_CLOSED attempts={attempt}",
        )
        return last_rc if last_rc else 1

    return last_rc if last_rc else 1


def github_api_base() -> str:
    return os.environ.get("GITHUB_API_URL", "https://api.github.com").rstrip("/")


def github_token() -> str:
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if not token:
        raise SystemExit("ERROR: GITHUB_TOKEN or GH_TOKEN is required")
    return token


def api_url(path: str, query: dict[str, str] | None = None) -> str:
    url = f"{github_api_base()}{path}"
    if query:
        url = f"{url}?{urllib.parse.urlencode(query)}"
    return url


def header_value(headers, name: str) -> str:
    value = headers.get(name)
    if value is None:
        return ""
    return str(value)


def transient_http_error_reason(code: int, headers, body_text: str) -> str | None:
    if code in TRANSIENT_WAIT_HTTP_CODES:
        return f"HTTP {code}"
    lower_body = body_text.lower()
    retry_after = header_value(headers, "Retry-After")
    remaining = header_value(headers, "X-RateLimit-Remaining")
    rate_limit_markers = (
        "rate limit",
        "secondary limit",
        "secondary rate limit",
        "abuse detection",
        "temporarily unavailable",
        "please retry",
    )
    if code in (403, 429) and (
        code == 429
        or retry_after
        or remaining == "0"
        or any(marker in lower_body for marker in rate_limit_markers)
    ):
        return f"HTTP {code} rate-limit/secondary-limit"
    return None


def looks_like_html(body: bytes, content_type: str) -> bool:
    if "html" in content_type.lower():
        return True
    prefix = body[:512].lstrip().lower()
    return prefix.startswith(b"<!doctype html") or prefix.startswith(b"<html")


def github_request(
    url: str,
    *,
    accept: str = "application/vnd.github+json",
    method: str = "GET",
) -> tuple[bytes, urllib.response.addinfourl]:
    req = urllib.request.Request(
        url,
        method=method,
        headers={
            "Accept": accept,
            "Authorization": f"Bearer {github_token()}",
            "User-Agent": "qsl-public-safety-gate",
        },
    )
    try:
        resp = urllib.request.urlopen(req)
        return resp.read(), resp
    except urllib.error.HTTPError as exc:
        body = exc.read().decode("utf-8", errors="replace")
        raise SystemExit(f"ERROR: GitHub API {exc.code} for {url}\n{body}") from exc


def github_get(path: str, query: dict[str, str] | None = None) -> dict:
    body, _ = github_request(api_url(path, query=query))
    return json.loads(body)


def github_get_for_wait(path: str, query: dict[str, str] | None = None) -> dict:
    url = api_url(path, query=query)
    req = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {github_token()}",
            "User-Agent": "qsl-public-safety-gate",
        },
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            body = resp.read()
            content_type = resp.headers.get("Content-Type", "")
    except urllib.error.HTTPError as exc:
        raw_body = exc.read()
        body_text = raw_body.decode("utf-8", errors="replace")
        reason = transient_http_error_reason(exc.code, exc.headers, body_text)
        if reason:
            raise TransientGitHubPollingError(reason) from exc
        raise SystemExit(f"ERROR: GitHub API {exc.code} for {url}\n{body_text}") from exc
    except (urllib.error.URLError, TimeoutError, ConnectionResetError, socket.timeout) as exc:
        raise TransientGitHubPollingError(
            f"temporary network error: {type(exc).__name__}"
        ) from exc
    except (http.client.RemoteDisconnected, OSError) as exc:
        raise TransientGitHubPollingError(
            f"temporary connection error: {type(exc).__name__}"
        ) from exc

    try:
        return json.loads(body)
    except json.JSONDecodeError as exc:
        kind = "HTML" if looks_like_html(body, content_type) else "non-JSON"
        raise TransientGitHubPollingError(f"{kind} response from GitHub API") from exc


def github_get_bytes(
    path: str,
    query: dict[str, str] | None = None,
    *,
    accept: str = "application/vnd.github.raw",
) -> bytes:
    body, _ = github_request(api_url(path, query=query), accept=accept)
    return body


def latest_run_for_name(check_runs: list[dict], name: str) -> dict | None:
    candidates = [run for run in check_runs if run.get("name") == name]
    if not candidates:
        return None
    return max(candidates, key=lambda run: run.get("id", 0))


def branch_head_sha(repo: str, branch: str) -> str:
    data = github_get(f"/repos/{repo}/branches/{branch}")
    return data["commit"]["sha"]


def commit_check_runs(repo: str, sha: str) -> list[dict]:
    data = github_get(
        f"/repos/{repo}/commits/{sha}/check-runs",
        {"per_page": "100"},
    )
    return data.get("check_runs", [])


def commit_check_runs_for_wait(repo: str, sha: str) -> list[dict]:
    data = github_get_for_wait(
        f"/repos/{repo}/commits/{sha}/check-runs",
        {"per_page": "100"},
    )
    return data.get("check_runs", [])


def pull_request(repo: str, number: int) -> dict:
    return github_get(f"/repos/{repo}/pulls/{number}")


def pull_request_files(repo: str, number: int) -> list[dict]:
    files: list[dict] = []
    page = 1
    while True:
        batch = github_get(
            f"/repos/{repo}/pulls/{number}/files",
            {"per_page": "100", "page": str(page)},
        )
        if not batch:
            break
        files.extend(batch)
        page += 1
    return files


def repo_file_bytes(repo: str, ref: str, path: str) -> bytes:
    quoted = urllib.parse.quote(path, safe="/")
    return github_get_bytes(
        f"/repos/{repo}/contents/{quoted}",
        {"ref": ref},
    )


def repo_file_text(repo: str, ref: str, path: str) -> str:
    return repo_file_bytes(repo, ref, path).decode("utf-8", errors="replace")


def repo_path_exists(repo: str, ref: str, path: str) -> bool:
    quoted = urllib.parse.quote(path, safe="/")
    url = api_url(f"/repos/{repo}/contents/{quoted}", {"ref": ref})
    req = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {github_token()}",
            "User-Agent": "qsl-public-safety-gate",
        },
    )
    try:
        with urllib.request.urlopen(req):
            return True
    except urllib.error.HTTPError as exc:
        if exc.code == 404:
            return False
        body = exc.read().decode("utf-8", errors="replace")
        raise SystemExit(f"ERROR: GitHub API {exc.code} for {url}\n{body}") from exc


def normalize_repo_path(path: str) -> str | None:
    parts: list[str] = []
    for part in PurePosixPath(path).parts:
        if part in ("", ".", "/"):
            continue
        if part == "..":
            if not parts:
                return None
            parts.pop()
            continue
        parts.append(part)
    return "/".join(parts)


def resolve_repo_target(source_path: str, target: str) -> str | None:
    if target.startswith("/"):
        return normalize_repo_path(target.lstrip("/"))
    joined = str(PurePosixPath(source_path).parent / target)
    return normalize_repo_path(joined)


def strip_code_blocks(text: str) -> str:
    return re.sub(r"```.*?```", "", text, flags=re.S)


def markdown_paths_for_scan(files: list[dict]) -> list[str]:
    result: list[str] = []
    for file_info in files:
        status = file_info.get("status")
        filename = file_info["filename"]
        if status == "removed":
            continue
        if filename == "README.md" or (
            filename.startswith("docs/") and filename.endswith(".md")
        ):
            result.append(filename)
    return sorted(result)


def root_doc_scan_path(path: str) -> bool:
    return path in ROOT_DOC_SCAN_NAMES


def content_scan_paths(files: list[dict]) -> list[str]:
    result: list[str] = []
    for file_info in files:
        status = file_info.get("status")
        filename = file_info["filename"]
        if status == "removed":
            continue
        if filename.startswith("docs/") or filename.startswith("inputs/") or root_doc_scan_path(
            filename
        ):
            result.append(filename)
    return sorted(result)


def vector_paths(files: list[dict]) -> list[str]:
    result: list[str] = []
    for file_info in files:
        status = file_info.get("status")
        filename = file_info["filename"]
        if status == "removed":
            continue
        if filename.startswith("inputs/suite2/vectors/") and filename.endswith(".json"):
            result.append(filename)
    return sorted(result)


def denylist_hits(files: list[dict]) -> list[str]:
    hits: list[str] = []
    for file_info in files:
        status = file_info.get("status")
        filename = file_info["filename"]
        if status == "removed":
            continue
        basename = PurePosixPath(filename).name
        if basename == ".env" or basename.endswith(DENYLIST_SUFFIXES):
            hits.append(filename)
    return sorted(hits)


def dependency_remediation_paths(files: list[dict]) -> list[str]:
    paths: list[str] = []
    for file_info in files:
        status = file_info.get("status")
        filename = file_info["filename"]
        if status == "removed":
            continue
        basename = PurePosixPath(filename).name
        if filename == "Cargo.lock" or basename == "Cargo.toml":
            paths.append(filename)
    return sorted(paths)


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


def github_job_log_text(repo: str, job_id: int) -> str:
    url = api_url(f"/repos/{repo}/actions/jobs/{job_id}/logs")
    req = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {github_token()}",
            "User-Agent": "qsl-public-safety-gate",
        },
    )
    opener = urllib.request.build_opener(NoRedirect)
    try:
        resp = opener.open(req)
        body = resp.read()
    except urllib.error.HTTPError as exc:
        if exc.code not in (301, 302, 303, 307, 308):
            body = exc.read().decode("utf-8", errors="replace")
            raise SystemExit(f"ERROR: GitHub API {exc.code} for {url}\n{body}") from exc
        location = exc.headers.get("Location")
        if not location:
            raise SystemExit(f"ERROR: GitHub API redirect for {url} had no Location") from exc
        unsigned_req = urllib.request.Request(
            location,
            headers={"User-Agent": "qsl-public-safety-gate"},
        )
        with urllib.request.urlopen(unsigned_req) as redirected:
            body = redirected.read()
    return body.decode("utf-8", errors="replace")


def branch_required_checks(repo: str, branch: str) -> tuple[list[str], dict[str, int]]:
    data = github_get(f"/repos/{repo}/branches/{branch}/protection/required_status_checks")
    contexts = list(data.get("contexts") or [])
    app_ids: dict[str, int] = {}
    for check in data.get("checks") or []:
        context = check.get("context")
        app_id = check.get("app_id")
        if context and app_id is not None:
            app_ids[context] = int(app_id)
    return contexts, app_ids


def check_completed_non_failing(run: dict | None) -> bool:
    return bool(
        run
        and run.get("status") == "completed"
        and run.get("conclusion") in ACCEPTED_CHECK_CONCLUSIONS
    )


def check_completed_success(run: dict | None) -> bool:
    return bool(run and run.get("status") == "completed" and run.get("conclusion") == "success")


def check_completed_failure(run: dict | None) -> bool:
    return bool(run and run.get("status") == "completed" and run.get("conclusion") == "failure")


def latest_run_map(check_runs: list[dict]) -> dict[str, dict]:
    mapped: dict[str, dict] = {}
    for run in check_runs:
        name = run.get("name")
        if not name:
            continue
        current = mapped.get(name)
        if current is None or run.get("id", 0) > current.get("id", 0):
            mapped[name] = run
    return mapped


def next_actions_entries(text: str) -> list[dict]:
    matches = list(
        re.finditer(r"^### (NA-[0-9A-Za-z-]+)\s+(?:-|\N{EM DASH})\s+(.+?)\s*$", text, re.M)
    )
    entries: list[dict] = []
    for index, match in enumerate(matches):
        start = match.end()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        body = text[start:end]
        status_match = re.search(r"^Status:\s+([A-Z]+)\s*$", body, re.M)
        status = status_match.group(1) if status_match else ""
        entries.append(
            {
                "id": match.group(1),
                "title": match.group(2),
                "status": status,
                "body": body,
            }
        )
    return entries


def active_ready_entry(text: str) -> tuple[dict | None, list[str]]:
    entries = next_actions_entries(text)
    ready = [entry for entry in entries if entry.get("status") == "READY"]
    return (ready[0] if len(ready) == 1 else None, [entry["id"] for entry in ready])


def scope_section_text(body: str) -> str:
    lines = body.splitlines()
    in_scope = False
    collected: list[str] = []
    stop_prefixes = (
        "Must protect:",
        "Deliverables:",
        "Acceptance:",
        "Objective:",
        "Notes:",
        "Required:",
    )
    for line in lines:
        stripped = line.strip()
        if stripped == "Scope:":
            in_scope = True
            continue
        if in_scope and stripped in stop_prefixes:
            break
        if in_scope:
            collected.append(line)
    return "\n".join(collected)


def active_scope_paths(entry: dict) -> list[str]:
    paths: set[str] = set()
    for raw in re.findall(r"`([^`]+)`", scope_section_text(entry.get("body", ""))):
        token = raw.strip()
        if not token or " " in token:
            continue
        if token.startswith("#"):
            continue
        if token.endswith("/**") or "/" in token or token.startswith(".") or "." in PurePosixPath(token).name:
            paths.add(token)
    return sorted(paths)


def path_allowed_by_scope(path: str, allowed: list[str]) -> bool:
    for item in allowed:
        if item.endswith("/**"):
            prefix = item[:-3]
            if path == prefix or path.startswith(prefix + "/"):
                return True
            continue
        if path == item:
            return True
    return False


def red_main_profile(name: str) -> dict:
    profile = RED_MAIN_REPAIR_PROFILES.get(name)
    if profile is None:
        known = ", ".join(sorted(RED_MAIN_REPAIR_PROFILES))
        raise SystemExit(f"ERROR: unknown red-main repair profile '{name}' (known: {known})")
    return profile


def validate_red_main_repair_evidence(
    evidence: dict,
    *,
    profile_name: str,
    pr_number: int,
    expected_sha: str | None,
    expected_markers: list[str],
    expected_active_ready_na: str | None = None,
) -> list[str]:
    errors: list[str] = []
    profile = red_main_profile(profile_name)
    profile_markers = list(profile["required_markers"])
    if not expected_markers:
        errors.append("at least one expected main-failure marker is required")
    missing_profile_markers = [marker for marker in profile_markers if marker not in expected_markers]
    if missing_profile_markers:
        errors.append(
            "expected marker set does not include profile markers: "
            + ",".join(missing_profile_markers)
        )

    required_contexts = list(evidence.get("required_contexts") or [])
    if "public-safety" not in required_contexts:
        errors.append("branch protection required checks do not include public-safety")

    active = evidence.get("active_ready")
    ready_items = list(evidence.get("ready_items") or [])
    if len(ready_items) != 1 or not active:
        errors.append(f"queue proof does not show exactly one READY item: {','.join(ready_items)}")
    elif expected_active_ready_na and active.get("id") != expected_active_ready_na:
        errors.append(
            f"active READY {active.get('id')} does not match expected {expected_active_ready_na}"
        )

    main_checks = evidence.get("main_checks") or {}
    public_safety = main_checks.get("public-safety")
    if not check_completed_failure(public_safety):
        errors.append("latest main public-safety is not completed/failure")
    advisories = main_checks.get("advisories")
    if not check_completed_non_failing(advisories):
        errors.append("latest main advisories is missing or failing")

    failure_check = str(profile["failure_check"])
    failure_run = main_checks.get(failure_check)
    if not check_completed_failure(failure_run):
        errors.append(f"latest main {failure_check} is not completed/failure")
    failure_log = str((failure_run or {}).get("log", ""))
    missing_markers = [marker for marker in expected_markers if marker not in failure_log]
    if missing_markers:
        errors.append("latest main failure log is missing markers: " + ",".join(missing_markers))

    pr = evidence.get("pr") or {}
    if pr.get("number") != pr_number:
        errors.append(f"PR number {pr.get('number')} does not match expected {pr_number}")
    if pr.get("state") != "open" or pr.get("merged"):
        errors.append("target PR is not open/unmerged")
    if pr.get("base") != "main":
        errors.append("target PR base is not main")
    if expected_sha and pr.get("head_sha") != expected_sha:
        errors.append(f"target PR head {pr.get('head_sha')} does not match expected {expected_sha}")

    pr722 = evidence.get("pr722") or {}
    if pr722.get("number") != 722 or pr722.get("state") != "closed" or pr722.get("merged"):
        errors.append("PR #722 is not closed/unmerged")
    if pr.get("number") == 722:
        errors.append("PR #722 cannot be used as the repair admission target")

    scope_paths = list((active or {}).get("scope_paths") or [])
    changed_paths = sorted(pr.get("files") or [])
    if not changed_paths:
        errors.append("target PR has no changed files")
    disallowed = [path for path in changed_paths if not path_allowed_by_scope(path, scope_paths)]
    if disallowed:
        errors.append("target PR changes paths outside active READY scope: " + ",".join(disallowed))
    missing_required_paths = [
        path for path in profile["required_paths"] if path not in changed_paths
    ]
    if missing_required_paths:
        errors.append(
            "target PR is missing profile-required repair paths: "
            + ",".join(missing_required_paths)
        )

    kt_blocked = []
    for path in changed_paths:
        for prefix in profile["kt_blocked_prefixes"]:
            if path.startswith(prefix):
                kt_blocked.append(path)
    if kt_blocked and not any(path_allowed_by_scope(path, scope_paths) for path in kt_blocked):
        errors.append("target PR touches KT/#708 paths outside active scope: " + ",".join(kt_blocked))

    pr_checks = pr.get("checks") or {}
    app_ids = evidence.get("required_app_ids") or {}
    for context in required_contexts:
        if context == "public-safety":
            continue
        run = pr_checks.get(context)
        if run is None:
            errors.append(f"target PR head is missing required check {context}")
            continue
        conclusion = run.get("conclusion")
        accepted = conclusion == "success" or (
            context in REQUIRED_CONTEXT_NEUTRAL_ALLOWED and conclusion == "neutral"
        )
        if run.get("status") != "completed" or not accepted:
            errors.append(f"target PR required check {context} is not accepted")
        expected_app = app_ids.get(context)
        observed_app = (run.get("app") or {}).get("id")
        if expected_app is not None and observed_app is not None and int(observed_app) != int(expected_app):
            errors.append(f"target PR required check {context} app id does not match protection")

    return errors


def print_red_main_repair_evidence(evidence: dict, errors: list[str]) -> None:
    active = evidence.get("active_ready") or {}
    pr = evidence.get("pr") or {}
    print(f"RED_MAIN_READY_COUNT={len(evidence.get('ready_items') or [])}")
    print(f"RED_MAIN_READY_ITEMS={','.join(evidence.get('ready_items') or [])}")
    print(f"RED_MAIN_ACTIVE_READY={active.get('id')}")
    print(f"RED_MAIN_SCOPE_PATH_COUNT={len(active.get('scope_paths') or [])}")
    for path in active.get("scope_paths") or []:
        print(f"RED_MAIN_SCOPE_PATH={path}")
    print(f"RED_MAIN_PR={pr.get('number')} base={pr.get('base')} head_sha={pr.get('head_sha')}")
    for path in sorted(pr.get("files") or []):
        print(f"RED_MAIN_PR_PATH={path}")
    for name, run in sorted((evidence.get("main_checks") or {}).items()):
        print(f"MAIN_CHECK {name}: status={run.get('status')} conclusion={run.get('conclusion')}")
    for name, run in sorted((pr.get("checks") or {}).items()):
        print(f"PR_CHECK {name}: status={run.get('status')} conclusion={run.get('conclusion')}")
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)


def required_pr_checks_evidence(
    repo: str,
    sha: str,
    required_contexts: list[str],
    *,
    interval_seconds: int,
    max_iterations: int,
) -> dict[str, dict]:
    for attempt in range(1, max_iterations + 1):
        check_runs = latest_run_map(commit_check_runs(repo, sha))
        pending = False
        failing = False
        for context in required_contexts:
            if context == "public-safety":
                continue
            run = check_runs.get(context)
            if run is None or run.get("status") != "completed":
                pending = True
                continue
            conclusion = run.get("conclusion")
            accepted = conclusion == "success" or (
                context in REQUIRED_CONTEXT_NEUTRAL_ALLOWED and conclusion == "neutral"
            )
            if not accepted:
                failing = True
        if not pending or failing or attempt == max_iterations:
            return {
                context: check_runs[context]
                for context in required_contexts
                if context != "public-safety" and context in check_runs
            }
        time.sleep(interval_seconds)
    return {}


def build_live_red_main_repair_evidence(
    repo: str,
    branch: str,
    pr_number: int,
    *,
    profile_name: str,
    pr_check_interval_seconds: int,
    pr_check_max_iterations: int,
) -> dict:
    profile = red_main_profile(profile_name)
    branch_sha = branch_head_sha(repo, branch)
    main_check_runs = latest_run_map(commit_check_runs(repo, branch_sha))
    required_contexts, required_app_ids = branch_required_checks(repo, branch)
    next_actions = repo_file_text(repo, branch, "NEXT_ACTIONS.md")
    active, ready_items = active_ready_entry(next_actions)
    active_data = None
    if active:
        active_data = {
            "id": active["id"],
            "title": active["title"],
            "scope_paths": active_scope_paths(active),
        }

    failure_check = str(profile["failure_check"])
    if failure_check in main_check_runs:
        main_check_runs[failure_check]["log"] = github_job_log_text(
            repo,
            int(main_check_runs[failure_check]["id"]),
        )

    pr = pull_request(repo, pr_number)
    pr_head_sha = pr["head"]["sha"]
    pr_files = sorted(file_info["filename"] for file_info in pull_request_files(repo, pr_number))
    active_scope = list((active_data or {}).get("scope_paths") or [])
    prelim_path_ok = bool(pr_files) and all(
        path_allowed_by_scope(path, active_scope) for path in pr_files
    )
    prelim_path_ok = prelim_path_ok and all(path in pr_files for path in profile["required_paths"])
    prelim_main_ok = (
        check_completed_failure(main_check_runs.get("public-safety"))
        and check_completed_non_failing(main_check_runs.get("advisories"))
        and check_completed_failure(main_check_runs.get(str(profile["failure_check"])))
    )
    prelim_pr_ok = pr.get("state") == "open" and pr["base"]["ref"] == "main"
    if prelim_path_ok and prelim_main_ok and prelim_pr_ok:
        pr_checks = required_pr_checks_evidence(
            repo,
            pr_head_sha,
            required_contexts,
            interval_seconds=pr_check_interval_seconds,
            max_iterations=pr_check_max_iterations,
        )
    else:
        pr_checks = latest_run_map(commit_check_runs(repo, pr_head_sha))
    pr722 = pull_request(repo, 722)
    return {
        "branch": branch,
        "branch_sha": branch_sha,
        "required_contexts": required_contexts,
        "required_app_ids": required_app_ids,
        "ready_items": ready_items,
        "active_ready": active_data,
        "main_checks": main_check_runs,
        "pr": {
            "number": pr_number,
            "state": pr.get("state"),
            "merged": bool(pr.get("merged_at")),
            "base": pr["base"]["ref"],
            "head_sha": pr_head_sha,
            "files": pr_files,
            "checks": pr_checks,
        },
        "pr722": {
            "number": 722,
            "state": pr722.get("state"),
            "merged": bool(pr722.get("merged_at")),
        },
    }


def validate_red_main_repair_pr(
    repo: str,
    pr_number: int,
    expected_sha: str | None,
    *,
    branch: str,
    profile_name: str,
    expected_markers: list[str],
    expected_active_ready_na: str | None,
    pr_check_interval_seconds: int,
    pr_check_max_iterations: int,
) -> int:
    evidence = build_live_red_main_repair_evidence(
        repo,
        branch,
        pr_number,
        profile_name=profile_name,
        pr_check_interval_seconds=pr_check_interval_seconds,
        pr_check_max_iterations=pr_check_max_iterations,
    )
    errors = validate_red_main_repair_evidence(
        evidence,
        profile_name=profile_name,
        pr_number=pr_number,
        expected_sha=expected_sha,
        expected_markers=expected_markers,
        expected_active_ready_na=expected_active_ready_na,
    )
    print_red_main_repair_evidence(evidence, errors)
    if errors:
        return 1
    print(
        f"ALLOW: PR {pr_number} is a bounded active-NA red-main repair for profile "
        f"{profile_name}"
    )
    return 0


def self_repair_bootstrap_paths(
    files: list[dict],
) -> tuple[list[str], list[str], list[str]]:
    allowed: list[str] = []
    disallowed: list[str] = []
    testplans: list[str] = []
    for file_info in files:
        filename = file_info["filename"]
        status = file_info.get("status")
        if status != "modified":
            disallowed.append(filename)
            continue
        if filename in SELF_REPAIR_BOOTSTRAP_ROOT_PATHS:
            allowed.append(filename)
            continue
        if SELF_REPAIR_BOOTSTRAP_TESTPLAN_RE.fullmatch(filename):
            allowed.append(filename)
            testplans.append(filename)
            continue
        disallowed.append(filename)
    return sorted(allowed), sorted(disallowed), sorted(testplans)


def validate_self_repair_bootstrap_pr(
    repo: str,
    pr_number: int,
    expected_sha: str | None,
    *,
    branch: str,
    check_name: str,
    main_advisories_check: str,
    allow_missing_main_check: bool = False,
) -> int:
    sha = branch_head_sha(repo, branch)
    main_check_runs = commit_check_runs(repo, sha)
    run = latest_run_for_name(main_check_runs, check_name)
    if run is None:
        if not allow_missing_main_check:
            print(
                f"ERROR: latest {branch} commit {sha} is missing check '{check_name}'",
                file=sys.stderr,
            )
            return 1
        print(
            f"WARN: latest {branch} commit {sha} is missing check '{check_name}'; "
            "continuing because advisories-side self-repair eligibility is still "
            "bounded by failing latest-main advisories plus exact PR scope"
        )
    else:
        status = run.get("status")
        conclusion = run.get("conclusion")
        print(
            f"{branch} sha={sha} check={check_name} status={status} "
            f"conclusion={conclusion}"
        )
        if status == "completed" and conclusion == "success":
            print(
                f"ERROR: latest {branch} {check_name} is already green; self-repair bootstrap "
                f"is not needed",
                file=sys.stderr,
            )
            return 1

    main_advisories = latest_run_for_name(main_check_runs, main_advisories_check)
    if main_advisories is None:
        print(
            f"ERROR: latest {branch} commit {sha} is missing check "
            f"'{main_advisories_check}'",
            file=sys.stderr,
        )
        return 1
    main_advisories_status = main_advisories.get("status")
    main_advisories_conclusion = main_advisories.get("conclusion")
    print(
        f"{branch} sha={sha} check={main_advisories_check} "
        f"status={main_advisories_status} conclusion={main_advisories_conclusion}"
    )
    if (
        main_advisories_status != "completed"
        or main_advisories_conclusion != "failure"
    ):
        print(
            f"ERROR: latest {branch} is not red because {main_advisories_check} is failing; "
            f"self-repair bootstrap is not allowed",
            file=sys.stderr,
        )
        return 1

    pr = pull_request(repo, pr_number)
    pr_head_sha = pr["head"]["sha"]
    print(f"PR {pr_number} head_sha={pr_head_sha}")
    if expected_sha and pr_head_sha != expected_sha:
        print(
            f"ERROR: PR {pr_number} head {pr_head_sha} does not match expected "
            f"{expected_sha}",
            file=sys.stderr,
        )
        return 1

    files = pull_request_files(repo, pr_number)
    print(f"PR {pr_number} changed_file_count={len(files)}")
    for file_info in files:
        print(file_info["filename"])

    allowed_paths, disallowed_paths, testplans = self_repair_bootstrap_paths(files)
    print(f"SELF_REPAIR_ALLOWED_COUNT={len(allowed_paths)}")
    print(f"SELF_REPAIR_DISALLOWED_COUNT={len(disallowed_paths)}")
    print(f"SELF_REPAIR_TESTPLAN_COUNT={len(testplans)}")
    if disallowed_paths:
        print(
            f"ERROR: PR {pr_number} changes paths outside the sanctioned "
            f"self-repair bootstrap scope",
            file=sys.stderr,
        )
        for path in disallowed_paths:
            print(path, file=sys.stderr)
        return 1

    missing_required = sorted(
        path
        for path in (
            ".github/workflows/public-ci.yml",
            "scripts/ci/public_safety_gate.py",
        )
        if path not in allowed_paths
    )
    if missing_required:
        print(
            f"ERROR: PR {pr_number} is missing required self-repair paths",
            file=sys.stderr,
        )
        for path in missing_required:
            print(path, file=sys.stderr)
        return 1
    if len(testplans) != 1:
        print(
            f"ERROR: PR {pr_number} must modify exactly one tests/NA-*public_safety*.md "
            f"testplan stub for self-repair bootstrap",
            file=sys.stderr,
        )
        return 1

    print(
        f"ALLOW: latest {branch} remains red via {main_advisories_check}, and PR "
        f"{pr_number} is a sanctioned workflow-only public-safety self-repair"
    )
    return 0


def check_main_public_safety(args: argparse.Namespace) -> int:
    sha = branch_head_sha(args.repo, args.branch)
    main_check_runs = commit_check_runs(args.repo, sha)
    run = latest_run_for_name(main_check_runs, args.check_name)
    if run is None:
        print(
            f"ERROR: latest {args.branch} commit {sha} is missing check '{args.check_name}'",
            file=sys.stderr,
        )
        return 1
    status = run.get("status")
    conclusion = run.get("conclusion")
    print(
        f"{args.branch} sha={sha} check={args.check_name} status={status} conclusion={conclusion}"
    )
    if status == "completed" and conclusion == "success":
        return 0
    if args.allow_self_repair_bootstrap_pr is not None:
        return validate_self_repair_bootstrap_pr(
            args.repo,
            args.allow_self_repair_bootstrap_pr,
            args.expected_pr_sha,
            branch=args.branch,
            check_name=args.check_name,
            main_advisories_check=args.main_advisories_check,
        )
    def red_main_attempt() -> int:
        return validate_red_main_repair_pr(
            args.repo,
            args.allow_red_main_repair_pr,
            args.expected_red_main_repair_sha or args.expected_pr_sha,
            branch=args.branch,
            profile_name=args.red_main_repair_profile,
            expected_markers=args.expected_main_failure_marker or [],
            expected_active_ready_na=args.expected_active_ready_na,
            pr_check_interval_seconds=args.pr_check_interval_seconds,
            pr_check_max_iterations=args.pr_check_max_iterations,
        )

    def advisory_attempt() -> int:
        return validate_advisory_remediation_pr(
            args,
            sha=sha,
            main_check_runs=main_check_runs,
        )

    main_advisories = latest_run_for_name(main_check_runs, args.main_advisories_check)
    advisory_is_blocker = check_completed_failure(main_advisories)
    attempts = []
    if advisory_is_blocker and args.allow_advisory_remediation_pr is not None:
        attempts.append(("advisory-remediation", advisory_attempt))
    if args.allow_red_main_repair_pr is not None:
        attempts.append(("red-main-repair", red_main_attempt))
    if not advisory_is_blocker and args.allow_advisory_remediation_pr is not None:
        attempts.append(("advisory-remediation", advisory_attempt))

    for name, attempt in attempts:
        rc = attempt()
        print(f"PUBLIC_SAFETY_ATTEMPT {name} rc={rc}")
        if rc == 0:
            return 0
    if attempts:
        print(
            f"ERROR: latest {args.branch} public safety is not green and no bounded "
            "admission path accepted this PR",
            file=sys.stderr,
        )
        return 1

    if args.allow_advisory_remediation_pr is None:
        print(
            f"ERROR: latest {args.branch} public safety is not green; relevant PRs stay blocked",
            file=sys.stderr,
        )
        return 1

    return 1


def validate_advisory_remediation_pr(
    args: argparse.Namespace,
    *,
    sha: str,
    main_check_runs: list[dict],
) -> int:
    pr = pull_request(args.repo, args.allow_advisory_remediation_pr)
    pr_head_sha = pr["head"]["sha"]
    print(f"PR {args.allow_advisory_remediation_pr} head_sha={pr_head_sha}")
    if args.expected_pr_sha and pr_head_sha != args.expected_pr_sha:
        print(
            f"ERROR: PR {args.allow_advisory_remediation_pr} head {pr_head_sha} does not match "
            f"expected {args.expected_pr_sha}",
            file=sys.stderr,
        )
        return 1

    main_advisories = latest_run_for_name(main_check_runs, args.main_advisories_check)
    if main_advisories is None:
        print(
            f"ERROR: latest {args.branch} commit {sha} is missing check "
            f"'{args.main_advisories_check}'",
            file=sys.stderr,
        )
        return 1
    main_advisories_status = main_advisories.get("status")
    main_advisories_conclusion = main_advisories.get("conclusion")
    print(
        f"{args.branch} sha={sha} check={args.main_advisories_check} "
        f"status={main_advisories_status} conclusion={main_advisories_conclusion}"
    )
    if main_advisories_status != "completed" or main_advisories_conclusion != "failure":
        print(
            f"ERROR: latest {args.branch} is red for a reason other than "
            f"{args.main_advisories_check}; advisory-remediation PRs stay blocked",
            file=sys.stderr,
        )
        return 1

    remediation_paths = dependency_remediation_paths(
        pull_request_files(args.repo, args.allow_advisory_remediation_pr)
    )
    print(
        f"PR {args.allow_advisory_remediation_pr} dependency_remediation_path_count="
        f"{len(remediation_paths)}"
    )
    for path in remediation_paths:
        print(path)
    if not remediation_paths:
        print(
            f"ERROR: PR {args.allow_advisory_remediation_pr} does not change Cargo.lock or any "
            f"Cargo.toml path; advisory-remediation bypass is not allowed",
            file=sys.stderr,
        )
        return 1

    pr_advisories = latest_run_for_name(
        commit_check_runs(args.repo, pr_head_sha), args.pr_advisories_check
    )
    if pr_advisories is None:
        print(
            f"ERROR: PR {args.allow_advisory_remediation_pr} head {pr_head_sha} is missing check "
            f"'{args.pr_advisories_check}'",
            file=sys.stderr,
        )
        return 1
    pr_advisories_status = pr_advisories.get("status")
    pr_advisories_conclusion = pr_advisories.get("conclusion")
    print(
        f"PR {args.allow_advisory_remediation_pr} head_sha={pr_head_sha} "
        f"check={args.pr_advisories_check} status={pr_advisories_status} "
        f"conclusion={pr_advisories_conclusion}"
    )
    if pr_advisories_status != "completed" or pr_advisories_conclusion != "success":
        print(
            f"ERROR: PR {args.allow_advisory_remediation_pr} does not clear "
            f"{args.pr_advisories_check} on its own head; relevant PRs stay blocked",
            file=sys.stderr,
        )
        return 1

    print(
        f"ALLOW: latest {args.branch} public safety is red via {args.main_advisories_check}, "
        f"but PR {args.allow_advisory_remediation_pr} clears {args.pr_advisories_check} on its "
        f"own head and changes dependency-remediation paths"
    )
    return 0


def wait_for_required_checks(
    *,
    repo: str,
    sha: str,
    required: list[str],
    interval_seconds: int,
    max_iterations: int,
    fetch_check_runs=commit_check_runs_for_wait,
    sleeper=time.sleep,
) -> int:
    transient_count = 0
    for attempt in range(1, max_iterations + 1):
        try:
            check_runs = fetch_check_runs(repo, sha)
        except TransientGitHubPollingError as exc:
            transient_count += 1
            print(f"ITER={attempt}/{max_iterations} sha={sha}")
            print(f"TRANSIENT_CHECK_POLL {exc}")
            if attempt != max_iterations:
                sleeper(interval_seconds)
            continue
        pending = False
        print(f"ITER={attempt}/{max_iterations} sha={sha}")
        for check_name in required:
            run = latest_run_for_name(check_runs, check_name)
            if run is None:
                pending = True
                print(f"CHECK {check_name}: missing")
                continue
            status = run.get("status")
            conclusion = run.get("conclusion")
            print(f"CHECK {check_name}: status={status} conclusion={conclusion}")
            if status != "completed":
                pending = True
                continue
            if conclusion != "success":
                print(
                    f"ERROR: {check_name} is not green on {sha} "
                    f"(conclusion={conclusion})",
                    file=sys.stderr,
                )
                return 1
        if not pending:
            print(f"OK: required checks green on {sha}")
            return 0
        if attempt != max_iterations:
            sleeper(interval_seconds)
    print(
        f"ERROR: required checks did not settle green on {sha} after bounded wait "
        f"(transient_poll_errors={transient_count})",
        file=sys.stderr,
    )
    return 2


def wait_for_commit_checks(args: argparse.Namespace) -> int:
    return wait_for_required_checks(
        repo=args.repo,
        sha=args.sha,
        required=args.required,
        interval_seconds=args.interval_seconds,
        max_iterations=args.max_iterations,
    )


def list_pr_files(args: argparse.Namespace) -> int:
    for filename in sorted(file_info["filename"] for file_info in pull_request_files(args.repo, args.pr)):
        print(filename)
    return 0


def verify_pr_head(args: argparse.Namespace) -> int:
    pr = pull_request(args.repo, args.pr)
    head_sha = pr["head"]["sha"]
    print(f"PR {args.pr} head_sha={head_sha}")
    if args.sha and head_sha != args.sha:
        print(
            f"ERROR: PR {args.pr} head {head_sha} does not match expected {args.sha}",
            file=sys.stderr,
        )
        return 1
    return 0


def scan_pr_changes(args: argparse.Namespace) -> int:
    pr = pull_request(args.repo, args.pr)
    head_sha = pr["head"]["sha"]
    base_sha = pr["base"]["sha"]
    if args.verify_sha and head_sha != args.verify_sha:
        print(
            f"ERROR: PR {args.pr} head {head_sha} does not match expected {args.verify_sha}",
            file=sys.stderr,
        )
        return 1
    ref = args.ref or head_sha
    files = pull_request_files(args.repo, args.pr)
    print(f"PR {args.pr} base={base_sha} head={head_sha} ref={ref}")
    print(f"CHANGED_FILE_COUNT={len(files)}")
    for file_info in files:
        print(file_info["filename"])

    hits = denylist_hits(files)
    print(f"DENY_HITS_FILES={len(hits)}")
    if hits:
        print("ERROR: denylist filenames detected:", file=sys.stderr)
        for path in hits:
            print(path, file=sys.stderr)
        return 1

    content_hits: list[str] = []
    for path in content_scan_paths(files):
        text = repo_file_text(args.repo, ref, path)
        if HIGH_CONF_PATTERN.search(text):
            content_hits.append(path)
    print(f"HC_COUNT={len(content_hits)}")
    if content_hits:
        print("ERROR: high-confidence credential hits detected:", file=sys.stderr)
        for path in content_hits:
            print(path, file=sys.stderr)
        return 1

    objects = 0
    arrays = 0
    vector_files = vector_paths(files)
    for path in vector_files:
        data = json.loads(repo_file_text(args.repo, ref, path))
        if data is None:
            print(f"ERROR: {path} is empty/null", file=sys.stderr)
            return 1
        if isinstance(data, list):
            if not data:
                print(f"ERROR: {path} is an empty array", file=sys.stderr)
                return 1
            arrays += 1
            continue
        if isinstance(data, dict):
            objects += 1
            if not any(key in data for key in ("cases", "vectors", "tests")):
                print(f"ERROR: {path} missing top-level cases/vectors/tests", file=sys.stderr)
                return 1
            if all(not data.get(key) for key in ("cases", "vectors", "tests")):
                print(f"ERROR: {path} has empty cases/vectors/tests", file=sys.stderr)
                return 1
            continue
        print(f"ERROR: {path} is not an object or array", file=sys.stderr)
        return 1
    print(f"VECTOR_JSON_CHANGED={len(vector_files)}")
    print(f"VECTOR_STRUCTURE_OK objects={objects} arrays={arrays}")

    missing_links: list[str] = []
    for path in markdown_paths_for_scan(files):
        text = strip_code_blocks(repo_file_text(args.repo, ref, path))
        for _, raw_target in MARKDOWN_LINK_RE.findall(text):
            target = raw_target.strip()
            if not target or "://" in target or target.startswith("mailto:"):
                continue
            if target.startswith("<") and target.endswith(">"):
                target = target[1:-1]
            target = target.split("#", 1)[0].strip()
            if not target:
                continue
            resolved = resolve_repo_target(path, target)
            if resolved is None or not repo_path_exists(args.repo, ref, resolved):
                missing_links.append(f"{path}:{target}")
    print(f"MARKDOWN_FILES_CHANGED={len(markdown_paths_for_scan(files))}")
    print(f"TOTAL_MISSING={len(missing_links)}")
    if missing_links:
        print("ERROR: broken relative markdown links detected:", file=sys.stderr)
        for item in missing_links:
            print(item, file=sys.stderr)
        return 1
    return 0


def write_repo_file(args: argparse.Namespace) -> int:
    body = repo_file_bytes(args.repo, args.ref, args.path)
    with open(args.output, "wb") as handle:
        handle.write(body)
    print(f"WROTE {args.path} at {args.ref} to {args.output}")
    return 0


def fixture_required_contexts() -> list[str]:
    return [
        "ci-4a",
        "ci-4b",
        "ci-4c",
        "ci-4d",
        "ci-4d-dur",
        "demo-cli-build",
        "demo-cli-smoke",
        "formal-scka-model",
        "goal-lint",
        "metadata-conformance-smoke",
        "suite2-vectors",
        "CodeQL",
        "macos-qsc-qshield-build",
        "public-safety",
    ]


def fixture_checks(
    contexts: list[str],
    *,
    missing: set[str] | None = None,
    failing: set[str] | None = None,
) -> dict[str, dict]:
    result: dict[str, dict] = {}
    missing = missing or set()
    failing = failing or set()
    for context in contexts:
        if context in missing:
            continue
        conclusion = "failure" if context in failing else "success"
        if context == "CodeQL" and context not in failing:
            conclusion = "neutral"
        result[context] = {
            "status": "completed",
            "conclusion": conclusion,
            "app": {"id": 15368},
        }
    result.get("CodeQL", {}).setdefault("app", {"id": 57789})
    return result


def fixture_red_main_evidence(**overrides) -> dict:
    contexts = fixture_required_contexts()
    pr_files = [
        "DECISIONS.md",
        "TRACEABILITY.md",
        "qsl/qsl-client/qsc/tests/send_commit.rs",
        "tests/NA-0237A_send_commit_fallout_repair_testplan.md",
    ]
    evidence = {
        "required_contexts": contexts,
        "required_app_ids": {context: 15368 for context in contexts if context != "CodeQL"},
        "ready_items": ["NA-0239"],
        "active_ready": {
            "id": "NA-0239",
            "title": "Synthetic send_commit repair lane",
            "scope_paths": [
                "DECISIONS.md",
                "TRACEABILITY.md",
                "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
                "qsl/qsl-client/qsc/tests/send_commit.rs",
                "tests/NA-0237A_send_commit_fallout_repair_testplan.md",
            ],
        },
        "main_checks": {
            "public-safety": {"status": "completed", "conclusion": "failure"},
            "advisories": {"status": "completed", "conclusion": "success"},
            "macos-qsc-full-serial": {
                "status": "completed",
                "conclusion": "failure",
                "log": "tests/send_commit.rs send_commit vault_mock_provider_retired",
            },
        },
        "pr": {
            "number": 721,
            "state": "open",
            "merged": False,
            "base": "main",
            "head_sha": "abc123",
            "files": pr_files,
            "checks": fixture_checks(contexts),
        },
        "pr722": {"number": 722, "state": "closed", "merged": False},
    }
    for key, value in overrides.items():
        if key == "pr":
            evidence["pr"].update(value)
        elif key == "main_checks":
            evidence["main_checks"].update(value)
        elif key == "active_ready":
            evidence["active_ready"].update(value)
        else:
            evidence[key] = value
    return evidence


def expect_fixture_result(name: str, expected_ok: bool, errors: list[str]) -> bool:
    observed_ok = not errors
    if observed_ok == expected_ok:
        verdict = "ADMITTED" if observed_ok else "REJECTED"
        print(f"FIXTURE {name}: PASS ({verdict})")
        if errors:
            print(f"FIXTURE {name}: reason={errors[0]}")
        return True
    print(f"FIXTURE {name}: FAIL expected_ok={expected_ok} observed_ok={observed_ok}")
    for error in errors:
        print(f"FIXTURE {name}: error={error}")
    return False


def validate_advisory_remediation_fixture(evidence: dict) -> list[str]:
    errors: list[str] = []
    main = evidence.get("main_checks") or {}
    if not check_completed_failure(main.get("public-safety")):
        errors.append("latest main public-safety is not failed")
    if not check_completed_failure(main.get("advisories")):
        errors.append("latest main advisories is not failed")
    if not dependency_remediation_paths(
        [{"filename": path, "status": "modified"} for path in evidence.get("pr", {}).get("files", [])]
    ):
        errors.append("advisory PR does not change dependency remediation paths")
    pr_advisories = (evidence.get("pr", {}).get("checks") or {}).get("advisories")
    if not check_completed_success(pr_advisories):
        errors.append("PR advisories check is not success")
    return errors


def validate_self_repair_fixture(evidence: dict) -> list[str]:
    errors: list[str] = []
    main = evidence.get("main_checks") or {}
    public_safety = main.get("public-safety")
    if public_safety and check_completed_success(public_safety):
        errors.append("latest main public-safety is already green")
    if not check_completed_failure(main.get("advisories")):
        errors.append("latest main advisories is not failed")
    files = [
        {"filename": path, "status": "modified"}
        for path in evidence.get("pr", {}).get("files", [])
    ]
    allowed, disallowed, testplans = self_repair_bootstrap_paths(files)
    if disallowed:
        errors.append("self-repair fixture has disallowed paths: " + ",".join(disallowed))
    for required in (".github/workflows/public-ci.yml", "scripts/ci/public_safety_gate.py"):
        if required not in allowed:
            errors.append("self-repair fixture is missing required path " + required)
    if len(testplans) != 1:
        errors.append("self-repair fixture must have exactly one testplan")
    return errors


def run_fixture_proofs(args: argparse.Namespace) -> int:
    profile = "send_commit_vault_mock_provider_retired"
    markers = ["send_commit", "vault_mock_provider_retired"]
    cases = [
        (
            "positive_721_equivalent",
            True,
            fixture_red_main_evidence(),
            721,
            "abc123",
            markers,
        ),
        (
            "wrong_pr",
            False,
            fixture_red_main_evidence(pr={"number": 720}),
            721,
            "abc123",
            markers,
        ),
        (
            "wrong_head",
            False,
            fixture_red_main_evidence(),
            721,
            "wrong",
            markers,
        ),
        (
            "unrelated_path",
            False,
            fixture_red_main_evidence(pr={"files": ["README.md"]}),
            721,
            "abc123",
            markers,
        ),
        (
            "missing_marker",
            False,
            fixture_red_main_evidence(
                main_checks={
                    "macos-qsc-full-serial": {
                        "status": "completed",
                        "conclusion": "failure",
                        "log": "tests/send_commit.rs send_commit",
                    }
                }
            ),
            721,
            "abc123",
            markers,
        ),
        (
            "advisories_red",
            False,
            fixture_red_main_evidence(
                main_checks={"advisories": {"status": "completed", "conclusion": "failure"}}
            ),
            721,
            "abc123",
            markers,
        ),
        (
            "kt_708_path_mismatch",
            False,
            fixture_red_main_evidence(
                pr={"files": ["tools/refimpl/quantumshield_refimpl/src/kt/mod.rs"]}
            ),
            721,
            "abc123",
            markers,
        ),
        (
            "multiple_ready",
            False,
            fixture_red_main_evidence(ready_items=["NA-0239", "NA-0240"]),
            721,
            "abc123",
            markers,
        ),
        (
            "missing_required_check",
            False,
            fixture_red_main_evidence(
                pr={"checks": fixture_checks(fixture_required_contexts(), missing={"ci-4a"})}
            ),
            721,
            "abc123",
            markers,
        ),
        (
            "unrelated_main_failure",
            False,
            fixture_red_main_evidence(
                main_checks={
                    "macos-qsc-full-serial": {
                        "status": "completed",
                        "conclusion": "failure",
                        "log": "unrelated panic",
                    }
                }
            ),
            721,
            "abc123",
            markers,
        ),
        (
            "ordinary_pr_main_red_blocked",
            False,
            fixture_red_main_evidence(pr={"files": ["README.md"]}),
            721,
            "abc123",
            markers,
        ),
    ]
    ok = True
    for name, expected_ok, evidence, pr_number, expected_sha, expected in cases:
        errors = validate_red_main_repair_evidence(
            evidence,
            profile_name=profile,
            pr_number=pr_number,
            expected_sha=expected_sha,
            expected_markers=expected,
            expected_active_ready_na="NA-0239",
        )
        ok = expect_fixture_result(name, expected_ok, errors) and ok

    advisory_evidence = {
        "main_checks": {
            "public-safety": {"status": "completed", "conclusion": "failure"},
            "advisories": {"status": "completed", "conclusion": "failure"},
        },
        "pr": {
            "files": ["Cargo.lock"],
            "checks": {"advisories": {"status": "completed", "conclusion": "success"}},
        },
    }
    ok = expect_fixture_result(
        "advisory_remediation_no_regression",
        True,
        validate_advisory_remediation_fixture(advisory_evidence),
    ) and ok
    advisory_not_blocker = {
        "main_checks": {
            "public-safety": {"status": "completed", "conclusion": "failure"},
            "advisories": {"status": "completed", "conclusion": "success"},
        },
        "pr": {
            "files": ["Cargo.lock"],
            "checks": {"advisories": {"status": "completed", "conclusion": "success"}},
        },
    }
    ok = expect_fixture_result(
        "advisory_remediation_not_blocker_rejected",
        False,
        validate_advisory_remediation_fixture(advisory_not_blocker),
    ) and ok

    self_repair = {
        "main_checks": {
            "public-safety": {"status": "completed", "conclusion": "failure"},
            "advisories": {"status": "completed", "conclusion": "failure"},
        },
        "pr": {
            "files": [
                ".github/workflows/public-ci.yml",
                "scripts/ci/public_safety_gate.py",
                "DECISIONS.md",
                "TRACEABILITY.md",
                "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
                "tests/NA-0239_public_safety_red_main_deadlock_prevention_testplan.md",
            ]
        },
    }
    ok = expect_fixture_result(
        "self_repair_no_regression",
        True,
        validate_self_repair_fixture(self_repair),
    ) and ok
    self_repair_runtime = {
        "main_checks": self_repair["main_checks"],
        "pr": {"files": self_repair["pr"]["files"] + ["Cargo.lock"]},
    }
    ok = expect_fixture_result(
        "self_repair_runtime_or_cargo_rejected",
        False,
        validate_self_repair_fixture(self_repair_runtime),
    ) and ok
    if ok:
        print("OK: NA-0239 public-safety fixture proofs passed")
        return 0
    return 1


PUSH_SUITE_CHECKS = [
    "qsc-linux-full-suite",
    "macos-qsc-full-serial",
    "qsc-adversarial-smoke",
]

FULL_SUITE_COST_CONTROL_CHECKS = [
    "qsc-linux-full-suite",
    "macos-qsc-full-serial",
]


def fixture_check_run(
    name: str,
    *,
    run_id: int,
    status: str = "completed",
    conclusion: str | None = "success",
) -> dict:
    return {
        "id": run_id,
        "name": name,
        "status": status,
        "conclusion": conclusion,
    }


def push_suite_runs(
    *,
    status: str = "completed",
    conclusion: str | None = "success",
    failing: str | None = None,
    missing: set[str] | None = None,
    start_id: int = 100,
) -> list[dict]:
    missing = missing or set()
    runs: list[dict] = []
    for index, name in enumerate(PUSH_SUITE_CHECKS):
        if name in missing:
            continue
        check_conclusion = "failure" if name == failing else conclusion
        runs.append(
            fixture_check_run(
                name,
                run_id=start_id + index,
                status=status,
                conclusion=check_conclusion,
            )
        )
    return runs


def run_wait_fixture(name: str, sequence: list[object], expected_rc: int) -> bool:
    calls = {"count": 0}

    def fetcher(repo: str, sha: str) -> list[dict]:
        index = min(calls["count"], len(sequence) - 1)
        calls["count"] += 1
        item = sequence[index]
        if isinstance(item, Exception):
            raise item
        return item  # type: ignore[return-value]

    rc = wait_for_required_checks(
        repo="QuantumShieldLabs/qsl-protocol",
        sha="fixture-sha",
        required=PUSH_SUITE_CHECKS,
        interval_seconds=0,
        max_iterations=max(len(sequence), 1),
        fetch_check_runs=fetcher,
        sleeper=lambda _seconds: None,
    )
    if rc == expected_rc:
        print(f"TIMEOUT_RESILIENCE_FIXTURE {name}: PASS rc={rc}")
        return True
    print(f"TIMEOUT_RESILIENCE_FIXTURE {name}: FAIL expected_rc={expected_rc} rc={rc}")
    return False


def run_timeout_resilience_selftest(args: argparse.Namespace) -> int:
    ok = True
    pending = push_suite_runs(status="in_progress", conclusion=None, start_id=10)
    success = push_suite_runs(start_id=20)
    ok = run_wait_fixture(
        "html_timeout_then_success",
        [pending, TransientGitHubPollingError("HTML response from GitHub API"), success],
        0,
    ) and ok
    ok = run_wait_fixture(
        "non_json_then_success",
        [pending, TransientGitHubPollingError("non-JSON response from GitHub API"), success],
        0,
    ) and ok
    ok = run_wait_fixture(
        "http_502_503_504_then_success",
        [
            pending,
            TransientGitHubPollingError("HTTP 502"),
            TransientGitHubPollingError("HTTP 503"),
            TransientGitHubPollingError("HTTP 504"),
            success,
        ],
        0,
    ) and ok
    for failed_name in PUSH_SUITE_CHECKS:
        ok = run_wait_fixture(
            f"{failed_name}_failure_fails_closed",
            [push_suite_runs(failing=failed_name, start_id=30)],
            1,
        ) and ok
    ok = run_wait_fixture(
        "watched_suite_pending_budget_expires",
        [pending, pending],
        2,
    ) and ok
    ok = run_wait_fixture(
        "watched_suite_missing_budget_expires",
        [
            push_suite_runs(missing={"macos-qsc-full-serial"}, start_id=40),
            push_suite_runs(missing={"macos-qsc-full-serial"}, start_id=50),
        ],
        2,
    ) and ok
    ok = run_wait_fixture(
        "stale_failure_ignored_for_latest_success",
        [
            [
                fixture_check_run("qsc-linux-full-suite", run_id=1, conclusion="failure"),
                fixture_check_run("qsc-linux-full-suite", run_id=2, conclusion="success"),
                fixture_check_run("macos-qsc-full-serial", run_id=3, conclusion="success"),
                fixture_check_run("qsc-adversarial-smoke", run_id=4, conclusion="success"),
            ]
        ],
        0,
    ) and ok
    ok = run_wait_fixture(
        "stale_success_ignored_for_latest_failure",
        [
            [
                fixture_check_run("qsc-linux-full-suite", run_id=1, conclusion="success"),
                fixture_check_run("qsc-linux-full-suite", run_id=2, conclusion="failure"),
                fixture_check_run("macos-qsc-full-serial", run_id=3, conclusion="success"),
                fixture_check_run("qsc-adversarial-smoke", run_id=4, conclusion="success"),
            ]
        ],
        1,
    ) and ok

    generic_403 = transient_http_error_reason(
        403,
        {"X-RateLimit-Remaining": "60"},
        "Resource not accessible by integration",
    )
    if generic_403 is None:
        print("TIMEOUT_RESILIENCE_FIXTURE branch_protection_403_non_bypass: PASS")
    else:
        print(
            "TIMEOUT_RESILIENCE_FIXTURE branch_protection_403_non_bypass: "
            f"FAIL reason={generic_403}"
        )
        ok = False
    limited_429 = transient_http_error_reason(429, {"Retry-After": "1"}, "rate limit")
    if limited_429:
        print("TIMEOUT_RESILIENCE_FIXTURE rate_limit_429_transient: PASS")
    else:
        print("TIMEOUT_RESILIENCE_FIXTURE rate_limit_429_transient: FAIL")
        ok = False

    if ok:
        print("OK: NA-0254 timeout resilience self-test passed")
        return 0
    return 1


def parse_scope_classifier_output(text: str) -> dict[str, str]:
    parsed: dict[str, str] = {}
    for raw in text.splitlines():
        if "=" not in raw:
            continue
        key, value = raw.split("=", 1)
        parsed[key.strip()] = value.strip()
    return parsed


def classify_ci_scope_fixture(paths: list[str]) -> dict[str, str]:
    cmd = ["bash", "scripts/ci/classify_ci_scope.sh", *paths]
    proc = subprocess.run(cmd, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        raise RuntimeError(
            "classifier failed rc="
            f"{proc.returncode} stdout={proc.stdout!r} stderr={proc.stderr!r}"
        )
    parsed = parse_scope_classifier_output(proc.stdout)
    required = {"docs_only", "workflow_security", "runtime_critical", "scope_class"}
    missing = sorted(required - set(parsed))
    if missing:
        raise RuntimeError(f"classifier output missing keys: {','.join(missing)}")
    return parsed


def run_full_suite_cost_control_fixture(
    name: str,
    paths: list[str],
    *,
    expected_docs_only: str,
    expected_workflow_security: str,
    expected_runtime_critical: str,
    expected_scope_class: str,
) -> bool:
    try:
        parsed = classify_ci_scope_fixture(paths)
    except RuntimeError as exc:
        print(f"FULL_SUITE_COST_CONTROL_FIXTURE {name}: FAIL {exc}")
        return False

    expected = {
        "docs_only": expected_docs_only,
        "workflow_security": expected_workflow_security,
        "runtime_critical": expected_runtime_critical,
        "scope_class": expected_scope_class,
    }
    mismatches = [
        f"{key}: expected={expected[key]} actual={parsed.get(key)}"
        for key in sorted(expected)
        if parsed.get(key) != expected[key]
    ]
    wait_required = parsed.get("docs_only") != "true"
    expected_wait_required = expected_docs_only != "true"
    jobs_should_run = wait_required
    if wait_required != expected_wait_required:
        mismatches.append(
            "full_suite_wait_required: "
            f"expected={expected_wait_required} actual={wait_required}"
        )
    if jobs_should_run != expected_wait_required:
        mismatches.append(
            "full_suite_jobs_should_run: "
            f"expected={expected_wait_required} actual={jobs_should_run}"
        )
    if mismatches:
        print(f"FULL_SUITE_COST_CONTROL_FIXTURE {name}: FAIL")
        for mismatch in mismatches:
            print(f"  {mismatch}")
        return False
    print(
        f"FULL_SUITE_COST_CONTROL_FIXTURE {name}: PASS "
        f"scope_class={parsed['scope_class']} "
        f"full_suite_wait_required={str(wait_required).lower()} "
        f"full_suite_jobs_should_run={str(jobs_should_run).lower()}"
    )
    return True


def run_full_suite_cost_control_selftest(args: argparse.Namespace) -> int:
    fixtures = [
        (
            "docs_governance_closeout",
            [
                "NEXT_ACTIONS.md",
                "DECISIONS.md",
                "TRACEABILITY.md",
                "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
                "tests/NA-0262A_closeout_restore_na0262_testplan.md",
            ],
            "true",
            "false",
            "false",
            "docs_only",
        ),
        (
            "start_here_root_doc",
            ["START_HERE.md"],
            "true",
            "false",
            "false",
            "docs_only",
        ),
        (
            "public_front_door_bundle",
            [
                "README.md",
                "START_HERE.md",
                "docs/public/INDEX.md",
                "docs/public/RELEASE_READINESS_EVIDENCE_MAP.md",
                "docs/public/EXTERNAL_REVIEW_PACKAGE.md",
                "docs/governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md",
                "tests/NA-0294_public_evidence_navigation_refresh_testplan.md",
                "DECISIONS.md",
                "TRACEABILITY.md",
                "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
            ],
            "true",
            "false",
            "false",
            "docs_only",
        ),
        (
            "runtime_qsc_path",
            ["qsl/qsl-client/qsc/src/main.rs"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "apps_qshield_cli_path",
            ["apps/qshield-cli/src/main.rs"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "scripts_ci_path",
            ["scripts/ci/public_safety_gate.py"],
            "false",
            "true",
            "false",
            "workflow_security",
        ),
        (
            "github_workflow_path",
            [".github/workflows/public-ci.yml"],
            "false",
            "true",
            "false",
            "workflow_security",
        ),
        (
            "cargo_toml_path",
            ["Cargo.toml"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "cargo_lock_path",
            ["Cargo.lock"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "qsl_server_path",
            ["qsl-server/src/main.rs"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "qsl_attachments_path",
            ["qsl-attachments/src/lib.rs"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "qsc_desktop_path",
            ["qsc-desktop/src/main.ts"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "mixed_docs_runtime_path",
            ["docs/INDEX.md", "qsl/qsl-client/qsc/src/main.rs"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "unknown_path",
            ["unknown/new.bin"],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
        (
            "ambiguous_empty_push_scope",
            [],
            "false",
            "false",
            "true",
            "runtime_critical",
        ),
    ]

    ok = True
    for name, paths, docs_only, workflow_security, runtime_critical, scope_class in fixtures:
        ok = run_full_suite_cost_control_fixture(
            name,
            paths,
            expected_docs_only=docs_only,
            expected_workflow_security=workflow_security,
            expected_runtime_critical=runtime_critical,
            expected_scope_class=scope_class,
        ) and ok

    if ok:
        print("OK: NA-0262A full-suite cost-control self-test passed")
        return 0
    return 1


def run_advisories_resilience_fixture(
    name: str,
    *,
    exit_code: int,
    output: str,
    expected: str,
) -> bool:
    observed, reason = classify_advisories_output(exit_code, output)
    if observed == expected:
        print(
            f"ADVISORIES_RESILIENCE_FIXTURE {name}: PASS "
            f"classification={observed} reason={reason}"
        )
        return True
    print(
        f"ADVISORIES_RESILIENCE_FIXTURE {name}: FAIL "
        f"expected={expected} observed={observed} reason={reason}"
    )
    return False


def run_advisories_retry_wrapper_fixture(name: str, command: list[str], expected_rc: int) -> bool:
    with tempfile.TemporaryDirectory(prefix="qsl-advisories-resilience-") as temp_dir:
        output = os.path.join(temp_dir, "audit_output.txt")
        old_flag = os.environ.get("QSL_ADVISORIES_RETRY_FLAG")
        os.environ["QSL_ADVISORIES_RETRY_FLAG"] = os.path.join(temp_dir, "seen")
        try:
            rc = run_cargo_audit_with_resilience(
                argparse.Namespace(
                    output=output,
                    max_retries=1,
                    retry_delay_seconds=0,
                    command=command,
                )
            )
        finally:
            if old_flag is None:
                os.environ.pop("QSL_ADVISORIES_RETRY_FLAG", None)
            else:
                os.environ["QSL_ADVISORIES_RETRY_FLAG"] = old_flag
        text = open(output, "r", encoding="utf-8", errors="replace").read()
    if rc != expected_rc:
        print(f"ADVISORIES_RETRY_FIXTURE {name}: FAIL expected_rc={expected_rc} rc={rc}")
        return False
    if expected_rc == 0:
        required = [
            "ADVISORIES_RETRY_TRANSIENT_FETCH",
            "ADVISORIES_TRANSIENT_FETCH_RETRY_OK",
        ]
    else:
        required = ["ADVISORIES_REAL_FINDING_FAIL_CLOSED"]
    missing = [marker for marker in required if marker not in text]
    if missing:
        print(
            f"ADVISORIES_RETRY_FIXTURE {name}: FAIL missing_markers={','.join(missing)}"
        )
        return False
    print(f"ADVISORIES_RETRY_FIXTURE {name}: PASS rc={rc}")
    return True


def run_advisories_resilience_selftest(args: argparse.Namespace) -> int:
    transient_fetch = """
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
    error: couldn't fetch advisory database: git operation failed: An IO error occurred when talking to the server
      -> error sending request for url (https://github.com/RustSec/advisory-db.git/info/refs?service=git-upload-pack)
    """
    real_advisory = """
    Crate:     rustls-webpki
    Version:   0.103.12
    Title:     verification issue
    ID:        RUSTSEC-2026-0104
    URL:       https://rustsec.org/advisories/RUSTSEC-2026-0104
    error: 1 vulnerability found!
    """
    warning_advisory = """
    Crate:     example
    Version:   1.2.3
    ID:        RUSTSEC-2026-9999
    warning: 1 warning found!
    """
    unknown_failure = "error: failed to parse lockfile at Cargo.lock"
    clean_success = """
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
    Loaded 1069 security advisories
    Scanning Cargo.lock for vulnerabilities
    """
    mixed_real_and_fetch = transient_fetch + "\n" + real_advisory

    ok = True
    ok = run_advisories_resilience_fixture(
        "clean_success",
        exit_code=0,
        output=clean_success,
        expected="clean_success",
    ) and ok
    ok = run_advisories_resilience_fixture(
        "transient_fetch_failure",
        exit_code=1,
        output=transient_fetch,
        expected="transient_fetch",
    ) and ok
    ok = run_advisories_resilience_fixture(
        "real_advisory_fails_closed",
        exit_code=1,
        output=real_advisory,
        expected="real_finding",
    ) and ok
    ok = run_advisories_resilience_fixture(
        "warning_advisory_fails_closed",
        exit_code=1,
        output=warning_advisory,
        expected="real_finding",
    ) and ok
    ok = run_advisories_resilience_fixture(
        "unknown_failure_fails_closed",
        exit_code=1,
        output=unknown_failure,
        expected="unknown_failure",
    ) and ok
    ok = run_advisories_resilience_fixture(
        "real_finding_not_downgraded_by_fetch_text",
        exit_code=1,
        output=mixed_real_and_fetch,
        expected="real_finding",
    ) and ok
    ok = run_advisories_retry_wrapper_fixture(
        "transient_fetch_retried_then_success",
        [
            "bash",
            "-c",
            (
                'flag="${QSL_ADVISORIES_RETRY_FLAG:?}"; '
                'if [ ! -e "$flag" ]; then '
                'touch "$flag"; '
                'printf "%s\\n" "Fetching advisory database from '
                'https://github.com/RustSec/advisory-db.git" '
                '"error: could not fetch advisory database: git operation failed: '
                'An IO error occurred when talking to the server"; '
                'exit 1; '
                'fi; '
                'rm -f "$flag"; '
                'printf "%s\\n" "Loaded 1069 security advisories" '
                '"Scanning Cargo.lock for vulnerabilities"; '
                'exit 0'
            ),
        ],
        0,
    ) and ok
    ok = run_advisories_retry_wrapper_fixture(
        "real_finding_not_retried",
        [
            "bash",
            "-c",
            (
                'printf "%s\\n" "Crate: rustls-webpki" '
                '"ID: RUSTSEC-2026-0104" '
                '"error: 1 vulnerability found!"; exit 1'
            ),
        ],
        1,
    ) and ok

    if ok:
        print("OK: NA-0267 advisories resilience self-test passed")
        return 0
    return 1


def validate_self_repair_bootstrap_pr_cmd(args: argparse.Namespace) -> int:
    return validate_self_repair_bootstrap_pr(
        args.repo,
        args.pr,
        args.sha,
        branch=args.branch,
        check_name=args.check_name,
        main_advisories_check=args.main_advisories_check,
        allow_missing_main_check=args.allow_missing_main_public_safety,
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Fail-closed public-safety helpers for main-health gating."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    main_parser = subparsers.add_parser(
        "check-main-public-safety",
        help="Require the latest branch head to have a green public-safety check",
    )
    main_parser.add_argument("--repo", required=True)
    main_parser.add_argument("--branch", default="main")
    main_parser.add_argument("--check-name", default="public-safety")
    main_parser.add_argument("--allow-self-repair-bootstrap-pr", type=int)
    main_parser.add_argument("--allow-advisory-remediation-pr", type=int)
    main_parser.add_argument("--allow-red-main-repair-pr", type=int)
    main_parser.add_argument("--expected-pr-sha")
    main_parser.add_argument("--expected-red-main-repair-sha")
    main_parser.add_argument(
        "--red-main-repair-profile",
        default="send_commit_vault_mock_provider_retired",
    )
    main_parser.add_argument("--expected-main-failure-marker", action="append")
    main_parser.add_argument("--expected-active-ready-na")
    main_parser.add_argument("--main-advisories-check", default="advisories")
    main_parser.add_argument("--pr-advisories-check", default="advisories")
    main_parser.add_argument("--pr-check-interval-seconds", type=int, default=20)
    main_parser.add_argument("--pr-check-max-iterations", type=int, default=180)
    main_parser.set_defaults(func=check_main_public_safety)

    wait_parser = subparsers.add_parser(
        "wait-commit-checks",
        help="Wait for named checks on one commit and require success",
    )
    wait_parser.add_argument("--repo", required=True)
    wait_parser.add_argument("--sha", required=True)
    wait_parser.add_argument("--required", action="append", required=True)
    wait_parser.add_argument("--interval-seconds", type=int, default=20)
    wait_parser.add_argument("--max-iterations", type=int, default=390)
    wait_parser.set_defaults(func=wait_for_commit_checks)

    list_pr_parser = subparsers.add_parser(
        "list-pr-files",
        help="Print the changed file paths for one PR",
    )
    list_pr_parser.add_argument("--repo", required=True)
    list_pr_parser.add_argument("--pr", required=True, type=int)
    list_pr_parser.set_defaults(func=list_pr_files)

    verify_pr_parser = subparsers.add_parser(
        "verify-pr-head",
        help="Require one PR to still point at the expected head SHA",
    )
    verify_pr_parser.add_argument("--repo", required=True)
    verify_pr_parser.add_argument("--pr", required=True, type=int)
    verify_pr_parser.add_argument("--sha", required=False)
    verify_pr_parser.set_defaults(func=verify_pr_head)

    scan_pr_parser = subparsers.add_parser(
        "scan-pr-changes",
        help="Run denylist, content, vector, and markdown checks on changed PR files via the API",
    )
    scan_pr_parser.add_argument("--repo", required=True)
    scan_pr_parser.add_argument("--pr", required=True, type=int)
    scan_pr_parser.add_argument("--ref")
    scan_pr_parser.add_argument("--verify-sha")
    scan_pr_parser.set_defaults(func=scan_pr_changes)

    write_file_parser = subparsers.add_parser(
        "write-repo-file",
        help="Write one file from a repo ref to a local path",
    )
    write_file_parser.add_argument("--repo", required=True)
    write_file_parser.add_argument("--ref", required=True)
    write_file_parser.add_argument("--path", required=True)
    write_file_parser.add_argument("--output", required=True)
    write_file_parser.set_defaults(func=write_repo_file)

    classify_advisories_parser = subparsers.add_parser(
        "classify-advisories-output",
        help="Classify cargo-audit output as clean, transient fetch, real finding, or unknown",
    )
    classify_advisories_parser.add_argument("--input", required=True)
    classify_advisories_parser.add_argument("--exit-code", required=True, type=int)
    classify_advisories_parser.set_defaults(func=classify_advisories_output_cmd)

    audit_resilience_parser = subparsers.add_parser(
        "run-cargo-audit-with-resilience",
        help="Run cargo audit with bounded transient advisory database fetch retries",
    )
    audit_resilience_parser.add_argument("--output", required=True)
    audit_resilience_parser.add_argument("--max-retries", type=int, default=2)
    audit_resilience_parser.add_argument("--retry-delay-seconds", type=int, default=20)
    audit_resilience_parser.add_argument("command", nargs=argparse.REMAINDER)
    audit_resilience_parser.set_defaults(func=run_cargo_audit_with_resilience)

    self_repair_parser = subparsers.add_parser(
        "validate-self-repair-bootstrap-pr",
        help="Require one PR to fit the sanctioned workflow-only self-repair bootstrap scope",
    )
    self_repair_parser.add_argument("--repo", required=True)
    self_repair_parser.add_argument("--pr", required=True, type=int)
    self_repair_parser.add_argument("--sha")
    self_repair_parser.add_argument("--branch", default="main")
    self_repair_parser.add_argument("--check-name", default="public-safety")
    self_repair_parser.add_argument("--main-advisories-check", default="advisories")
    self_repair_parser.add_argument(
        "--allow-missing-main-public-safety",
        action="store_true",
        help=(
            "Permit the advisories job to classify a sanctioned self-repair PR when "
            "latest-main public-safety has not attached yet; final public-safety "
            "gating remains strict."
        ),
    )
    self_repair_parser.set_defaults(func=validate_self_repair_bootstrap_pr_cmd)

    fixture_parser = subparsers.add_parser(
        "run-na0239-fixture-proofs",
        help="Run local fixture proofs for NA-0239 public-safety red-main admission",
    )
    fixture_parser.set_defaults(func=run_fixture_proofs)

    timeout_resilience_parser = subparsers.add_parser(
        "selftest-timeout-resilience",
        help="Run deterministic NA-0254 wait-polling timeout resilience fixtures",
    )
    timeout_resilience_parser.set_defaults(func=run_timeout_resilience_selftest)

    full_suite_cost_parser = subparsers.add_parser(
        "selftest-full-suite-cost-control",
        help="Run deterministic NA-0262A full-suite cost-control classification fixtures",
    )
    full_suite_cost_parser.set_defaults(func=run_full_suite_cost_control_selftest)

    advisories_resilience_parser = subparsers.add_parser(
        "selftest-advisories-resilience",
        help="Run deterministic NA-0267 cargo-audit advisories resilience fixtures",
    )
    advisories_resilience_parser.set_defaults(func=run_advisories_resilience_selftest)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
