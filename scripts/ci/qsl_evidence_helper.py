#!/usr/bin/env python3
"""Read-only evidence helpers for QSL governance and CI diagnostics."""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
import re
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Sequence
from urllib.parse import unquote


REPO_DEFAULT = "QuantumShieldLabs/qsl-protocol"

REQUIRED_CONTEXTS = [
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

DEFAULT_NA_SELECTION = [
    "NA-0253",
    "NA-0252",
    "NA-0251",
    "NA-0250",
    "NA-0249",
    "NA-0248",
    "NA-0247",
    "NA-0246",
    "NA-0245",
    "NA-0244",
    "NA-0243",
    "NA-0242",
    "NA-0241",
    "NA-0240",
    "NA-0239",
    "NA-0238",
    "NA-0237",
]

DEFAULT_DECISION_SELECTION = [
    "D-0110",
    *[f"D-{number:04d}" for number in range(439, 473)],
]

IGNORED_LINK_PARTS = {
    ".git",
    "node_modules",
    "target",
    "dist",
}

SECRET_PATTERNS = [
    (
        "private_key",
        re.compile(r"-----BEGIN (?:RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----"),
    ),
    ("github_token", re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{30,}\b")),
    ("slack_token", re.compile(r"\bxox[baprs]-[A-Za-z0-9-]{20,}\b")),
    ("aws_access_key_id", re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b")),
    ("google_api_key", re.compile(r"\bAIza[0-9A-Za-z_-]{35}\b")),
    ("openai_key", re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b")),
    (
        "jwt",
        re.compile(r"\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}\b"),
    ),
]

PR_BODY_FIELDS = [
    "Goals:",
    "Impact:",
    "No-regression:",
    "Tests/Vectors:",
]

OVERCLAIM_PHRASES = [
    "production-ready",
    "proven true Triple Ratchet",
    "quantum-proof",
    "metadata-free",
    "anonymity",
]


class HelperError(RuntimeError):
    """Expected helper failure with a concise operator-facing message."""


@dataclass(frozen=True)
class QueueItem:
    na: str
    title: str
    status: str
    body: str


def repo_root() -> Path:
    try:
        out = run(["git", "rev-parse", "--show-toplevel"], check=True).stdout.strip()
        return Path(out)
    except HelperError:
        return Path.cwd()


def run(args: Sequence[str], *, check: bool = False, text: bool = True) -> subprocess.CompletedProcess[str]:
    try:
        proc = subprocess.run(
            list(args),
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=text,
        )
    except OSError as exc:
        raise HelperError(f"command failed to start: {' '.join(args)}: {exc}") from exc
    if check and proc.returncode != 0:
        stderr = proc.stderr.strip()
        raise HelperError(f"command failed ({proc.returncode}): {' '.join(args)}" + (f": {stderr}" if stderr else ""))
    return proc


def require_gh(report_only: bool) -> bool:
    if shutil.which("gh"):
        return True
    message = "ERROR: gh is not available; GitHub-backed helper command cannot run"
    if report_only:
        print(message)
        return False
    raise HelperError(message)


def gh_json(args: Sequence[str], *, report_only: bool = False) -> object | None:
    if not require_gh(report_only):
        return None
    proc = run(["gh", *args])
    if proc.returncode != 0:
        message = "ERROR: gh command failed: " + " ".join(["gh", *args])
        stderr = proc.stderr.strip()
        if stderr:
            message += f": {stderr}"
        if report_only:
            print(message)
            return None
        raise HelperError(message)
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        message = f"ERROR: gh returned non-JSON output for {' '.join(args)}"
        if report_only:
            print(message)
            return None
        raise HelperError(message) from exc


def gh_lines(args: Sequence[str], *, report_only: bool = False) -> list[str] | None:
    if not require_gh(report_only):
        return None
    proc = run(["gh", *args])
    if proc.returncode != 0:
        message = "ERROR: gh command failed: " + " ".join(["gh", *args])
        stderr = proc.stderr.strip()
        if stderr:
            message += f": {stderr}"
        if report_only:
            print(message)
            return None
        raise HelperError(message)
    return [line for line in proc.stdout.splitlines() if line.strip()]


def read_pattern_file(path: str) -> list[str]:
    patterns: list[str] = []
    for line in Path(path).read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped and not stripped.startswith("#"):
            patterns.append(stripped)
    return patterns


def load_patterns(inline: Sequence[str] | None, files: Sequence[str] | None) -> list[str]:
    patterns = list(inline or [])
    for path in files or []:
        patterns.extend(read_pattern_file(path))
    return patterns


def pattern_matches(path: str, pattern: str) -> bool:
    pattern = pattern.strip()
    if not pattern:
        return False
    if pattern.endswith("/**"):
        return path == pattern[:-3].rstrip("/") or path.startswith(pattern[:-3])
    if pattern.endswith("/"):
        return path.startswith(pattern)
    if any(ch in pattern for ch in "*?["):
        return fnmatch.fnmatch(path, pattern)
    return path == pattern or path.startswith(pattern.rstrip("/") + "/")


def any_pattern_matches(path: str, patterns: Sequence[str]) -> bool:
    return any(pattern_matches(path, pattern) for pattern in patterns)


def parse_queue(path: Path) -> list[QueueItem]:
    text = path.read_text(encoding="utf-8")
    items: list[QueueItem] = []
    pattern = re.compile(
        r"^### (NA-\d+[A-Z]?) — ([^\n]+)\n(?P<body>.*?)(?=^### |\Z)",
        re.M | re.S,
    )
    for match in pattern.finditer(text):
        body = match.group("body")
        status_match = re.search(r"(?m)^\s*-?\s*Status:\s*([A-Z_]+)\b", body)
        status = status_match.group(1) if status_match else "MISSING"
        items.append(QueueItem(match.group(1), match.group(2), status, body))
    return items


def queue_command(args: argparse.Namespace) -> int:
    items = parse_queue(Path(args.file))
    ready = [(item.na, item.title) for item in items if item.status == "READY"]
    print("READY_COUNT", len(ready))
    for na, title in ready:
        print("READY", na, title)
    selections = args.select or DEFAULT_NA_SELECTION
    by_na = {item.na: item for item in items}
    for na in selections:
        item = by_na.get(na)
        if item:
            print(item.na, item.status, item.title)
    if len(ready) != 1 and not args.allow_nonready_count:
        return 2
    return 0


def decision_ids(path: Path) -> list[str]:
    ids: list[str] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        heading = re.match(r"^###\s+(D-\d{4})\b", line)
        bullet = re.match(r"^- \*\*ID:\*\*\s*(D-\d{4})\b", line)
        if heading:
            ids.append(heading.group(1))
        if bullet:
            ids.append(bullet.group(1))
    return ids


def decisions_command(args: argparse.Namespace) -> int:
    ids = decision_ids(Path(args.file))
    counts: dict[str, int] = {}
    for decision_id in ids:
        counts[decision_id] = counts.get(decision_id, 0) + 1
    duplicates = {key: value for key, value in counts.items() if value > 1}
    print("DECISION_ENTRY_COUNT", len(ids))
    if ids:
        print("LATEST_DECISION_ENTRY", ids[-1])
    for decision_id in args.select or DEFAULT_DECISION_SELECTION:
        print(decision_id, counts.get(decision_id, 0))
    print("DUPLICATE_COUNT", len(duplicates))
    for decision_id in sorted(duplicates):
        print("DUPLICATE", decision_id, duplicates[decision_id])
    return 2 if duplicates else 0


def changed_paths(base: str, head: str = "HEAD") -> list[str]:
    proc = run(["git", "diff", "--name-only", f"{base}...{head}"], check=True)
    return sorted(line for line in proc.stdout.splitlines() if line.strip())


def scope_guard_command(args: argparse.Namespace) -> int:
    allowed = load_patterns(args.allowed, args.allowed_file)
    forbidden = load_patterns(args.forbidden, args.forbidden_file)
    paths = changed_paths(args.base, args.head)
    failures: list[str] = []
    print("CHANGED_PATH_COUNT", len(paths))
    for path in paths:
        explicitly_forbidden = any_pattern_matches(path, forbidden)
        allowed_match = any_pattern_matches(path, allowed) if allowed else True
        classification = "allowed"
        if explicitly_forbidden or not allowed_match:
            classification = "forbidden"
            failures.append(path)
        print(f"PATH {classification} {path}")
    print("FORBIDDEN_COUNT", len(failures))
    return 2 if failures else 0


def pr_head_sha(repo: str, pr_number: int, report_only: bool) -> str | None:
    data = gh_json(["api", f"/repos/{repo}/pulls/{pr_number}"], report_only=report_only)
    if not isinstance(data, dict):
        return None
    head = data.get("head") or {}
    sha = head.get("sha")
    return sha if isinstance(sha, str) else None


def check_runs_for_sha(repo: str, sha: str, report_only: bool) -> list[dict] | None:
    data = gh_json(
        ["api", f"/repos/{repo}/commits/{sha}/check-runs?per_page=100"],
        report_only=report_only,
    )
    if not isinstance(data, dict):
        return None
    runs = data.get("check_runs")
    return runs if isinstance(runs, list) else []


def latest_by_name(runs: Iterable[dict]) -> dict[str, dict]:
    latest: dict[str, dict] = {}
    for run_data in runs:
        name = run_data.get("name")
        if not isinstance(name, str):
            continue
        previous = latest.get(name)
        current_key = (
            str(run_data.get("completed_at") or ""),
            str(run_data.get("started_at") or ""),
            int(run_data.get("id") or 0),
        )
        previous_key = (
            str((previous or {}).get("completed_at") or ""),
            str((previous or {}).get("started_at") or ""),
            int((previous or {}).get("id") or 0),
        )
        if previous is None or current_key >= previous_key:
            latest[name] = run_data
    return latest


def conclusion_is_red(conclusion: str | None) -> bool:
    return conclusion not in (None, "success", "neutral", "skipped")


def context_passes(name: str, status: str | None, conclusion: str | None, allow_codeql_neutral: bool) -> bool:
    if status != "completed":
        return False
    if name == "CodeQL" and conclusion == "neutral":
        return allow_codeql_neutral
    return conclusion in ("success", "neutral", "skipped")


def checks_summary_command(args: argparse.Namespace) -> int:
    if bool(args.pr) == bool(args.sha):
        raise HelperError("provide exactly one of --pr or --sha")
    sha = args.sha
    if args.pr:
        sha = pr_head_sha(args.repo, args.pr, args.report_only)
    if not sha:
        return 1 if args.report_only else 2
    print("CHECKS_SHA", sha)
    runs = check_runs_for_sha(args.repo, sha, args.report_only)
    if runs is None:
        return 1 if args.report_only else 2
    by_name = latest_by_name(runs)
    failures: list[str] = []
    for context in REQUIRED_CONTEXTS:
        run_data = by_name.get(context)
        if not run_data:
            print(f"CHECK {context} status=missing conclusion=missing url=")
            failures.append(context)
            continue
        status = run_data.get("status")
        conclusion = run_data.get("conclusion")
        url = run_data.get("html_url") or run_data.get("details_url") or ""
        print(f"CHECK {context} status={status} conclusion={conclusion} url={url}")
        if not context_passes(context, status, conclusion, args.allow_codeql_neutral):
            failures.append(context)
    print("REQUIRED_CONTEXT_FAILURE_COUNT", len(failures))
    return 0 if args.report_only or not failures else 2


def default_sha() -> str:
    proc = run(["git", "rev-parse", "origin/main"])
    if proc.returncode == 0:
        return proc.stdout.strip()
    return run(["git", "rev-parse", "HEAD"], check=True).stdout.strip()


def public_safety_status_command(args: argparse.Namespace) -> int:
    sha = args.sha or default_sha()
    print("PUBLIC_SAFETY_SHA", sha)
    runs = check_runs_for_sha(args.repo, sha, args.report_only)
    if runs is None:
        return 1 if args.report_only else 2
    by_name = latest_by_name(runs)
    names = [
        "public-safety",
        "qsc-linux-full-suite",
        "macos-qsc-full-serial",
        "qsc-adversarial-smoke",
    ]
    public_safety_red = False
    public_safety_ambiguous = False
    for name in names:
        run_data = by_name.get(name)
        if not run_data:
            print(f"CHECK {name} status=missing conclusion=missing url=")
            if name == "public-safety":
                public_safety_ambiguous = True
            continue
        status = run_data.get("status")
        conclusion = run_data.get("conclusion")
        url = run_data.get("html_url") or run_data.get("details_url") or ""
        print(f"CHECK {name} status={status} conclusion={conclusion} url={url}")
        if name == "public-safety":
            public_safety_red = status == "completed" and conclusion_is_red(conclusion)
            public_safety_ambiguous = status != "completed"
    if public_safety_red:
        print("PUBLIC_SAFETY_RED yes")
        return 0 if args.report_only else 2
    print("PUBLIC_SAFETY_RED no")
    if public_safety_ambiguous:
        print("PUBLIC_SAFETY_AMBIGUOUS yes")
        return 0 if args.report_only else 2
    print("PUBLIC_SAFETY_AMBIGUOUS no")
    return 0


def ignored_path(path: Path) -> bool:
    parts = set(path.parts)
    return bool(parts & IGNORED_LINK_PARTS)


def markdown_files(root: Path) -> list[Path]:
    files: list[Path] = []
    for path in root.rglob("*.md"):
        rel = path.relative_to(root)
        if ignored_path(rel):
            continue
        files.append(path)
    return sorted(files)


def link_targets(text: str) -> Iterable[str]:
    link_re = re.compile(r"\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)")
    for raw in link_re.findall(text):
        target = raw.strip()
        if target.startswith("<") and target.endswith(">"):
            target = target[1:-1]
        yield unquote(target.strip())


def link_check_command(args: argparse.Namespace) -> int:
    root = Path(args.root).resolve()
    missing: list[tuple[str, str]] = []
    for md in markdown_files(root):
        text = md.read_text(encoding="utf-8", errors="replace")
        for target in link_targets(text):
            if not target or target.startswith("#"):
                continue
            if "://" in target or target.startswith("mailto:"):
                continue
            if target.startswith("/"):
                candidate = root / target.lstrip("/")
            else:
                candidate = (md.parent / target).resolve()
            try:
                candidate.relative_to(root)
            except ValueError:
                missing.append((md.relative_to(root).as_posix(), target))
                continue
            if not candidate.exists():
                missing.append((md.relative_to(root).as_posix(), target))
    for src, target in missing[:50]:
        print(f"MISSING_LINK {src} -> {target}")
    if len(missing) > 50:
        print(f"MISSING_LINK_TRUNCATED {len(missing) - 50}")
    print("TOTAL_MISSING", len(missing))
    return 2 if missing else 0


@dataclass(frozen=True)
class ScanLine:
    path: str
    line_no: int
    text: str


@dataclass(frozen=True)
class SecretFinding:
    rule: str
    path: str
    line_no: int


def text_files(paths: Sequence[str] | None) -> list[Path]:
    if paths:
        return [Path(path) for path in paths]
    proc = run(["git", "ls-files"], check=True)
    return [Path(line) for line in proc.stdout.splitlines() if line.strip()]


def full_scan_lines(paths: Sequence[str] | None) -> Iterable[ScanLine]:
    for path in text_files(paths):
        if not path.exists() or path.is_dir() or ignored_path(path):
            continue
        try:
            lines = path.read_text(encoding="utf-8").splitlines()
        except UnicodeDecodeError:
            continue
        for idx, line in enumerate(lines, start=1):
            yield ScanLine(path.as_posix(), idx, line)


def added_scan_lines(base: str, paths: Sequence[str] | None) -> Iterable[ScanLine]:
    cmd = ["git", "diff", "--unified=0", f"{base}...HEAD"]
    if paths:
        cmd.extend(["--", *paths])
    proc = run(cmd, check=True)
    current_path = ""
    new_line = 0
    for line in proc.stdout.splitlines():
        if line.startswith("+++ b/"):
            current_path = line[len("+++ b/") :]
            continue
        hunk = re.match(r"@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@", line)
        if hunk:
            new_line = int(hunk.group(1))
            continue
        if line.startswith("+") and not line.startswith("+++"):
            yield ScanLine(current_path, new_line, line[1:])
            new_line += 1
        elif line.startswith("-") and not line.startswith("---"):
            continue
        elif current_path:
            new_line += 1


def secret_findings(lines: Iterable[ScanLine]) -> tuple[int, list[SecretFinding]]:
    line_count = 0
    findings: list[SecretFinding] = []
    for scan_line in lines:
        line_count += 1
        for name, pattern in SECRET_PATTERNS:
            if pattern.search(scan_line.text):
                findings.append(SecretFinding(name, scan_line.path, scan_line.line_no))
                break
    return line_count, findings


def leak_scan_command(args: argparse.Namespace) -> int:
    if args.mode == "full":
        lines = full_scan_lines(args.paths)
    else:
        lines = added_scan_lines(args.base, args.paths)
    line_count, findings = secret_findings(lines)
    print("SCAN_MODE", args.mode)
    print("SCAN_LINE_COUNT", line_count)
    print("SECRET_FINDING_COUNT", len(findings))
    for finding in findings[:20]:
        print(
            "SECRET_FINDING "
            f"type={finding.rule} path={finding.path} line={finding.line_no} redaction=[redacted]"
        )
    if len(findings) > 20:
        print(f"SECRET_FINDING_TRUNCATED {len(findings) - 20}")
    return 2 if findings else 0


def pr_body_preflight_command(args: argparse.Namespace) -> int:
    if args.file:
        body = Path(args.file).read_text(encoding="utf-8")
    else:
        body = sys.stdin.read()
    missing = []
    for field in PR_BODY_FIELDS:
        field_re = re.compile(rf"(?m)^\s*{re.escape(field)}\s*\S+")
        if not field_re.search(body):
            missing.append(field)
    overclaims: list[str] = []
    if args.scan_overclaims:
        lower = body.lower()
        overclaims = [phrase for phrase in OVERCLAIM_PHRASES if phrase.lower() in lower]
    for field in missing:
        print("MISSING_FIELD", field)
    for phrase in overclaims:
        print("PROHIBITED_PHRASE", phrase)
    print("MISSING_FIELD_COUNT", len(missing))
    print("PROHIBITED_PHRASE_COUNT", len(overclaims))
    return 2 if missing or overclaims else 0


def pr_files(repo: str, pr_number: int, report_only: bool) -> list[str] | None:
    lines = gh_lines(
        [
            "api",
            "--paginate",
            f"/repos/{repo}/pulls/{pr_number}/files",
            "--jq",
            ".[].filename",
        ],
        report_only=report_only,
    )
    return sorted(lines) if lines is not None else None


def classify_paths(paths: Sequence[str]) -> tuple[bool, bool, bool, str]:
    docs_only = True
    workflow_security = False
    runtime_critical = False
    for path in paths:
        docs_path = (
            re.match(r"^tests/.+\.md$", path)
            or path
            in {
                "NEXT_ACTIONS.md",
                "TRACEABILITY.md",
                "DECISIONS.md",
                "STATUS.md",
                "README.md",
                "SECURITY.md",
                "SUPPORT.md",
                "CONTRIBUTING.md",
                "CODE_OF_CONDUCT.md",
                "THIRD_PARTY_NOTICES.md",
                "LICENSE",
            }
            or path.startswith("docs/")
        )
        workflow_path = path.startswith(".github/workflows/") or path.startswith(".github/actions/") or path.startswith("scripts/ci/")
        if workflow_path:
            docs_only = False
            workflow_security = True
        elif docs_path:
            continue
        else:
            docs_only = False
            runtime_critical = True
    scope_class = "docs_only"
    if runtime_critical and workflow_security:
        scope_class = "runtime_and_workflow"
    elif runtime_critical:
        scope_class = "runtime_critical"
    elif workflow_security:
        scope_class = "workflow_security"
    return docs_only, workflow_security, runtime_critical, scope_class


def main_public_safety_red(repo: str, report_only: bool) -> tuple[bool | None, str | None]:
    sha = default_sha()
    runs = check_runs_for_sha(repo, sha, report_only)
    if runs is None:
        return None, sha
    run_data = latest_by_name(runs).get("public-safety")
    if not run_data:
        return None, sha
    status = run_data.get("status")
    conclusion = run_data.get("conclusion")
    return bool(status == "completed" and conclusion_is_red(conclusion)), sha


def workflow_dispatch_reliance(repo: str, sha: str, report_only: bool) -> tuple[str, str]:
    data = gh_json(
        ["api", f"/repos/{repo}/actions/runs?head_sha={sha}&per_page=100"],
        report_only=report_only,
    )
    if not isinstance(data, dict):
        return "unknown", "unknown"
    runs = data.get("workflow_runs")
    if not isinstance(runs, list):
        return "unknown", "unknown"
    events = {str(item.get("event")) for item in runs if item.get("event")}
    if "workflow_dispatch" in events and "pull_request" not in events and "pull_request_target" not in events:
        return "yes", "unknown"
    if "workflow_dispatch" in events:
        return "partial", "unknown"
    return "no", "n/a"


def ci_admission_preflight_command(args: argparse.Namespace) -> int:
    sha = pr_head_sha(args.repo, args.pr, args.report_only)
    files = pr_files(args.repo, args.pr, args.report_only)
    if sha is None or files is None:
        return 1 if args.report_only else 2
    docs_only, workflow_security, runtime_critical, scope_class = classify_paths(files)
    triggered = not docs_only
    red, main_sha = main_public_safety_red(args.repo, args.report_only)
    dispatch_reliance, rollup_accepts = workflow_dispatch_reliance(args.repo, sha, args.report_only)
    circular = "unknown"
    if red is False:
        circular = "no"
    elif red is True and workflow_security:
        circular = "yes"
    elif red is True:
        circular = "unknown"
    likely_pass = "yes"
    if red is True or circular == "yes":
        likely_pass = "no"
    elif red is None or circular == "unknown":
        likely_pass = "unknown"

    print("PR", args.pr)
    print("PR_HEAD_SHA", sha)
    print("CHANGED_FILE_COUNT", len(files))
    for path in files:
        print("CHANGED_FILE", path)
    print("SCOPE_CLASS", scope_class)
    print("DOCS_ONLY", "yes" if docs_only else "no")
    print("WORKFLOW_SECURITY", "yes" if workflow_security else "no")
    print("RUNTIME_CRITICAL", "yes" if runtime_critical else "no")
    print("QSC_ADVERSARIAL_TRIGGERED", "yes" if triggered else "no")
    print("MAIN_SHA", main_sha or "unknown")
    print("PUBLIC_SAFETY_RED_ON_MAIN", "unknown" if red is None else ("yes" if red else "no"))
    print("WORKFLOW_DISPATCH_RELIED_ON", dispatch_reliance)
    print("PR_ROLLUP_ACCEPTS_WORKFLOW_DISPATCH", rollup_accepts)
    print("HELPER_PR_CAN_LIKELY_PASS_REQUIRED_CHECKS", likely_pass)
    print("CIRCULAR_DEPENDENCY_RISK", circular)
    if args.report_only:
        return 0
    return 2 if circular != "no" or likely_pass != "yes" else 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Read-only QSL evidence, governance, and CI diagnostics helper."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    queue = subparsers.add_parser("queue", help="Parse NEXT_ACTIONS.md READY state.")
    queue.add_argument("--file", default="NEXT_ACTIONS.md")
    queue.add_argument("--select", action="append", help="NA id to print; repeatable.")
    queue.add_argument("--allow-nonready-count", action="store_true")
    queue.set_defaults(func=queue_command)

    decisions = subparsers.add_parser("decisions", help="Parse DECISIONS.md entries.")
    decisions.add_argument("--file", default="DECISIONS.md")
    decisions.add_argument("--select", action="append", help="Decision id to print; repeatable.")
    decisions.set_defaults(func=decisions_command)

    scope = subparsers.add_parser("scope-guard", help="Classify changed paths against allow/deny patterns.")
    scope.add_argument("--base", required=True)
    scope.add_argument("--head", default="HEAD")
    scope.add_argument("--allowed", action="append", help="Allowed path/glob/prefix; repeatable.")
    scope.add_argument("--allowed-file", action="append", help="File containing allowed patterns.")
    scope.add_argument("--forbidden", action="append", help="Forbidden path/glob/prefix; repeatable.")
    scope.add_argument("--forbidden-file", action="append", help="File containing forbidden patterns.")
    scope.set_defaults(func=scope_guard_command)

    checks = subparsers.add_parser("checks-summary", help="Summarize required checks for a PR or SHA.")
    target = checks.add_mutually_exclusive_group(required=True)
    target.add_argument("--pr", type=int)
    target.add_argument("--sha")
    checks.add_argument("--repo", default=REPO_DEFAULT)
    checks.add_argument("--report-only", action="store_true")
    checks.add_argument("--allow-codeql-neutral", action="store_true")
    checks.set_defaults(func=checks_summary_command)

    public_safety = subparsers.add_parser("public-safety-status", help="Summarize public-safety state.")
    public_safety.add_argument("--sha")
    public_safety.add_argument("--repo", default=REPO_DEFAULT)
    public_safety.add_argument("--report-only", action="store_true")
    public_safety.set_defaults(func=public_safety_status_command)

    link = subparsers.add_parser("link-check", help="Check relative markdown links.")
    link.add_argument("--root", default=".")
    link.set_defaults(func=link_check_command)

    leak = subparsers.add_parser("leak-scan", help="Scan added lines or files for high-confidence secrets.")
    leak.add_argument("--mode", choices=("added", "full"), default="added")
    leak.add_argument("--base", default="origin/main")
    leak.add_argument("--paths", nargs="*")
    leak.set_defaults(func=leak_scan_command)

    pr_body = subparsers.add_parser("pr-body-preflight", help="Validate required PR body metadata.")
    pr_body.add_argument("--file")
    overclaim_group = pr_body.add_mutually_exclusive_group()
    overclaim_group.add_argument("--scan-overclaims", dest="scan_overclaims", action="store_true", default=True)
    overclaim_group.add_argument("--no-overclaim-scan", dest="scan_overclaims", action="store_false")
    pr_body.set_defaults(func=pr_body_preflight_command)

    admission = subparsers.add_parser("ci-admission-preflight", help="Read-only CI/admission feasibility summary.")
    admission.add_argument("--pr", type=int, required=True)
    admission.add_argument("--repo", default=REPO_DEFAULT)
    admission.add_argument("--report-only", action="store_true")
    admission.set_defaults(func=ci_admission_preflight_command)

    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return int(args.func(args))
    except HelperError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
