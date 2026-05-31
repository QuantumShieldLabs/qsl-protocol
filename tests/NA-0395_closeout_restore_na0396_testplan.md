Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0395 Closeout and NA-0396 Restoration Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-215

## Objective

Validate that NA-0395 closeout marks the IETF/CFRG RFC/draft boundary mapping
lane DONE and restores exactly one READY successor:

`NA-0396 -- QSL Dependency / Advisory Watch Trigger Policy Plan`

The closeout must not implement NA-0396.

## Protected Invariants

- READY_COUNT is exactly one.
- READY is the selected NA-0396 successor.
- NA-0395 is DONE.
- D-0772 exists once.
- D-0773 exists once.
- D-0774 is absent.
- No runtime, service, protocol, crypto, dependency, workflow, public docs,
  website, backup, response archive, qsl-server, qsl-attachments, qshield
  runtime, qstart/qresume, or secret-bearing path is changed.
- No draft-as-final, compliance, certification, production-readiness,
  public-internet readiness, metadata-free, anonymity, untraceable, bug-free,
  perfect-crypto, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0395_closeout_restore_na0396_testplan.md`

## Forbidden Scope

Forbidden changes include README, START_HERE, public docs, `.github/**`, Cargo
files, qsp, qsc, qsl, qsl-client, apps, tools, inputs, formal, scripts,
qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo
or service implementation paths, branch-protection/public-safety configuration,
backup scripts/timers/fstab/local system paths, branch deletion, local Codex
history roots, and qstart/qresume tooling.

## Packet U Prerequisites

Verify before closeout:

- Packet U PR #1053 merged.
- Packet U merge commit is recorded.
- Post-merge public-safety is green.
- READY_COUNT is 1 and READY is NA-0395.
- D-0772 exists.
- D-0773 is absent.
- Selected successor is exact.
- No durable RFC/draft report exists outside authorized governance evidence.

## NEXT_ACTIONS Requirements

Verify `NEXT_ACTIONS.md`:

- Marks NA-0395 DONE.
- Records PR #1053 head and merge evidence.
- Records post-merge public-safety evidence.
- Records D-0772 and D-0773.
- Restores selected NA-0396 as READY.
- States that NA-0396 is not implemented by the closeout.
- Preserves no runtime/service/protocol/crypto/dependency/workflow, no secret,
  no backup mutation, no target setup, no public/readiness/privacy overclaim,
  no compliance/certification, and no unsupported current-technology claim
  boundaries.

## Decision Requirements

Verify D-0773:

- Title is `NA-0395 closeout and NA-0396 restoration`.
- Status is Accepted.
- Goals are G1, G2, G3, G4, G5.
- States NA-0395 delivered IETF/CFRG RFC/draft boundary mapping.
- States selected NA-0396 is based on NA-0395 evidence.
- States no NA-0396 implementation is authorized by closeout.
- Protects runtime, security, public-claim, dependency, workflow, backup, and
  sibling-repo boundaries.

## Traceability Requirements

Verify `TRACEABILITY.md` links:

- D-0773.
- D-0772.
- Packet U evidence.
- Packet V closeout testplan.
- selected NA-0396 successor.
- backup-impact classification.
- qsl-server and qsl-attachments read-only boundaries.

## Validation Requirements

Run:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- D-0773 / D-0774 count check.
- scope guard with exact allowed closeout paths.
- link-check.
- leak-scan.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint / PR-body preflight.

## CI Expectations

- public-safety remains required.
- Required PR checks must pass before merge.
- Post-merge public-safety must pass.
- No admin bypass, direct push, squash, rebase, force-push, amend, or
  branch-deletion command is allowed.

## Backup Impact

No backup-plan update is required because closeout changes only tracked
qsl-protocol governance/testplan/traceability/journal paths. Future durable
advisory reports or dependency-policy evidence stores require separate
backup-impact review.
