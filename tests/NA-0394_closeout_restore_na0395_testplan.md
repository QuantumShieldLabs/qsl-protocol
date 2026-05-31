Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0394 Closeout and NA-0395 Restoration Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-213

## Objective

Validate that NA-0394 closeout marks the PQC standards alignment mapping lane
DONE and restores exactly one READY successor:

`NA-0395 -- QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan`

The closeout must not implement NA-0395.

## Protected Invariants

- READY_COUNT is exactly one.
- READY is the selected NA-0395 successor.
- NA-0394 is DONE.
- D-0770 exists once.
- D-0771 exists once.
- D-0772 is absent.
- No runtime, service, protocol, crypto, dependency, workflow, public docs,
  website, backup, response archive, qsl-server, qsl-attachments, qshield
  runtime, qstart/qresume, or secret-bearing path is changed.
- No compliance, certification, production-readiness, public-internet readiness,
  metadata-free, anonymity, untraceable, bug-free, perfect-crypto, or
  external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0394_closeout_restore_na0395_testplan.md`

## Forbidden Scope

Forbidden changes include README, START_HERE, public docs, `.github/**`, Cargo
files, qsp, qsc, qsl, qsl-client, apps, tools, inputs, formal, scripts,
qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo
or service implementation paths, branch-protection/public-safety configuration,
backup scripts/timers/fstab/local system paths, branch deletion, local Codex
history roots, and qstart/qresume tooling.

## Packet S Prerequisites

Verify before closeout:

- Packet S PR #1051 merged.
- Packet S merge commit is recorded.
- Post-merge public-safety is green.
- READY_COUNT is 1 and READY is NA-0394.
- D-0770 exists.
- D-0771 is absent.
- Selected successor is exact.
- No durable PQC report exists outside authorized governance evidence.

## NEXT_ACTIONS Requirements

Verify `NEXT_ACTIONS.md`:

- Marks NA-0394 DONE.
- Records PR #1051 head and merge evidence.
- Records post-merge public-safety evidence.
- Records D-0770 and D-0771.
- Restores selected NA-0395 as READY.
- States that NA-0395 is not implemented by the closeout.
- Preserves no runtime/service/protocol/crypto/dependency/workflow, no secret,
  no backup mutation, no target setup, no public/readiness/privacy overclaim,
  no compliance/certification, and no unsupported current-technology claim
  boundaries.

## Decision Requirements

Verify D-0771:

- Title is `NA-0394 closeout and NA-0395 restoration`.
- Status is Accepted.
- Goals are G1, G2, G3, G4, G5.
- States NA-0394 delivered PQC standards alignment / migration evidence
  mapping.
- States selected NA-0395 is based on NA-0394 evidence.
- States no NA-0395 implementation is authorized by closeout.
- Protects runtime, security, public-claim, dependency, workflow, backup, and
  sibling-repo boundaries.

## Traceability Requirements

Verify `TRACEABILITY.md` links:

- D-0771.
- D-0770.
- Packet S evidence.
- Packet T closeout testplan.
- selected NA-0395 successor.
- backup-impact classification.
- qsl-server and qsl-attachments read-only boundaries.

## Validation Requirements

Run:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- D-0771 / D-0772 count check.
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
RFC/draft reports or public technical paper evidence stores require separate
backup-impact review.
