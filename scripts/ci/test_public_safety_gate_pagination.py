#!/usr/bin/env python3
"""Self-test: public_safety_gate.py reads EVERY page of a commit's check runs.

The defect this file guards: commit_check_runs() and commit_check_runs_for_wait() fetched ONE
page of 100 check runs. main's head f7542fab collected 161 (scheduled runs keep landing on an
unmoving head), public-safety sat on page 2, and check-main-public-safety reported main
"missing check 'public-safety'" -- blocking every relevant PR and push run.

No network: the module's own fetch seams (github_get, github_get_for_wait) are replaced by a fake
that serves synthetic pages by the "page" query key (an absent key is page 1, as GitHub does).

Run: python3 scripts/ci/test_public_safety_gate_pagination.py
Exit 0 = all checks passed. Any failure prints what was expected and what happened.
"""

from __future__ import annotations

import contextlib
import importlib.util
import io
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location(
    "public_safety_gate", os.path.join(HERE, "public_safety_gate.py")
)
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)

REPO = "QuantumShieldLabs/qsl-protocol"
SHA = "f" * 40
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


def run(name: str, status: str = "completed", conclusion: str | None = "success", *, run_id: int) -> dict:
    return {"id": run_id, "name": name, "status": status, "conclusion": conclusion}


def filler(count: int, start_id: int) -> list[dict]:
    return [run("main-red-sentinel", conclusion="skipped", run_id=start_id + i) for i in range(count)]


class FakeGitHub:
    """Serves /commits/{sha}/check-runs by page and /branches/{b} for branch_head_sha."""

    def __init__(self, pages: list[list[dict]] | None, total_count: int | None, *, endless: bool = False):
        self.pages = pages or []
        self.total_count = total_count
        self.endless = endless
        self.requested: list[int] = []

    def __call__(self, path: str, query: dict[str, str] | None = None) -> dict:
        query = query or {}
        if path == f"/repos/{REPO}/branches/main":
            return {"commit": {"sha": SHA}}
        if path != f"/repos/{REPO}/commits/{SHA}/check-runs":
            raise AssertionError(f"unexpected GitHub path {path}")
        page = int(query.get("page", "1"))
        self.requested.append(page)
        if self.endless:
            batch = filler(int(query.get("per_page", "30")), start_id=page * 1000)
        else:
            batch = self.pages[page - 1] if page <= len(self.pages) else []
        data: dict = {"check_runs": batch}
        if self.total_count is not None:
            data["total_count"] = self.total_count
        return data


@contextlib.contextmanager
def seam(fake: FakeGitHub):
    saved = (gate.github_get, gate.github_get_for_wait)
    gate.github_get = fake
    gate.github_get_for_wait = fake
    try:
        yield
    finally:
        gate.github_get, gate.github_get_for_wait = saved


def main_161_pages() -> list[list[dict]]:
    # 161 runs: page 1 is 100 fillers; page 2 carries public-safety, advisories and the two
    # push suites among 57 more fillers.
    page2 = [
        run("public-safety", run_id=5001),
        run("advisories", run_id=5002),
        run("qsc-sharded-suite", run_id=5003),
        run("macos-qsc-sharded-suite", run_id=5004),
    ] + filler(57, start_id=6000)
    return [filler(100, start_id=1), page2]


def call(fn, *args, **kwargs):
    out, err = io.StringIO(), io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
        try:
            result = fn(*args, **kwargs)
            exc = None
        except SystemExit as caught:
            result, exc = None, caught
    return result, exc, out.getvalue(), err.getvalue()


def t1_check_main_161_two_pages() -> None:
    fake = FakeGitHub(main_161_pages(), 161)
    args = gate.build_parser().parse_args(["check-main-public-safety", "--repo", REPO])
    with seam(fake):
        rc, exc, out, err = call(gate.check_main_public_safety, args)
    want = f"main sha={SHA} check=public-safety status=completed conclusion=success"
    check(
        "T1 check_main_161_two_pages: public-safety on page 2 is found, rc=0",
        exc is None and rc == 0 and want in out,
        f"rc={rc} exc={exc}\nstdout={out}\nstderr={err}",
    )


def t2_commit_check_runs_161() -> None:
    for label, fn in (
        ("commit_check_runs", gate.commit_check_runs),
        ("commit_check_runs_for_wait", gate.commit_check_runs_for_wait),
    ):
        fake = FakeGitHub(main_161_pages(), 161)
        with seam(fake):
            runs, exc, out, err = call(fn, REPO, SHA)
        names = {r["name"] for r in runs or []}
        check(
            f"T2 {label}_161: all 161 runs over pages [1, 2]",
            exc is None and len(runs or []) == 161 and "public-safety" in names
            and fake.requested == [1, 2],
            f"len={len(runs or [])} pages={fake.requested} exc={exc}\nstderr={err}",
        )


def t3_wait_161_two_pages() -> None:
    fake = FakeGitHub(main_161_pages(), 161)
    with seam(fake):
        rc, exc, out, err = call(
            gate.wait_for_required_checks,
            repo=REPO,
            sha=SHA,
            required=["qsc-sharded-suite", "macos-qsc-sharded-suite"],
            interval_seconds=0,
            max_iterations=1,
            sleeper=lambda _seconds: None,
        )
    check(
        "T3 wait_161_two_pages: required checks on page 2 settle green, rc=0",
        exc is None and rc == 0 and f"OK: required checks green on {SHA}" in out,
        f"rc={rc} exc={exc}\nstdout={out}\nstderr={err}",
    )


def t4_exactly_100_then_empty() -> None:
    for total in (100, None):
        for label, fn in (
            ("commit_check_runs", gate.commit_check_runs),
            ("commit_check_runs_for_wait", gate.commit_check_runs_for_wait),
        ):
            fake = FakeGitHub([filler(100, start_id=1), []], total)
            with seam(fake):
                runs, exc, out, err = call(fn, REPO, SHA)
            check(
                f"T4 {label} exactly 100 then an empty page (total_count={total}): 100, no error",
                exc is None and len(runs or []) == 100 and fake.requested[:1] == [1]
                and len(fake.requested) <= 2,
                f"len={len(runs or [])} pages={fake.requested} exc={exc}\nstderr={err}",
            )


def t5_page_cap_exceeded() -> None:
    cap = getattr(gate, "COMMIT_CHECK_RUNS_MAX_PAGES", None)
    for label, fn in (
        ("commit_check_runs", gate.commit_check_runs),
        ("commit_check_runs_for_wait", gate.commit_check_runs_for_wait),
    ):
        fake = FakeGitHub(None, 10**6, endless=True)
        with seam(fake):
            runs, exc, out, err = call(fn, REPO, SHA)
        message = str(exc.code) if exc is not None else ""
        check(
            f"T5 {label} page cap exceeded: fails loud, never a truncated list",
            exc is not None and runs is None and "page cap" in message
            and cap is not None and len(fake.requested) == cap,
            f"cap={cap} pages_requested={len(fake.requested)} returned={len(runs or [])} "
            f"exc={message!r}",
        )


def main() -> int:
    print("public_safety_gate.py check-run pagination self-test")
    t1_check_main_161_two_pages()
    t2_commit_check_runs_161()
    t3_wait_161_two_pages()
    t4_exactly_100_then_empty()
    t5_page_cap_exceeded()
    if FAILURES:
        print(f"FAILED: {len(FAILURES)} of {CHECKS} checks")
        return 1
    print(f"OK: {CHECKS} of {CHECKS} checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
