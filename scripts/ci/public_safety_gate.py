#!/usr/bin/env python3
import argparse
import json
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import PurePosixPath


DENYLIST_SUFFIXES = (".pem", ".key", ".p12")
ROOT_DOC_SCAN_NAMES = {
    "README.md",
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


def wait_for_commit_checks(args: argparse.Namespace) -> int:
    for attempt in range(1, args.max_iterations + 1):
        check_runs = commit_check_runs(args.repo, args.sha)
        pending = False
        print(f"ITER={attempt}/{args.max_iterations} sha={args.sha}")
        for check_name in args.required:
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
                    f"ERROR: {check_name} is not green on {args.sha} "
                    f"(conclusion={conclusion})",
                    file=sys.stderr,
                )
                return 1
        if not pending:
            print(f"OK: required checks green on {args.sha}")
            return 0
        if attempt != args.max_iterations:
            time.sleep(args.interval_seconds)
    print(
        f"ERROR: required checks did not settle green on {args.sha} after bounded wait",
        file=sys.stderr,
    )
    return 2


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

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
