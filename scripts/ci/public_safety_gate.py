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
) -> int:
    sha = branch_head_sha(repo, branch)
    main_check_runs = commit_check_runs(repo, sha)
    run = latest_run_for_name(main_check_runs, check_name)
    if run is None:
        print(
            f"ERROR: latest {branch} commit {sha} is missing check '{check_name}'",
            file=sys.stderr,
        )
        return 1
    status = run.get("status")
    conclusion = run.get("conclusion")
    print(f"{branch} sha={sha} check={check_name} status={status} conclusion={conclusion}")
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
    run = latest_run_for_name(commit_check_runs(args.repo, sha), args.check_name)
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
    print(
        f"ERROR: latest {args.branch} public safety is not green; relevant PRs stay blocked",
        file=sys.stderr,
    )
    return 1


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
    main_parser.add_argument("--expected-pr-sha")
    main_parser.add_argument("--main-advisories-check", default="advisories")
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
    self_repair_parser.set_defaults(func=validate_self_repair_bootstrap_pr_cmd)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
