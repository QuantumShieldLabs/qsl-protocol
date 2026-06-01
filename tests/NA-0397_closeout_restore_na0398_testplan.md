# NA-0397 Closeout and NA-0398 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0397 is closed only after the code/crypto research-watch and
audit-follow-up plan is merged, post-merge public-safety is green, and exactly
one READY successor is restored:
`NA-0398 -- QSL Metadata Privacy / Secure Messaging Claim Boundary Plan`.

## Protected Invariants

- READY_COUNT is exactly 1 after closeout.
- READY item is NA-0398.
- NA-0397 is DONE.
- D-0776 exists once.
- D-0777 exists once.
- D-0778 is absent.
- NA-0398 is not implemented by this closeout.
- No runtime, protocol, crypto, dependency, Cargo, workflow, public-doc,
  website, backup-script, qsl-server, qsl-attachments, qshield runtime,
  response archive, local tool, or secret-bearing path is changed.
- No automatic READY promotion occurs outside the selected successor.
- No public, readiness, privacy, external-review, vulnerability-free,
  bug-free, or perfect-crypto claim is expanded.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0397_closeout_restore_na0398_testplan.md`

## Forbidden Scope

Forbidden paths include runtime/service/protocol/crypto/dependency files,
`Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, `qsc/**`, `qsp/**`,
`qsl/**`, `apps/**`, `formal/**`, qsl-server, qsl-attachments, website,
README, START_HERE, docs/public, backup scripts/timers/fstab/services, response
archives, request/directive/journal history roots, `/srv/qbuild/tools/**`, and
local/off-host backup or restore configuration.

## Required Closeout Evidence

- Packet W PR #1057 merged.
- Packet W head SHA is recorded as `40f11d5cb34`.
- Packet W merge SHA is recorded as `38b4da16362e`.
- Packet W post-merge public-safety on `38b4da16362e` is success.
- D-0776 records the NA-0397 code/crypto research-watch planning decision.
- D-0777 records closeout and selected successor restoration.
- `NEXT_ACTIONS.md` marks NA-0397 DONE and NA-0398 READY.
- `TRACEABILITY.md` links D-0777, D-0776, PR #1057, closeout testplan, selected
  successor, backup-impact classification, and sibling repo boundaries.

## Successor Handoff

The restored successor is:
`NA-0398 -- QSL Metadata Privacy / Secure Messaging Claim Boundary Plan`.

Future NA-0398 may create a metadata privacy and secure messaging
claim-boundary plan only if live NA-0398 scope authorizes it. This closeout
must not create metadata/privacy findings, update dependencies, change runtime
code, change workflows, or make public claims.

## Backup Impact

The closeout changes only tracked qsl-protocol governance and testplan files.
No backup-plan update is required. Any future durable metadata/privacy report,
code/crypto audit report, recurring external-watch report store, public
technical paper evidence store, or off-host backup change requires separate
backup-impact review.

## Required Validation

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard ...`
- `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint or PR body preflight with a standalone
  `Goals: G1, G2, G3, G4, G5` line.

## CI Expectations

The closeout PR must pass required checks normally, including public-safety and
goal-lint. CodeQL neutral/skipped may be accepted only under the existing
docs-only policy. No admin bypass, squash, rebase, direct push, force-push,
amend-after-PR, or branch-deletion command is allowed.
