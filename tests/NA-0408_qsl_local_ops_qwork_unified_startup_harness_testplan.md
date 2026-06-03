Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0408 qwork Unified Startup Harness Testplan

## Purpose

Validate the local qwork startup harness and the qsl-protocol governance
evidence that records it. This testplan proves qwork starts safe clean
checkouts deterministically, fails closed on unsafe states, preserves qstart and
qresume compatibility, and leaves runtime, dependency, workflow, backup, public,
and sibling-repo surfaces unchanged.

## Required Local Tool Checks

Run from the qsl-protocol repo root unless a command has an absolute path.

```bash
bash -n /srv/qbuild/tools/qwork.sh
bash -n /srv/qbuild/tools/qshell.sh
rg -n "reset --hard|git reset|git stash|git clean|clean -f|rm -rf|rm -f|force-push|git push|push |checkout -f|merge --no-ff" /srv/qbuild/tools/qwork.sh /srv/qbuild/tools/qshell.sh || true
/srv/qbuild/tmp/NA0408_qwork_startup_harness_20260603T112854-0500/qwork_test_harness.sh
```

Expected results:

- Bash syntax checks exit zero.
- Forbidden-command scan reports zero matches.
- Harness output ends with `QWORK_TEST_HARNESS_OK`.

## Harness Coverage

The harness must cover at minimum:

- existing clean current checkout passes.
- existing clean stale main fast-forwards.
- dirty tracked file fails with `reason=dirty-worktree`.
- dirty index fails with `reason=dirty-index`.
- untracked file fails with `reason=untracked-files`.
- local ahead fails with `reason=local-ahead`.
- non-main branch fails with `reason=non-main-branch`.
- qsl-protocol current READY lane passes with `ready_count=1`.
- qsl-protocol wrong requested lane fails with `reason=queue-lane-mismatch`.
- JSON proof is written under the lane log directory.

## Live Smoke

```bash
source /srv/qbuild/tools/qshell.sh
qwork NA-0408 qsl-protocol
git status --porcelain=v1 --branch
test -f /srv/qbuild/logs/NA-0408/startup.qsl-protocol.json
```

Expected qwork proof includes:

- `startup_result=OK`
- `lane=NA-0408`
- `repo=qsl-protocol`
- `branch=main`
- `upstream=origin/main`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0408`
- `requested_lane_status=READY`

## Compatibility Smoke

```bash
source /srv/qbuild/tools/qshell.sh
qstart NA-0408 qsl-protocol
qresume NA-0408 qsl-protocol
```

Expected results:

- Both commands exit zero on clean `main`.
- Both report already-current qsl-protocol state.
- Neither command breaks qwork or mutates dirty state.

## Governance Checks

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 - <<'PY'
from pathlib import Path
for token in ["D-0799", "D-0800", "D-0801"]:
    print(token, Path("DECISIONS.md").read_text().count(f"- **ID:** {token}"))
PY
git diff --name-only origin/main...HEAD
git diff --check
```

Expected results:

- `READY_COUNT 1`.
- `READY NA-0408 QSL Local Ops qwork Unified Startup Command Implementation Harness`.
- `D-0799 1`.
- `D-0800 1`.
- `D-0801 0`.
- Changed paths are limited to the Packet C allowed qsl-protocol evidence paths.
- `git diff --check` exits zero.

## Link, Leak, and Overclaim Checks

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main --paths \
  docs/governance/evidence/NA-0408_qsl_local_ops_qwork_unified_startup_harness.md \
  tests/NA-0408_qsl_local_ops_qwork_unified_startup_harness_testplan.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body-file> --scan-overclaims
```

Expected results:

- `TOTAL_MISSING 0`.
- `SECRET_FINDING_COUNT 0`.
- PR-body preflight reports no missing required fields and no prohibited phrases.

## Dependency, Formal, and Client Checks

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
```

Expected results:

- `cargo audit --deny warnings` exits zero.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13`
  or a newer safe version.
- Formatting, qsc send_commit, formal model checks, and feasible qshield-cli
  build/test commands pass.

## Acceptance

The implementation evidence passes only if qwork proves deterministic clean
startup, all unsafe states fail closed without mutation, qstart/qresume remain
compatible, D-0800 exists exactly once, D-0801 remains absent, and no forbidden
runtime, dependency, workflow, backup, public, qsl-server, or qsl-attachments
surface changes.
