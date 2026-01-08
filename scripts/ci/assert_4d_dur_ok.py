#!/usr/bin/env python3
import glob
import json
import os
from pathlib import Path


def newest_report() -> Path:
    cands = [Path(p) for p in glob.glob("artifacts/*/4D/D2_durability.json")]
    if not cands:
        raise SystemExit("[4D-DUR] ASSERT FAIL: no D2_durability.json found under artifacts/*/4D/")
    # Prefer lexicographically newest run_id (timestamped).
    return sorted(cands, key=lambda p: p.as_posix())[-1]


def main() -> int:
    p = newest_report()
    j = json.loads(p.read_text())
    if not j.get("ok"):
        raise SystemExit(f"[4D-DUR] ASSERT FAIL: report.ok is false (run_id={j.get('run_id')})")
    cov = j.get("coverage") or {}
    if cov.get("profile") != "durability":
        raise SystemExit(f"[4D-DUR] ASSERT FAIL: coverage.profile != durability (got {cov.get('profile')})")
    req = cov.get("required_prefixes") or []
    expected = {"IT-DUR-001", "IT-DUR-002", "IT-DUR-003", "IT-DUR-004", "IT-DUR-005"}
    missing = sorted(expected - set(req))
    if missing:
        raise SystemExit(f"[4D-DUR] ASSERT FAIL: coverage.required_prefixes missing: {missing} (got {req})")

    extra = sorted(set(req) - expected)
    if extra:
        raise SystemExit(f"[4D-DUR] ASSERT FAIL: coverage.required_prefixes contains unexpected entries: {extra} (expected {sorted(expected)})")
    # Ensure at least one passing result for each required prefix.
    results = j.get("results") or []
    passed = [r for r in results if r.get("ok") is True]
    for pref in req:
        if not any((r.get("p3_case_id") or "").startswith(pref) or (r.get("case_id") or "").startswith(pref) for r in passed):
            raise SystemExit(f"[4D-DUR] ASSERT FAIL: missing passing case for required prefix: {pref}")

    if len(passed) != len(expected):
        raise SystemExit(f"[4D-DUR] ASSERT FAIL: passed_cases != {len(expected)} (got {len(passed)})")

    print(f"[4D-DUR] OK: run_id={j.get('run_id')} report={p} passed_cases={len(passed)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
