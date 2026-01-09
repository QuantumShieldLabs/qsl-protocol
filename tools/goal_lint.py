#!/usr/bin/env python3
import json, os, re, subprocess, sys
from pathlib import Path

# ---- Configuration (tune paths to your repo) ----
CORE_PATH_PATTERNS = [
    r"(^|/)spec(/|$)",
    r"(^|/)docs(/|$)",
    r"(^|/)src(/|$)",
    r"(^|/)code(/|$)",
    r"(^|/)protocol(/|$)",
    r"(^|/)envelope(/|$)",
]
TEST_PATH_PATTERNS = [
    r"(^|/)tests(/|$)",
    r"(^|/)vectors(/|$)",
    r"(^|/)harness(/|$)",
]
DOCS_ONLY_PATH_PATTERNS = [
    r"^README\.md$",
    r"^docs/INDEX\.md$",
    r"^docs/CHECKLIST_.*\.md$",
    r"^docs/.*",
    r"^\.github/ISSUE_TEMPLATE/.*",
    r"^\.github/PULL_REQUEST_TEMPLATE\.md$",
]
REQUIRED_DOCS_FOR_CORE_CHANGES = [
    "TRACEABILITY.md",
    "DECISIONS.md",
]

GOALS_RE = re.compile(r"\bGoals:\s*(G[1-5](\s*,\s*G[1-5])*)\b", re.IGNORECASE)

def run(cmd):
    return subprocess.check_output(cmd, text=True).strip()

def git_changed_files(base_sha, head_sha):
    out = run(["git", "diff", "--name-only", f"{base_sha}...{head_sha}"])
    return [line.strip() for line in out.splitlines() if line.strip()]

def any_match(patterns, path):
    return any(re.search(p, path) for p in patterns)

def main():
    event_path = os.environ.get("GITHUB_EVENT_PATH")
    if not event_path or not Path(event_path).exists():
        print("ERROR: GITHUB_EVENT_PATH missing; cannot lint PR metadata.")
        return 1

    event = json.loads(Path(event_path).read_text(encoding="utf-8"))
    pr = event.get("pull_request") or {}
    body = (pr.get("body") or "").strip()

    # 1) Require Goals declaration
    if not GOALS_RE.search(body):
        print("ERROR: PR body must include a line like: 'Goals: G1, G2'.")
        print("Hint: use .github/pull_request_template.md")
        return 1

    # 2) Determine changed files
    base = pr.get("base", {}).get("sha")
    head = pr.get("head", {}).get("sha")
    if not base or not head:
        print("ERROR: cannot determine base/head SHAs from event payload.")
        return 1

    changed = git_changed_files(base, head)
    if not changed:
        print("OK: no file changes detected.")
        return 0

    core_touched = [p for p in changed if any_match(CORE_PATH_PATTERNS, p)]
    tests_touched = [p for p in changed if any_match(TEST_PATH_PATTERNS, p)]
    docs_only = all(any_match(DOCS_ONLY_PATH_PATTERNS, p) for p in changed)

    # 3) If core protocol surfaces changed, require traceability/decisions and tests/vectors
    if core_touched:
        missing_docs = [d for d in REQUIRED_DOCS_FOR_CORE_CHANGES if d not in changed]
        if missing_docs:
            print("ERROR: Core protocol paths changed, but required governance docs were not updated:")
            for d in missing_docs:
                print(f"  - {d}")
            print("Update TRACEABILITY.md and/or add a DECISIONS.md entry as appropriate.")
            return 1

        if not tests_touched and not docs_only:
            print("ERROR: Core protocol paths changed, but no tests/vectors were modified.")
            print("Add or update at least one file under tests/, vectors/, or harness/.")
            return 1

    print("OK: goal compliance checks passed.")
    if core_touched:
        print("Core changes:")
        for p in core_touched:
            print(f"  - {p}")
    return 0

if __name__ == "__main__":
    sys.exit(main())
