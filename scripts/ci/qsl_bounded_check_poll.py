#!/usr/bin/env python3
"""Bounded CI/public-safety polling helper for QSL local operations.

The helper is intentionally standalone: it uses only the Python standard
library, talks to GitHub through `gh api` in live mode, and supports fixture
mode for deterministic no-network tests.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable
from urllib.parse import quote


DEFAULT_REPO = "QuantumShieldLabs/qsl-protocol"
PUBLIC_SAFETY = "public-safety"
RED_CONCLUSIONS = {
    "action_required",
    "cancelled",
    "failure",
    "startup_failure",
    "timed_out",
}
DOCS_ONLY_SKIP_NAMES = {
    "ci-4a",
    "ci-4b",
    "ci-4c",
    "ci-4d",
    "ci-4d-dur",
    "demo-cli-build",
    "demo-cli-smoke",
    "formal-scka-model",
    "macos-qsc-qshield-build",
    "metadata-conformance-smoke",
    "qsc-adversarial-smoke",
    "qsc-linux-full-suite",
    "suite2-vectors",
}


class HelperError(RuntimeError):
    """Expected helper failure with a concise operator-facing message."""


@dataclass(frozen=True)
class ApiError:
    endpoint: str
    status: int | None
    message: str


@dataclass(frozen=True)
class CheckRun:
    name: str
    status: str
    conclusion: str | None = None
    url: str | None = None
    details_url: str | None = None
    workflow_name: str | None = None
    run_id: int | None = None
    check_run_id: int | None = None
    started_at: str | None = None
    completed_at: str | None = None

    @classmethod
    def from_json(cls, value: dict[str, Any]) -> "CheckRun":
        return cls(
            name=str(value.get("name") or ""),
            status=str(value.get("status") or "unknown"),
            conclusion=value.get("conclusion"),
            url=value.get("html_url") or value.get("url"),
            details_url=value.get("details_url") or value.get("html_url") or value.get("url"),
            workflow_name=(value.get("workflow_name") or value.get("workflowName")),
            run_id=_as_int(value.get("run_id") or value.get("runId")),
            check_run_id=_as_int(value.get("id")),
            started_at=value.get("started_at") or value.get("startedAt"),
            completed_at=value.get("completed_at") or value.get("completedAt"),
        )


@dataclass(frozen=True)
class Target:
    mode: str
    repo: str
    sha: str | None = None
    pr: int | None = None
    branch: str | None = None


@dataclass(frozen=True)
class Snapshot:
    target: Target
    check_runs: list[CheckRun]
    required_contexts: list[str] = field(default_factory=list)
    api_errors: list[ApiError] = field(default_factory=list)


@dataclass
class Policy:
    mode: str
    required: bool = False
    all_checks: bool = False
    public_safety: bool = False
    report_only: bool = False
    allow_docs_only_skips: bool = False
    docs_only: bool = False
    allow_codeql_neutral: bool = False
    allow_codeql_skipped: bool = False
    max_iters: int = 1
    interval: float = 0.0
    required_contexts: list[str] = field(default_factory=list)
    markers: list[str] = field(default_factory=list)

    @classmethod
    def from_mapping(cls, mode: str, value: dict[str, Any]) -> "Policy":
        return cls(
            mode=mode,
            required=bool(value.get("required", False)),
            all_checks=bool(value.get("all_checks", False)),
            public_safety=bool(value.get("public_safety", False)),
            report_only=bool(value.get("report_only", False)),
            allow_docs_only_skips=bool(value.get("allow_docs_only_skips", False)),
            docs_only=bool(value.get("docs_only", False)),
            allow_codeql_neutral=bool(value.get("allow_codeql_neutral", False)),
            allow_codeql_skipped=bool(value.get("allow_codeql_skipped", False)),
            max_iters=int(value.get("max_iters", 1)),
            interval=float(value.get("interval", 0.0)),
            required_contexts=[str(item) for item in value.get("required_contexts", [])],
            markers=[str(item) for item in value.get("markers", [])],
        )


@dataclass
class Evaluation:
    verdict: str
    exit_code: int
    reason: str
    checks: list[dict[str, Any]]
    failures: list[dict[str, Any]] = field(default_factory=list)
    pending: list[dict[str, Any]] = field(default_factory=list)
    missing_required: list[str] = field(default_factory=list)
    stale_failures: list[dict[str, Any]] = field(default_factory=list)
    api_errors: list[dict[str, Any]] = field(default_factory=list)
    markers: list[str] = field(default_factory=list)

    @property
    def done(self) -> bool:
        return self.verdict not in {"pending", "api_retry"}


def _as_int(value: Any) -> int | None:
    if value is None:
        return None
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def parse_time(value: str | None) -> datetime:
    if not value:
        return datetime.min.replace(tzinfo=timezone.utc)
    normalized = value[:-1] + "+00:00" if value.endswith("Z") else value
    try:
        parsed = datetime.fromisoformat(normalized)
    except ValueError:
        return datetime.min.replace(tzinfo=timezone.utc)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed


def check_sort_key(check: CheckRun) -> tuple[datetime, datetime, int, int]:
    return (
        parse_time(check.completed_at),
        parse_time(check.started_at),
        check.run_id or -1,
        check.check_run_id or -1,
    )


def latest_by_name(checks: Iterable[CheckRun]) -> dict[str, CheckRun]:
    latest: dict[str, CheckRun] = {}
    for check in checks:
        if not check.name:
            continue
        previous = latest.get(check.name)
        if previous is None or check_sort_key(check) >= check_sort_key(previous):
            latest[check.name] = check
    return latest


def is_red(check: CheckRun) -> bool:
    if check.status != "completed":
        return False
    return (check.conclusion or "").lower() in RED_CONCLUSIONS


def check_record(check: CheckRun, *, accepted: bool | None = None, note: str | None = None) -> dict[str, Any]:
    return {
        "name": check.name,
        "status": check.status,
        "conclusion": check.conclusion,
        "accepted": accepted,
        "url": check.details_url or check.url,
        "workflow_name": check.workflow_name,
        "run_id": check.run_id,
        "check_run_id": check.check_run_id,
        "note": note,
    }


def api_error_record(error: ApiError) -> dict[str, Any]:
    return {"endpoint": error.endpoint, "status": error.status, "message": error.message}


def accepted_completed(check: CheckRun, policy: Policy) -> tuple[bool, str]:
    conclusion = (check.conclusion or "").lower()
    if check.status != "completed":
        return False, "not_completed"
    if conclusion == "success":
        return True, "success"
    if conclusion == "neutral" and check.name == "CodeQL" and policy.allow_codeql_neutral:
        return True, "codeql_neutral_allowed"
    if conclusion == "skipped" and check.name == "CodeQL" and policy.allow_codeql_skipped:
        return True, "codeql_skipped_allowed"
    if conclusion == "skipped" and policy.docs_only and policy.allow_docs_only_skips:
        if check.name in DOCS_ONLY_SKIP_NAMES:
            return True, "docs_only_skip_allowed"
    return False, f"unaccepted_{conclusion or 'missing_conclusion'}"


def stale_failure_records(checks: list[CheckRun], latest: dict[str, CheckRun]) -> list[dict[str, Any]]:
    stale: list[dict[str, Any]] = []
    for check in checks:
        current = latest.get(check.name)
        if current is None or current is check:
            continue
        if current.check_run_id == check.check_run_id:
            continue
        if is_red(check):
            stale.append(check_record(check, accepted=False, note="stale_failed_rerun"))
    return stale


def evaluate(snapshot: Snapshot, policy: Policy, *, timed_out: bool = False) -> Evaluation:
    latest = latest_by_name(snapshot.check_runs)
    checks: list[dict[str, Any]] = []
    failures: list[dict[str, Any]] = []
    pending: list[dict[str, Any]] = []
    missing: list[str] = []
    stale = stale_failure_records(snapshot.check_runs, latest)
    api_errors = [api_error_record(error) for error in snapshot.api_errors]

    def record_required(name: str) -> None:
        check = latest.get(name)
        if check is None:
            missing.append(name)
            return
        accepted, note = accepted_completed(check, policy)
        checks.append(check_record(check, accepted=accepted, note=note))
        if check.status != "completed":
            pending.append(check_record(check, accepted=False, note="not_completed"))
        elif not accepted:
            failures.append(check_record(check, accepted=False, note=note))

    if policy.report_only:
        for check in sorted(latest.values(), key=lambda item: item.name):
            accepted, note = accepted_completed(check, policy)
            checks.append(check_record(check, accepted=accepted, note=note))
        return Evaluation(
            verdict="report_only",
            exit_code=0,
            reason="summary_only",
            checks=checks,
            failures=[],
            pending=[],
            missing_required=[],
            stale_failures=stale,
            api_errors=api_errors,
            markers=policy.markers,
        )

    if policy.public_safety:
        record_required(PUBLIC_SAFETY)
    elif policy.required:
        required_contexts = policy.required_contexts or snapshot.required_contexts
        if not required_contexts:
            return Evaluation(
                verdict="failure",
                exit_code=2,
                reason="ambiguous_required_state_no_required_contexts",
                checks=[],
                api_errors=api_errors,
                markers=policy.markers,
            )
        for name in required_contexts:
            record_required(name)
    elif policy.all_checks:
        if not latest:
            missing.append("all_check_runs")
        for check in sorted(latest.values(), key=lambda item: item.name):
            accepted, note = accepted_completed(check, policy)
            checks.append(check_record(check, accepted=accepted, note=note))
            if check.status != "completed":
                pending.append(check_record(check, accepted=False, note="not_completed"))
            elif not accepted:
                failures.append(check_record(check, accepted=False, note=note))
    else:
        return Evaluation(
            verdict="failure",
            exit_code=2,
            reason="no_gate_policy_selected",
            checks=[],
            api_errors=api_errors,
            markers=policy.markers,
        )

    if failures:
        return Evaluation(
            verdict="failure",
            exit_code=1,
            reason="red_or_unaccepted_check",
            checks=checks,
            failures=failures,
            pending=pending,
            missing_required=missing,
            stale_failures=stale,
            api_errors=api_errors,
            markers=policy.markers,
        )
    if pending or missing:
        if timed_out:
            reason = "api_failure_timeout" if api_errors and not checks else "timeout_pending_or_missing"
            return Evaluation(
                verdict="failure",
                exit_code=1,
                reason=reason,
                checks=checks,
                failures=failures,
                pending=pending,
                missing_required=missing,
                stale_failures=stale,
                api_errors=api_errors,
                markers=policy.markers,
            )
        verdict = "api_retry" if api_errors and not checks else "pending"
        reason = "api_error_retry" if verdict == "api_retry" else "pending_or_missing"
        return Evaluation(
            verdict=verdict,
            exit_code=1,
            reason=reason,
            checks=checks,
            failures=failures,
            pending=pending,
            missing_required=missing,
            stale_failures=stale,
            api_errors=api_errors,
            markers=policy.markers,
        )
    return Evaluation(
        verdict="success",
        exit_code=0,
        reason="requested_condition_satisfied",
        checks=checks,
        failures=[],
        pending=[],
        missing_required=[],
        stale_failures=stale,
        api_errors=api_errors,
        markers=policy.markers,
    )


def gh_api(endpoint: str) -> Any:
    proc = subprocess.run(
        ["gh", "api", endpoint, "-H", "Accept: application/vnd.github+json"],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if proc.returncode != 0:
        raise HelperError(proc.stderr.strip() or f"gh api failed for {endpoint}")
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise HelperError(f"gh api returned malformed JSON for {endpoint}") from exc


def gh_api_or_error(endpoint: str) -> tuple[Any | None, ApiError | None]:
    try:
        return gh_api(endpoint), None
    except HelperError as exc:
        status = None
        message = str(exc)
        for candidate in ("404", "403", "500", "502", "503", "504"):
            if candidate in message:
                status = int(candidate)
                break
        return None, ApiError(endpoint=endpoint, status=status, message=message)


def fetch_check_runs(repo: str, sha: str) -> tuple[list[CheckRun], list[ApiError]]:
    checks: list[CheckRun] = []
    errors: list[ApiError] = []
    for page in range(1, 6):
        endpoint = f"/repos/{repo}/commits/{sha}/check-runs?per_page=100&page={page}"
        data, error = gh_api_or_error(endpoint)
        if error:
            errors.append(error)
            break
        assert isinstance(data, dict)
        page_runs = [CheckRun.from_json(item) for item in data.get("check_runs", [])]
        checks.extend(page_runs)
        total = int(data.get("total_count", len(checks)))
        if len(checks) >= total or not page_runs:
            break
    return checks, errors


def fetch_live_snapshot(args: argparse.Namespace, policy: Policy) -> Snapshot:
    repo = args.repo
    api_errors: list[ApiError] = []
    required_contexts: list[str] = []
    target = Target(mode=policy.mode, repo=repo, sha=getattr(args, "sha", None), pr=getattr(args, "pr", None))

    if policy.mode == "pr":
        pr_endpoint = f"/repos/{repo}/pulls/{args.pr}"
        pr_data, error = gh_api_or_error(pr_endpoint)
        if error:
            return Snapshot(target=target, check_runs=[], api_errors=[error])
        assert isinstance(pr_data, dict)
        sha = pr_data.get("head", {}).get("sha")
        base_branch = pr_data.get("base", {}).get("ref")
        target = Target(mode="pr", repo=repo, sha=sha, pr=args.pr, branch=base_branch)
        if policy.required:
            branch = quote(str(base_branch), safe="")
            required_endpoint = f"/repos/{repo}/branches/{branch}/protection/required_status_checks"
            required_data, required_error = gh_api_or_error(required_endpoint)
            if required_error:
                api_errors.append(required_error)
            elif isinstance(required_data, dict):
                required_contexts = [str(item) for item in required_data.get("contexts", [])]
        if not sha:
            return Snapshot(
                target=target,
                check_runs=[],
                required_contexts=required_contexts,
                api_errors=api_errors + [ApiError(pr_endpoint, None, "PR head SHA missing")],
            )
        checks, check_errors = fetch_check_runs(repo, str(sha))
        return Snapshot(
            target=target,
            check_runs=checks,
            required_contexts=required_contexts,
            api_errors=api_errors + check_errors,
        )

    if not target.sha:
        return Snapshot(
            target=target,
            check_runs=[],
            api_errors=[ApiError("argument:sha", None, "SHA is required")],
        )
    checks, check_errors = fetch_check_runs(repo, target.sha)
    return Snapshot(target=target, check_runs=checks, api_errors=check_errors)


def load_fixture(path: Path, policy_name: str) -> tuple[list[Snapshot], Policy]:
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise HelperError(f"malformed fixture JSON: {path}: {exc}") from exc
    if not isinstance(raw, dict):
        raise HelperError(f"fixture root must be an object: {path}")
    policies = raw.get("policies", {})
    if policy_name not in policies:
        raise HelperError(f"fixture policy not found: {policy_name}")
    policy = Policy.from_mapping(str(policies[policy_name].get("mode", raw.get("mode", "fixture"))), policies[policy_name])
    target_raw = raw.get("target", {})
    target = Target(
        mode=str(target_raw.get("mode", policy.mode)),
        repo=str(target_raw.get("repo", DEFAULT_REPO)),
        sha=target_raw.get("sha"),
        pr=_as_int(target_raw.get("pr")),
        branch=target_raw.get("branch"),
    )
    fixture_required = [str(item) for item in raw.get("required_contexts", [])]
    if fixture_required and not policy.required_contexts:
        policy.required_contexts = fixture_required
    fixture_markers = [str(item) for item in raw.get("markers", [])]
    policy.markers = list(dict.fromkeys([*policy.markers, *fixture_markers]))
    iterations = raw.get("iterations")
    if not isinstance(iterations, list) or not iterations:
        raise HelperError(f"fixture must contain non-empty iterations: {path}")
    snapshots: list[Snapshot] = []
    for step in iterations:
        if not isinstance(step, dict):
            raise HelperError(f"fixture iteration must be an object: {path}")
        checks = [CheckRun.from_json(item) for item in step.get("check_runs", [])]
        errors = [
            ApiError(
                endpoint=str(item.get("endpoint", "fixture")),
                status=_as_int(item.get("status")),
                message=str(item.get("message", "")),
            )
            for item in step.get("api_errors", [])
        ]
        snapshots.append(
            Snapshot(
                target=target,
                check_runs=checks,
                required_contexts=[str(item) for item in step.get("required_contexts", fixture_required)],
                api_errors=errors,
            )
        )
    return snapshots, policy


def print_human(snapshot: Snapshot, policy: Policy, evaluation: Evaluation, iteration: int, max_iters: int) -> None:
    target = snapshot.target
    target_parts = [f"mode={target.mode}", f"repo={target.repo}"]
    if target.pr is not None:
        target_parts.append(f"pr={target.pr}")
    if target.sha:
        target_parts.append(f"sha={target.sha}")
    if target.branch:
        target_parts.append(f"branch={target.branch}")
    print(
        "QSL_BOUNDED_CHECK_POLL",
        f"verdict={evaluation.verdict}",
        f"reason={evaluation.reason}",
        f"iter={iteration}/{max_iters}",
        *target_parts,
    )
    for error in evaluation.api_errors:
        print(
            "API_ERROR",
            f"endpoint={error.get('endpoint')}",
            f"status={error.get('status')}",
            f"message={error.get('message')}",
        )
    for check in evaluation.checks:
        print(format_check("CHECK", check))
    for check in evaluation.failures:
        print(format_check("FAIL", check))
    for check in evaluation.pending:
        print(format_check("PENDING", check))
    for name in evaluation.missing_required:
        print("MISSING_REQUIRED", f"name={name}")
    for check in evaluation.stale_failures:
        print(format_check("STALE_FAILURE", check))
    for marker in evaluation.markers:
        print("MARKER", marker)
    if policy.report_only:
        print("REPORT_ONLY true")


def format_check(prefix: str, check: dict[str, Any]) -> str:
    parts = [
        prefix,
        f"name={check.get('name')}",
        f"status={check.get('status')}",
        f"conclusion={check.get('conclusion')}",
        f"accepted={check.get('accepted')}",
    ]
    if check.get("note"):
        parts.append(f"note={check.get('note')}")
    if check.get("run_id") is not None:
        parts.append(f"run_id={check.get('run_id')}")
    if check.get("check_run_id") is not None:
        parts.append(f"check_run_id={check.get('check_run_id')}")
    if check.get("workflow_name"):
        parts.append(f"workflow={check.get('workflow_name')}")
    if check.get("url"):
        parts.append(f"url={check.get('url')}")
    return " ".join(str(part) for part in parts)


def print_json(snapshot: Snapshot, evaluation: Evaluation, iteration: int, max_iters: int) -> None:
    print(
        json.dumps(
            {
                "verdict": evaluation.verdict,
                "exit_code": evaluation.exit_code,
                "reason": evaluation.reason,
                "iteration": iteration,
                "max_iters": max_iters,
                "target": {
                    "mode": snapshot.target.mode,
                    "repo": snapshot.target.repo,
                    "sha": snapshot.target.sha,
                    "pr": snapshot.target.pr,
                    "branch": snapshot.target.branch,
                },
                "checks": evaluation.checks,
                "failures": evaluation.failures,
                "pending": evaluation.pending,
                "missing_required": evaluation.missing_required,
                "stale_failures": evaluation.stale_failures,
                "api_errors": evaluation.api_errors,
                "markers": evaluation.markers,
            },
            sort_keys=True,
        )
    )


def validate_bounds(interval: float, max_iters: int) -> None:
    if max_iters < 1:
        raise HelperError("--max-iters must be >= 1")
    if interval < 0:
        raise HelperError("--interval must be >= 0")


def poll_live(args: argparse.Namespace, policy: Policy) -> int:
    validate_bounds(policy.interval, policy.max_iters)
    last_snapshot = Snapshot(Target(policy.mode, args.repo), [])
    last_evaluation = Evaluation("failure", 2, "not_evaluated", [])
    for index in range(1, policy.max_iters + 1):
        snapshot = fetch_live_snapshot(args, policy)
        timed_out = index == policy.max_iters
        evaluation = evaluate(snapshot, policy, timed_out=timed_out)
        last_snapshot = snapshot
        last_evaluation = evaluation
        if evaluation.done:
            emit(args, snapshot, policy, evaluation, index, policy.max_iters)
            return evaluation.exit_code
        emit(args, snapshot, policy, evaluation, index, policy.max_iters)
        if not timed_out and policy.interval:
            time.sleep(policy.interval)
    emit(args, last_snapshot, policy, last_evaluation, policy.max_iters, policy.max_iters)
    return last_evaluation.exit_code


def run_fixture(args: argparse.Namespace) -> int:
    snapshots, policy = load_fixture(Path(args.fixture), args.policy)
    if args.max_iters is not None:
        policy.max_iters = args.max_iters
    if args.interval is not None:
        policy.interval = args.interval
    validate_bounds(policy.interval, policy.max_iters)
    last_evaluation: Evaluation | None = None
    last_snapshot: Snapshot | None = None
    for index in range(1, policy.max_iters + 1):
        snapshot = snapshots[min(index - 1, len(snapshots) - 1)]
        timed_out = index == policy.max_iters
        evaluation = evaluate(snapshot, policy, timed_out=timed_out)
        last_evaluation = evaluation
        last_snapshot = snapshot
        if evaluation.done:
            emit(args, snapshot, policy, evaluation, index, policy.max_iters)
            return evaluation.exit_code
        emit(args, snapshot, policy, evaluation, index, policy.max_iters)
    assert last_evaluation is not None
    assert last_snapshot is not None
    return last_evaluation.exit_code


def emit(args: argparse.Namespace, snapshot: Snapshot, policy: Policy, evaluation: Evaluation, iteration: int, max_iters: int) -> None:
    if getattr(args, "json_output", False):
        print_json(snapshot, evaluation, iteration, max_iters)
    else:
        print_human(snapshot, policy, evaluation, iteration, max_iters)


def pr_command(args: argparse.Namespace) -> int:
    policy = Policy(
        mode="pr",
        required=args.required,
        all_checks=args.all,
        allow_docs_only_skips=args.allow_docs_only_skips,
        docs_only=args.allow_docs_only_skips,
        allow_codeql_neutral=args.allow_codeql_neutral,
        allow_codeql_skipped=args.allow_codeql_skipped,
        max_iters=args.max_iters,
        interval=args.interval,
    )
    return poll_live(args, policy)


def public_safety_command(args: argparse.Namespace) -> int:
    policy = Policy(
        mode="public-safety",
        public_safety=True,
        allow_codeql_neutral=args.allow_codeql_neutral,
        allow_codeql_skipped=args.allow_codeql_skipped,
        max_iters=args.max_iters,
        interval=args.interval,
    )
    return poll_live(args, policy)


def sha_summary_command(args: argparse.Namespace) -> int:
    policy = Policy(
        mode="sha-summary",
        report_only=True,
        allow_docs_only_skips=args.allow_docs_only_skips,
        docs_only=args.allow_docs_only_skips,
        allow_codeql_neutral=True,
        allow_codeql_skipped=True,
        max_iters=1,
        interval=0.0,
    )
    return poll_live(args, policy)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Bounded QSL CI/public-safety polling helper.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    pr = subparsers.add_parser("pr", help="Poll PR check-runs on the PR head SHA.")
    pr.add_argument("--repo", default=DEFAULT_REPO)
    pr.add_argument("--pr", type=int, required=True)
    group = pr.add_mutually_exclusive_group(required=True)
    group.add_argument("--required", action="store_true", help="Gate branch-protection required contexts.")
    group.add_argument("--all", action="store_true", help="Gate all observed PR-head check-runs.")
    pr.add_argument("--interval", type=float, default=10.0)
    pr.add_argument("--max-iters", type=int, default=180)
    pr.add_argument("--allow-docs-only-skips", action="store_true")
    pr.add_argument("--allow-codeql-neutral", action="store_true")
    pr.add_argument("--allow-codeql-skipped", action="store_true")
    pr.add_argument("--json", dest="json_output", action="store_true")
    pr.set_defaults(func=pr_command)

    public = subparsers.add_parser("public-safety", help="Poll public-safety by commit SHA.")
    public.add_argument("--repo", default=DEFAULT_REPO)
    public.add_argument("--sha", required=True)
    public.add_argument("--interval", type=float, default=10.0)
    public.add_argument("--max-iters", type=int, default=180)
    public.add_argument("--allow-codeql-neutral", action="store_true")
    public.add_argument("--allow-codeql-skipped", action="store_true")
    public.add_argument("--json", dest="json_output", action="store_true")
    public.set_defaults(func=public_safety_command)

    summary = subparsers.add_parser("sha-summary", help="Summarize check-runs by SHA without gating PR-only contexts.")
    summary.add_argument("--repo", default=DEFAULT_REPO)
    summary.add_argument("--sha", required=True)
    summary.add_argument("--report-only", action="store_true", help="Required explicit report-only acknowledgement.")
    summary.add_argument("--allow-docs-only-skips", action="store_true")
    summary.add_argument("--json", dest="json_output", action="store_true")
    summary.set_defaults(func=sha_summary_command)

    fixture = subparsers.add_parser("fixture", help="Run a no-network fixture policy.")
    fixture.add_argument("--fixture", required=True)
    fixture.add_argument("--policy", required=True)
    fixture.add_argument("--interval", type=float)
    fixture.add_argument("--max-iters", type=int)
    fixture.add_argument("--json", dest="json_output", action="store_true")
    fixture.set_defaults(func=run_fixture)
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    if args.command == "sha-summary" and not args.report_only:
        parser.error("sha-summary requires --report-only")
    try:
        return int(args.func(args))
    except HelperError as exc:
        if getattr(args, "json_output", False):
            print(json.dumps({"verdict": "error", "exit_code": 2, "reason": str(exc)}))
        else:
            print(f"ERROR {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
