Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-08

# NA-0254 Closeout Restore NA-0255 Test Plan

## Objective

Validate that NA-0254 closes only after Packet B merged and post-merge public-safety completed green, then restore exactly one successor: `NA-0255 — External Website Evidence-Boundary Implementation Execution`.

## Protected Invariants

- `public-safety` remains a required, green protected check.
- NA-0254 timeout-resilience hardening remains fail-closed for real watched-suite failures.
- Exactly one READY item exists after closeout.
- NA-0255 does not authorize qsl-protocol implementation changes.
- NA-0255 must verify the exact external website repository before mutation.
- External website copy must remain evidence-bound and avoid production-readiness, proven true Triple Ratchet, anonymity, and metadata-elimination overclaims.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0254_closeout_restore_na0255_testplan.md`

Forbidden path classes include `.github/**`, `scripts/**`, Cargo manifests/locks, qsc/qsl app/runtime/protocol/crypto/demo/service implementation paths, qsl-server, qsl-attachments, qsc-desktop, website, apps, tools, inputs, formal, branch-protection settings, and public-safety/check configuration.

Expected guard command:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed "NEXT_ACTIONS.md" --allowed "DECISIONS.md" --allowed "TRACEABILITY.md" --allowed "docs/ops/ROLLING_OPERATIONS_JOURNAL.md" --allowed "tests/NA-0254_closeout_restore_na0255_testplan.md"
```

## Pre-Closeout Proof

Required preconditions:

- PR #759 is merged.
- PR #759 head is `e95128a36736`.
- PR #759 merge commit is `1361e10b8a06`.
- Post-merge main `public-safety` completed success.
- Post-merge main `qsc-linux-full-suite`, `macos-qsc-full-serial`, and `qsc-adversarial-smoke` completed success.
- `D-0476` exists once.
- `D-0477` is absent before closeout edits.
- READY_COUNT is `1` with READY `NA-0254`.

## Queue Parser Expectation

After closeout:

```text
READY_COUNT 1
READY NA-0255 External Website Evidence-Boundary Implementation Execution
NA-0254 DONE Public-Safety Timeout-Resilient Push-Suite Polling Hardening
```

## Decision Parser Expectation

After closeout:

- `D-0476` exists once.
- `D-0477` exists once.
- no duplicate decision IDs exist.

## NA-0255 Boundary Proof

The restored NA-0255 entry must state:

- external website repository only.
- exact repo must be verified before mutation.
- qsl-protocol may be used only as read-only evidence source.
- no qsl-protocol implementation changes unless later explicitly authorized.
- no qsl-protocol branch-protection or public-safety changes.
- website copy remains evidence-bound.
- rollback/screenshot/build preview evidence is required.

## Validation Commands

```bash
git diff --check origin/main...HEAD
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed "NEXT_ACTIONS.md" --allowed "DECISIONS.md" --allowed "TRACEABILITY.md" --allowed "docs/ops/ROLLING_OPERATIONS_JOURNAL.md" --allowed "tests/NA-0254_closeout_restore_na0255_testplan.md"
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Run local goal-lint with a synthetic PR event whose body includes `Goals: G1, G3, G5`.

## CI Expectations

- Required PR checks pass normally.
- CodeQL may be accepted as neutral only if GitHub accepts the aggregate required context.
- `public-safety` succeeds on the PR head and post-merge main.
- Merge uses a normal merge commit with a validated head SHA.
- No branch-protection exception, admin bypass, check spoofing, direct push, squash merge, or rebase merge occurs.
