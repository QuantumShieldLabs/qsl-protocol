#!/usr/bin/env python3
"""NA-0629 (WF-0018) — doc-staleness lint.

A lightweight, NON-REQUIRED nudge (rides the lane gate + local runs, like the NA-0628 anti-regression
scan — it is not wired into branch protection). It flags any "living" strategic doc whose
`Last-Updated:` trails the newest DECISIONS.md decision date by more than THRESHOLD_DAYS, so the
strategic narrative cannot silently drift 6–18 weeks behind live truth again (the WF-0018 finding).

It deliberately EXCLUDES:
  - constitutional docs that are stable by design (GOALS.md, PROJECT_CHARTER.md);
  - deprecated compatibility stubs (they carry no Last-Updated and point at live truth);
  - historical/superseded audit artifacts (they carry a Superseded-By header).

Usage:
  python3 tests/NA-0629_doc_staleness_lint.py                 # lint the living set
  python3 tests/NA-0629_doc_staleness_lint.py --self-test     # prove it FAILS on a synthetic stale doc
"""
from __future__ import annotations

import datetime
import re
import sys
from pathlib import Path

THRESHOLD_DAYS = 45

# Living strategic docs that SHOULD track main. Add here when a new living strategic doc is created.
LIVING_DOCS = [
    "ROADMAP.md",
    "FORMAL_VERIFICATION_PLAN.md",
    "docs/program/DOC-PROG-001_Goal_to_Release_Roadmap_v0.1.0_DRAFT.md",
    "docs/public/INDEX.md",
    "docs/public/PROGRESS.md",
    "docs/public/EXTERNAL_REVIEW_PACKAGE.md",
    "docs/public/RELEASE_READINESS_EVIDENCE_MAP.md",
]

DATE_RE = re.compile(r"^\s*Last-?Updated:\s*.*?(\d{4}-\d{2}-\d{2})", re.IGNORECASE | re.MULTILINE)
DECISION_DATE_RE = re.compile(r"^\s*- \*\*Date:\*\*\s*(\d{4}-\d{2}-\d{2})", re.MULTILINE)


def parse_date(s: str) -> datetime.date:
    return datetime.date.fromisoformat(s)


def doc_last_updated(path: Path) -> datetime.date | None:
    if not path.exists():
        return None
    m = DATE_RE.search(path.read_text(encoding="utf-8"))
    return parse_date(m.group(1)) if m else None


def newest_decision_date(root: Path) -> datetime.date:
    text = (root / "DECISIONS.md").read_text(encoding="utf-8")
    dates = [parse_date(d) for d in DECISION_DATE_RE.findall(text)]
    if not dates:
        raise SystemExit("FATAL: no decision dates found in DECISIONS.md")
    return max(dates)


def lint(root: Path, extra_docs: list[str] | None = None) -> list[str]:
    reference = newest_decision_date(root)
    stale: list[str] = []
    for rel in LIVING_DOCS + (extra_docs or []):
        d = doc_last_updated(root / rel)
        if d is None:
            stale.append(f"{rel}: no `Last-Updated:` date found (living strategic doc must carry one)")
            continue
        lag = (reference - d).days
        if lag > THRESHOLD_DAYS:
            stale.append(
                f"{rel}: Last-Updated {d} trails the newest decision ({reference}) by {lag} days "
                f"(> {THRESHOLD_DAYS}). Refresh it or move it out of the living set."
            )
    return stale


def main() -> int:
    root = Path(__file__).resolve().parents[1]

    if "--self-test" in sys.argv:
        # Prove the lint CAN fail: a synthetic doc dated far in the past MUST be flagged.
        synthetic = root / "tests" / "_na0629_synthetic_stale.md"
        synthetic.write_text("Last-Updated: 2000-01-01\n", encoding="utf-8")
        try:
            hits = lint(root, extra_docs=["tests/_na0629_synthetic_stale.md"])
        finally:
            synthetic.unlink(missing_ok=True)
        flagged = any("_na0629_synthetic_stale" in h for h in hits)
        print("SELF-TEST:", "PASS (synthetic stale doc flagged)" if flagged else "FAIL (not flagged!)")
        return 0 if flagged else 2

    stale = lint(root)
    if stale:
        print("DOC-STALENESS LINT — living strategic docs trailing live truth:", file=sys.stderr)
        for s in stale:
            print(f"  - {s}", file=sys.stderr)
        return 1
    print(f"DOC-STALENESS LINT OK: all {len(LIVING_DOCS)} living strategic docs are current "
          f"(within {THRESHOLD_DAYS} days of the newest decision).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
