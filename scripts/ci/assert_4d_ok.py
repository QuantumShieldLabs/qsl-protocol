#!/usr/bin/env python3
"""Fail-closed assertion for Phase 4D (security) gates.

This is intentionally conservative:
- requires the authoritative 4D interop report to exist
- requires report.ok == true and no top-level errors
- requires coverage.profile == "security"
- requires at least one passing result for every required_prefix

Usage (repo root):
  python3 scripts/ci/assert_4d_ok.py
"""

from __future__ import annotations

import json
import os
import sys
from typing import Any, Dict, List, Optional, Tuple


def _find_latest_report() -> Tuple[str, str]:
    """Return (run_id, path) for the latest D1_interop_security.json."""
    root = os.path.abspath(os.getcwd())
    arts = os.path.join(root, "artifacts")
    if not os.path.isdir(arts):
        raise FileNotFoundError("artifacts/ directory not found")

    candidates: List[Tuple[float, str, str]] = []
    # artifacts/<RUN_ID>/4D/D1_interop_security.json
    for run_id in os.listdir(arts):
        p = os.path.join(arts, run_id, "4D", "D1_interop_security.json")
        if os.path.isfile(p):
            try:
                mtime = os.path.getmtime(p)
            except OSError:
                mtime = 0.0
            candidates.append((mtime, run_id, p))

    if not candidates:
        raise FileNotFoundError("No Phase 4D report found under artifacts/*/4D/D1_interop_security.json")

    candidates.sort(key=lambda t: (t[0], t[1]))
    _, run_id, path = candidates[-1]
    return run_id, path


def _load_json(path: str) -> Dict[str, Any]:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def _fail(msg: str) -> int:
    print(f"[4D] ASSERT FAIL: {msg}", file=sys.stderr)
    return 1


def main() -> int:
    try:
        run_id, path = _find_latest_report()
    except Exception as e:
        return _fail(str(e))

    try:
        report = _load_json(path)
    except Exception as e:
        return _fail(f"Failed to parse JSON: {path}: {e}")

    ok = report.get("ok") is True
    errors = report.get("errors") or []
    coverage = report.get("coverage") or {}

    if not ok:
        return _fail(f"report.ok is false (run_id={run_id})")

    if errors:
        return _fail(f"report has errors (run_id={run_id}): {errors[:3]}")

    if coverage.get("profile") != "security":
        return _fail(f"unexpected coverage.profile (expected security, got {coverage.get('profile')!r})")

    required: List[str] = list(coverage.get("required_prefixes") or [])
    if not required:
        return _fail("coverage.required_prefixes missing or empty")

    results = report.get("results") or []
    passed = {r.get("p3_case_id") for r in results if r.get("ok") is True and isinstance(r.get("p3_case_id"), str)}

    missing = [rid for rid in required if rid not in passed]
    if missing:
        return _fail(f"missing passing results for required cases: {missing}")

    print(f"[4D] OK: run_id={run_id} report={path} passed_cases={len(passed)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
