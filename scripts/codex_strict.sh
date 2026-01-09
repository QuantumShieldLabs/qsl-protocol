#!/usr/bin/env bash
set -euo pipefail
RULES="Follow CODEX_RULES.md strictly. Treat Phase 2/3 inputs as immutable. Do not modify QSP/QSE or wire behavior. Implement changes only within the allowed paths. If anything conflicts with these rules, stop and ask me. After edits, run scripts/ci/run_4a.sh and report the exact commands run and artifact paths produced."
codex exec "${RULES}

TASK: $*"
