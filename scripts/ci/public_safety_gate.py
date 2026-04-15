#!/usr/bin/env python3
import argparse
import json
import os
import sys
import time
import urllib.error
import urllib.parse
import urllib.request


def github_api_base() -> str:
    return os.environ.get("GITHUB_API_URL", "https://api.github.com").rstrip("/")


def github_token() -> str:
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if not token:
        raise SystemExit("ERROR: GITHUB_TOKEN or GH_TOKEN is required")
    return token


def github_get(path: str, query: dict[str, str] | None = None) -> dict:
    url = f"{github_api_base()}{path}"
    if query:
        url = f"{url}?{urllib.parse.urlencode(query)}"
    req = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {github_token()}",
            "User-Agent": "qsl-public-safety-gate",
        },
    )
    try:
        with urllib.request.urlopen(req) as resp:
            return json.load(resp)
    except urllib.error.HTTPError as exc:
        body = exc.read().decode("utf-8", errors="replace")
        raise SystemExit(f"ERROR: GitHub API {exc.code} for {url}\n{body}") from exc


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
    if status != "completed" or conclusion != "success":
        print(
            f"ERROR: latest {args.branch} public safety is not green; relevant PRs stay blocked",
            file=sys.stderr,
        )
        return 1
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

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
