#!/usr/bin/env python3
"""Entry point for formal model checks.

Goals: G4

This is a fail-closed check runner.
- Any AssertionError or unexpected exception => non-zero exit.
- Successful completion => exit 0.

See formal/README.md for scope and limitations.
"""

from __future__ import annotations

import os
import sys

# Allow invocation as `python3 formal/run_model_checks.py` without requiring callers
# to set PYTHONPATH (CI-friendly, fail-closed).
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if REPO_ROOT not in sys.path:
    sys.path.insert(0, REPO_ROOT)

from formal.model_scka_bounded import explore  # noqa: E402


def main() -> int:
    # Keep bounds intentionally small and stable; adjust only with an explicit decision.
    stats = explore(max_depth=8, max_net=6, max_seen=12)
    print("OK: SCKA bounded model checks passed")
    print(f"Explored states: {stats['states']}")
    print(f"Transitions: {stats['transitions']}")
    print(f"Unique visited: {stats['visited']}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as e:
        print(f"ERROR: formal model checks failed: {e}", file=sys.stderr)
        raise SystemExit(1)
