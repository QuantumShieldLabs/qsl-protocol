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
NA0237A_REPAIR_PR = 721
NA0237A_EXPECTED_PR708_HEAD = "7f54ea7ab4ae7347af4655183dfb24188cf1a8ce"
NA0237A_READY_ITEM = "NA-0237A"
NA0237A_FAILURE_SUITE_CHECK = "macos-qsc-full-serial"
NA0237A_REPAIR_ALLOWED_PATHS = {
    "DECISIONS.md",
    "NEXT_ACTIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
    "qsl/qsl-client/qsc/tests/send_commit.rs",
    "tests/NA-0237A_send_commit_fallout_repair_testplan.md",
}
NA0237A_REQUIRED_PR_CHECKS = (
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
)


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


def next_actions_statuses(text: str) -> dict[str, str]:
    statuses: dict[str, str] = {}
    current: str | None = None
    for raw_line in text.splitlines():
        heading = re.match(r"^### (NA-\d+[A-Z]?)\b", raw_line)
        if heading:
            current = heading.group(1)
            continue
        if current is None:
            continue
        status = re.match(r"^(?:-\s*)?(?:\*\*)?Status(?:\*\*)?:\s+(.+?)\s*$", raw_line)
        if status:
            statuses[current] = status.group(1).strip()
    return statuses


def require_na0237a_queue(repo: str, ref: str) -> bool:
    statuses = next_actions_statuses(repo_file_text(repo, ref, "NEXT_ACTIONS.md"))
    ready = sorted(item for item, status in statuses.items() if status == "READY")
    print(f"{ref} READY_COUNT={len(ready)}")
    print(f"{ref} READY_ITEMS={','.join(ready)}")
    for item in ("NA-0237", "NA-0237A", "NA-0237B", "NA-0237C", "NA-0237D", "NA-0238"):
        print(f"{ref} {item}={statuses.get(item)}")
    if ready != [NA0237A_READY_ITEM]:
        print(
            f"ERROR: {ref} does not have {NA0237A_READY_ITEM} as the sole READY item",
            file=sys.stderr,
        )
        return False
    required = {
        "NA-0237": "BLOCKED",
        "NA-0237B": "DONE",
        "NA-0237C": "DONE",
        "NA-0237D": "DONE",
        "NA-0238": "BACKLOG",
    }
    for item, expected in required.items():
        if statuses.get(item) != expected:
            print(
                f"ERROR: {ref} {item}={statuses.get(item)}; expected {expected}",
                file=sys.stderr,
            )
            return False
    return True


def require_pr708_unchanged(repo: str) -> bool:
    pr = pull_request(repo, 708)
    state = pr.get("state")
    head_sha = pr["head"]["sha"]
    print(f"PR 708 state={state} head_sha={head_sha}")
    if state != "open" or head_sha != NA0237A_EXPECTED_PR708_HEAD:
        print(
            f"ERROR: PR 708 moved or closed; expected open at {NA0237A_EXPECTED_PR708_HEAD}",
            file=sys.stderr,
        )
        return False
    return True


def require_main_failure_fingerprint(
    repo: str,
    branch: str,
    main_check_runs: list[dict],
    *,
    check_name: str,
    main_advisories_check: str,
    expected_markers: list[str],
    require_advisories_success: bool,
) -> bool:
    if not expected_markers:
        print("ERROR: at least one expected main-failure marker is required", file=sys.stderr)
        return False

    run = latest_run_for_name(main_check_runs, check_name)
    if run is None:
        print(f"ERROR: latest {branch} is missing check '{check_name}'", file=sys.stderr)
        return False
    print(
        f"{branch} check={check_name} status={run.get('status')} "
        f"conclusion={run.get('conclusion')} url={run.get('html_url')}"
    )
    if run.get("status") != "completed" or run.get("conclusion") != "failure":
        print(f"ERROR: latest {branch} {check_name} is not completed/failure", file=sys.stderr)
        return False

    advisories = latest_run_for_name(main_check_runs, main_advisories_check)
    if advisories is None:
        print(
            f"ERROR: latest {branch} is missing check '{main_advisories_check}'",
            file=sys.stderr,
        )
        return False
    print(
        f"{branch} check={main_advisories_check} status={advisories.get('status')} "
        f"conclusion={advisories.get('conclusion')} url={advisories.get('html_url')}"
    )
    if require_advisories_success and (
        advisories.get("status") != "completed" or advisories.get("conclusion") != "success"
    ):
        print(
            f"ERROR: latest {branch} {main_advisories_check} is not completed/success",
            file=sys.stderr,
        )
        return False

    suite = latest_run_for_name(main_check_runs, NA0237A_FAILURE_SUITE_CHECK)
    if suite is None:
        print(
            f"ERROR: latest {branch} is missing check '{NA0237A_FAILURE_SUITE_CHECK}'",
            file=sys.stderr,
        )
        return False
    print(
        f"{branch} check={NA0237A_FAILURE_SUITE_CHECK} status={suite.get('status')} "
        f"conclusion={suite.get('conclusion')} url={suite.get('html_url')}"
    )
    if suite.get("status") != "completed" or suite.get("conclusion") != "failure":
        print(
            f"ERROR: latest {branch} {NA0237A_FAILURE_SUITE_CHECK} is not completed/failure",
            file=sys.stderr,
        )
        return False
    log_text = github_job_log_text(repo, int(suite["id"]))
    missing = [marker for marker in expected_markers if marker not in log_text]
    print(f"MAIN_FAILURE_MARKERS={','.join(expected_markers)}")
    if missing:
        print(
            f"ERROR: latest {branch} failure log is missing expected markers: "
            f"{','.join(missing)}",
            file=sys.stderr,
        )
        return False
    return True


def require_na0237a_repair_paths(files: list[dict]) -> bool:
    paths = sorted(file_info["filename"] for file_info in files)
    print(f"NA0237A_REPAIR_CHANGED_FILE_COUNT={len(paths)}")
    for path in paths:
        print(path)
    disallowed = [path for path in paths if path not in NA0237A_REPAIR_ALLOWED_PATHS]
    if disallowed:
        print("ERROR: PR changes paths outside NA-0237A repair scope", file=sys.stderr)
        for path in disallowed:
            print(path, file=sys.stderr)
        return False
    if "qsl/qsl-client/qsc/tests/send_commit.rs" not in paths:
        print("ERROR: PR does not change qsc send_commit repair path", file=sys.stderr)
        return False
    forbidden_prefixes = (
        ".github/",
        "scripts/",
        "qsc-desktop/",
        "qsl-server/",
        "qsl-attachments/",
        "website/",
    )
    for path in paths:
        if (
            path in ("Cargo.toml", "Cargo.lock")
            or path.startswith(forbidden_prefixes)
            or path.startswith("tools/refimpl/")
        ):
            print(f"ERROR: forbidden path changed: {path}", file=sys.stderr)
            return False
    return True


def require_target_pr_checks(repo: str, pr_head_sha: str) -> bool:
    check_runs = commit_check_runs(repo, pr_head_sha)
    ok = True
    for check_name in NA0237A_REQUIRED_PR_CHECKS:
        run = latest_run_for_name(check_runs, check_name)
        if run is None:
            print(f"ERROR: PR head is missing required check {check_name}", file=sys.stderr)
            ok = False
            continue
        status = run.get("status")
        conclusion = run.get("conclusion")
        print(
            f"PR check {check_name}: status={status} conclusion={conclusion} "
            f"url={run.get('html_url')}"
        )
        accepted = conclusion == "success" or (
            check_name == "CodeQL" and conclusion in ("success", "neutral")
        )
        if status != "completed" or not accepted:
            print(
                f"ERROR: PR head required check {check_name} is not accepted",
                file=sys.stderr,
            )
            ok = False
    return ok


def require_send_commit_repair_content(repo: str, pr_head_sha: str) -> bool:
    text = repo_file_text(
        repo,
        pr_head_sha,
        "qsl/qsl-client/qsc/tests/send_commit.rs",
    )
    needles = (
        "mock_key_source_remains_retired",
        "vault_mock_provider_retired",
        "init_mock_vault",
        "qsc_assert_command",
    )
    missing = [needle for needle in needles if needle not in text]
    print(f"SEND_COMMIT_REPAIR_NEEDLES={','.join(needles)}")
    if missing:
        print(
            f"ERROR: send_commit repair content is missing {','.join(missing)}",
            file=sys.stderr,
        )
        return False
    return True


def validate_na0237a_main_public_safety_remediation_pr(
    repo: str,
    pr_number: int,
    expected_sha: str | None,
    *,
    branch: str,
    check_name: str,
    main_advisories_check: str,
    expected_markers: list[str],
) -> int:
    if pr_number != NA0237A_REPAIR_PR:
        print(
            f"ERROR: main-public-safety remediation is limited to PR {NA0237A_REPAIR_PR}",
            file=sys.stderr,
        )
        return 1
    if not expected_sha:
        print("ERROR: expected remediation PR head SHA is required", file=sys.stderr)
        return 1

    sha = branch_head_sha(repo, branch)
    main_check_runs = commit_check_runs(repo, sha)
    print(f"{branch} sha={sha}")
    if not require_na0237a_queue(repo, branch):
        return 1
    if not require_pr708_unchanged(repo):
        return 1
    if not require_main_failure_fingerprint(
        repo,
        branch,
        main_check_runs,
        check_name=check_name,
        main_advisories_check=main_advisories_check,
        expected_markers=expected_markers,
        require_advisories_success=True,
    ):
        return 1

    pr = pull_request(repo, pr_number)
    pr_head_sha = pr["head"]["sha"]
    print(f"PR {pr_number} state={pr.get('state')} base={pr['base']['ref']} head_sha={pr_head_sha}")
    if pr.get("state") != "open" or pr["base"]["ref"] != "main":
        print(f"ERROR: PR {pr_number} is not an open PR against main", file=sys.stderr)
        return 1
    if pr_head_sha != expected_sha:
        print(
            f"ERROR: PR {pr_number} head {pr_head_sha} does not match expected {expected_sha}",
            file=sys.stderr,
        )
        return 1
    if not require_na0237a_repair_paths(pull_request_files(repo, pr_number)):
        return 1
    if not require_target_pr_checks(repo, pr_head_sha):
        return 1
    if not require_send_commit_repair_content(repo, pr_head_sha):
        return 1

    print(
        f"ALLOW: PR {pr_number} is the bounded NA-0237A send_commit red-main repair "
        f"for the fingerprinted latest-{branch} public-safety failure"
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
        if filename in SELF_REPAIR_BOOTSTRAP_ROOT_PATHS and status == "modified":
            allowed.append(filename)
            continue
        if SELF_REPAIR_BOOTSTRAP_TESTPLAN_RE.fullmatch(filename) and status in (
            "added",
            "modified",
        ):
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
    expected_main_failure_markers: list[str] | None = None,
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
    admission_reason = main_advisories_check
    if (
        main_advisories_status != "completed"
        or main_advisories_conclusion != "failure"
    ):
        markers = expected_main_failure_markers or []
        if not markers or not require_main_failure_fingerprint(
            repo,
            branch,
            main_check_runs,
            check_name=check_name,
            main_advisories_check=main_advisories_check,
            expected_markers=markers,
            require_advisories_success=True,
        ):
            print(
                f"ERROR: latest {branch} is not red because {main_advisories_check} "
                "is failing, and no bounded main-failure fingerprint admitted this "
                "self-repair bootstrap",
                file=sys.stderr,
            )
            return 1
        admission_reason = "bounded main public-safety failure fingerprint"

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
        f"ALLOW: latest {branch} remains red via {admission_reason}, and PR "
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
            expected_main_failure_markers=args.expected_main_failure_marker,
        )

    main_advisories = latest_run_for_name(main_check_runs, args.main_advisories_check)
    main_advisories_is_failure = (
        main_advisories is not None
        and main_advisories.get("status") == "completed"
        and main_advisories.get("conclusion") == "failure"
    )

    if args.allow_advisory_remediation_pr is not None and main_advisories_is_failure:
        return validate_advisory_remediation_pr(args, main_check_runs)

    if args.allow_main_public_safety_remediation_pr is not None:
        return validate_na0237a_main_public_safety_remediation_pr(
            args.repo,
            args.allow_main_public_safety_remediation_pr,
            args.expected_remediation_pr_sha,
            branch=args.branch,
            check_name=args.check_name,
            main_advisories_check=args.main_advisories_check,
            expected_markers=args.expected_main_failure_marker or [],
        )

    if args.allow_advisory_remediation_pr is None:
        print(
            f"ERROR: latest {args.branch} public safety is not green; relevant PRs stay blocked",
            file=sys.stderr,
        )
        return 1

    return validate_advisory_remediation_pr(args, main_check_runs)


def validate_advisory_remediation_pr(
    args: argparse.Namespace,
    main_check_runs: list[dict],
) -> int:
    sha = branch_head_sha(args.repo, args.branch)
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


def validate_self_repair_bootstrap_pr_cmd(args: argparse.Namespace) -> int:
    return validate_self_repair_bootstrap_pr(
        args.repo,
        args.pr,
        args.sha,
        branch=args.branch,
        check_name=args.check_name,
        main_advisories_check=args.main_advisories_check,
        expected_main_failure_markers=args.expected_main_failure_marker,
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
    main_parser.add_argument("--allow-main-public-safety-remediation-pr", type=int)
    main_parser.add_argument("--expected-pr-sha")
    main_parser.add_argument("--expected-remediation-pr-sha")
    main_parser.add_argument("--expected-main-failure-marker", action="append")
    main_parser.add_argument("--main-advisories-check", default="advisories")
    main_parser.add_argument("--pr-advisories-check", default="advisories")
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
    self_repair_parser.add_argument("--expected-main-failure-marker", action="append")
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

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
