#!/usr/bin/env python3
"""Self-test for the operator-infrastructure literal gate.

NA-0686 / D-1325 (ENG-0089). The gate had NO tests. That is the defect this file
closes, and it is the same defect the gate itself exists to catch one level down:
an instrument nobody instruments is believed rather than checked.

⚠ THE FINDING THAT MADE THIS URGENT (NA-0685). Running the scan in `--mode diff`
over an empty input made it refuse a vacuous pass -- "NOTHING EXAMINED", exit 2 --
which is correct, valuable, and was **completely unguarded**. Nothing proved the
scan refuses an empty input rather than reporting clean. A gate that silently
becomes a no-op when its base ref is not fetched is worse than no gate, because
CI goes green and everyone believes it.

⚠ WHY THIS FILE CARRIES NO OPERATOR LITERAL. It runs in the tree the Tier-1 scan
examines, so a test written the obvious way -- paste the literal, assert the scan
finds it -- would make the gate fail on its own test file. Every needle here is
therefore either ASSEMBLED AT RUN TIME from fragments that never appear
contiguously in this source, or synthesised against a made-up token whose digest
is computed on the fly. The machinery is tested; the names are not republished.

Run: python3 scripts/ci/infra_literal_scan_selftest.py
Exit 0 = all checks passed. Any failure prints what was expected and what happened.
"""

from __future__ import annotations

import hashlib
import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
SCANNER = os.path.join(HERE, "infra_literal_scan.py")
SALT = b"qsl-infra-literal-scan-v1|"

FAILURES: list[str] = []
CHECKS = 0


def check(name: str, ok: bool, detail: str = "") -> None:
    global CHECKS
    CHECKS += 1
    if ok:
        print(f"  ok    {name}")
    else:
        print(f"  FAIL  {name}")
        if detail:
            for line in detail.strip().splitlines():
                print(f"          {line}")
        FAILURES.append(name)


def git(repo: str, *args: str) -> None:
    subprocess.run(["git", "-C", repo, *args], check=True, capture_output=True)


def new_repo(tmp: str) -> str:
    repo = tempfile.mkdtemp(dir=tmp)
    git(repo, "init", "-q")
    git(repo, "config", "user.email", "selftest@example.invalid")
    git(repo, "config", "user.name", "selftest")
    git(repo, "config", "remote.origin.url", "https://example.invalid/qsl-selftest.git")
    return repo


def run_scan(repo: str, *args: str) -> tuple[int, str]:
    proc = subprocess.run(
        [sys.executable, SCANNER, *args],
        cwd=repo,
        capture_output=True,
        text=True,
    )
    return proc.returncode, proc.stdout + proc.stderr


def write(repo: str, name: str, body: str, stage: bool = True) -> None:
    with open(os.path.join(repo, name), "w", encoding="utf-8") as fh:
        fh.write(body)
    if stage:
        git(repo, "add", name)


# ---------------------------------------------------------------------------
# THE NEEDLES, assembled so this file never contains them contiguously.
# ---------------------------------------------------------------------------

def cgnat_line() -> str:
    """A CGNAT address in 100.64/10. Never contiguous in this source."""
    return "export RELAY_URL=https://" + "100." + "99." + "234.5" + ":8443/v1/inbox"


def ddns_line() -> str:
    """A host under the retired public dynamic-DNS domain."""
    return "curl https://probe-host." + "ddns" + "free" + ".com/v1/health"


def main() -> int:
    print("infra-literal-scan selftest")
    with tempfile.TemporaryDirectory() as tmp:

        # -------------------------------------------------------------------
        print("\n[1] the vacuous-pass refusal (NA-0685's unguarded behaviour)")
        # -------------------------------------------------------------------
        empty = new_repo(tmp)
        code, out = run_scan(empty, "--mode", "tree")
        check(
            "tree mode over an EMPTY tree refuses to report a pass",
            code == 2 and "NOTHING EXAMINED" in out,
            f"exit={code}\n{out}",
        )
        check(
            "...and does not print the word 'clean'",
            "clean" not in out,
            out,
        )

        # An unfetched base ref is how a diff-mode gate silently no-ops in CI.
        write(empty, "seed.md", "nothing interesting\n")
        git(empty, "commit", "-q", "-m", "seed")
        code, out = run_scan(empty, "--mode", "diff", "--base", "HEAD")
        check(
            "diff mode with ZERO added lines refuses to report a pass",
            code == 2 and "NOTHING EXAMINED" in out,
            f"exit={code}\n{out}",
        )

        # ...but staged mode must NOT refuse: a deletion-only or rename-only
        # commit legitimately has no added lines, and blocking those would make
        # the pre-commit hook unusable. This asymmetry is deliberate and is
        # exactly the kind of detail that gets "simplified" away later.
        code, out = run_scan(empty, "--mode", "staged")
        check(
            "staged mode with zero added lines PASSES (deletion-only commits)",
            code == 0 and "clean" in out,
            f"exit={code}\n{out}",
        )

        # -------------------------------------------------------------------
        print("\n[2] the scan reports WHAT IT EXAMINED, not merely a verdict")
        # -------------------------------------------------------------------
        code, out = run_scan(empty, "--mode", "tree")
        check(
            "tree mode prints file and line counts",
            code == 0 and "files," in out and "lines examined" in out,
            f"exit={code}\n{out}",
        )

        # -------------------------------------------------------------------
        print("\n[3] Tier-2b structural classes fail closed on an ADDED line")
        # -------------------------------------------------------------------
        for label, needle, cls in (
            ("CGNAT 100.64/10", cgnat_line(), "tailnet_cgnat"),
            ("public dynamic-DNS host", ddns_line(), "public_ddns_host"),
        ):
            repo = new_repo(tmp)
            write(repo, "seed.md", "seed\n")
            git(repo, "commit", "-q", "-m", "seed")
            write(repo, "runbook.md", needle + "\n")
            code, out = run_scan(repo, "--mode", "staged")
            check(
                f"{label}: added line is REFUSED and the class is NAMED",
                code == 1 and cls in out,
                f"exit={code}\n{out}",
            )
            # ...and green once the line is gone. A control that only ever goes
            # red proves the scan is broken, not that it is discriminating.
            git(repo, "rm", "-q", "--cached", "runbook.md")
            os.remove(os.path.join(repo, "runbook.md"))
            write(repo, "runbook.md", "no literal here\n")
            code, out = run_scan(repo, "--mode", "staged")
            check(
                f"{label}: GREEN again once the literal is removed",
                code == 0 and cls not in out,
                f"exit={code}\n{out}",
            )

        # -------------------------------------------------------------------
        print("\n[4] the Tier-1 per-path allowlist is a BUDGET, not an exemption")
        # -------------------------------------------------------------------
        # Synthesised against a token that exists nowhere but this test, so the
        # budget logic is exercised without naming any real host.
        token = "zqxwvtok"  # 8 chars: inside the Tier-1 matcher's length filter
        tok_digest = hashlib.sha256(SALT + token.encode()).hexdigest()
        repo = new_repo(tmp)
        write(repo, "seed.md", "seed\n")
        git(repo, "commit", "-q", "-m", "seed")

        harness = f'''
import subprocess, sys, importlib.util, hashlib
spec = importlib.util.spec_from_file_location("s", {SCANNER!r})
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
m.TIER1_NAME_DIGESTS[{tok_digest!r}] = "synthetic_probe"
m.TIER1_ALLOWLISTED_CLASSES.add("synthetic_probe")
m.TIER1_MATCHER = m._NameMatcher(m.TIER1_NAME_DIGESTS, {{7, 8, 9, 10}})
budget = int(sys.argv[1])
if budget:
    m.TIER1_TREE_ALLOWLIST[m._digest("qsl-selftest:probe.md")] = budget
sys.argv = ["scan", "--mode", "tree"]
sys.exit(m.main())
'''
        harness_path = os.path.join(repo, "_harness.py")
        with open(harness_path, "w", encoding="utf-8") as fh:
            fh.write(harness)

        # Two occurrences on one line: the budget counts OCCURRENCES, so a lane
        # cannot double up on an existing line and stay under a per-line count.
        write(repo, "probe.md", f"host {token} and {token} again\n")

        def run_harness(budget: int) -> tuple[int, str]:
            proc = subprocess.run(
                [sys.executable, "_harness.py", str(budget)],
                cwd=repo,
                capture_output=True,
                text=True,
            )
            return proc.returncode, proc.stdout + proc.stderr

        code, out = run_harness(0)
        check(
            "no baseline: the class FAILS the tree scan",
            code == 1 and "synthetic_probe" in out,
            f"exit={code}\n{out}",
        )

        code, out = run_harness(2)
        check(
            "baseline == actual (2): PASSES, and prints the exception it met",
            code == 0 and "allowlisted history met" in out and "probe.md" in out,
            f"exit={code}\n{out}",
        )

        code, out = run_harness(1)
        check(
            "baseline 1 < actual 2: FAILS and names the file as OVER BASELINE",
            code == 1 and "OVER BASELINE" in out and "probe.md" in out,
            f"exit={code}\n{out}",
        )

        code, out = run_harness(5)
        check(
            "baseline 5 > actual 2: PASSES and reports the DECREASE",
            code == 0 and "DECREASED" in out,
            f"exit={code}\n{out}",
        )

    print(f"\n{CHECKS} checks, {len(FAILURES)} failed")
    if FAILURES:
        print("FAILED: " + ", ".join(FAILURES))
        return 1
    # A selftest that examined nothing is the very thing it exists to forbid.
    if CHECKS == 0:
        print("NOTHING EXAMINED -- refusing to report a pass over zero checks")
        return 2
    print("infra-literal-scan selftest: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
