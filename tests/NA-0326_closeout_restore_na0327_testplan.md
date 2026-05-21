Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0326 Closeout Restore NA-0327 Testplan

## Objective

Close NA-0326 after PR #914 merged the metadata runtime qshield embedded
relay/demo retry-cadence normalization authorization plan, then restore exactly
one READY successor:

`NA-0327 -- Metadata Runtime qshield Demo Retry Cadence Normalization Implementation Harness`

This closeout must not implement NA-0327.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0327.
- NA-0326 is DONE.
- D-0634 exists once.
- D-0635 exists once.
- D-0636 is absent.
- Measurement remains distinct from mitigation.
- The NA-0326 authorization plan is not presented as implemented mitigation.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production timing.
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.
- Timing metadata and traffic shape are not claimed hidden.
- No prohibited metadata-free, anonymity, untraceable, production-readiness, public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0326_closeout_restore_na0327_testplan.md`

## Forbidden Scope

- NA-0327 implementation.
- Runtime timing mitigation.
- Retry-cadence normalization implementation.
- Jitter, batching, cover traffic, queue scheduling, send scheduling, receive
  scheduling, transport padding, or service deployment behavior.
- qshield runtime implementation.
- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- `Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, `formal/**`,
  `inputs/**`, `tools/**`, `apps/**`, `qsl/**`, `qsp/**`, `qsc/**`,
  `qsc-desktop/**`, `docs/public/**`, `README.md`, `START_HERE.md`, website or
  external website paths, branch-protection configuration, and public-safety
  configuration.

## PR #914 Proof Requirements

1. PR #914 is merged.
2. PR #914 title is `NA-0326: add qshield demo retry cadence authorization`.
3. PR #914 merge commit is recorded.
4. PR #914 head commit is recorded.
5. PR #914 evidence remains bounded to authorization/design-only proof.

## Public-Safety Proof Requirements

1. `public-safety` completed success for the PR #914 merge commit.
2. Public-safety remains a required branch-protection check.
3. Docs-only cost-control skips are allowed only when public-safety is green.
4. No closeout merge may proceed if required CI is failed, cancelled, timed out,
   action-required, missing, or ambiguous.

## Queue/Decision Proof Requirements

1. Start state has READY_COUNT 1 and READY NA-0326.
2. End state has READY_COUNT 1 and READY NA-0327.
3. NA-0326 is DONE.
4. D-0634 exists once.
5. D-0635 exists once.
6. D-0636 is absent.
7. Duplicate decision count is zero.

## Claim-Boundary Requirements

- Measurement remains distinct from mitigation.
- The authorization plan is not runtime mitigation.
- The closeout does not claim timing metadata is hidden.
- The closeout does not claim traffic shape is hidden.
- The closeout does not claim prohibited metadata-free, anonymity, untraceable, production-readiness, public-internet-readiness, or external-review-complete status.
- qshield embedded relay/demo evidence is not presented as qsl-server or
  qsl-attachments production timing evidence.

## Backup-Impact Requirements

- No backup-plan update is required because the closeout touches only queue,
  decision, traceability, journal, and testplan documentation.
- The preserved D132 bundle and any D132 stash state are not deleted or cleaned
  up by this closeout.
- Any missing preservation item must be reported as operational evidence rather
  than repaired through unauthorized cleanup.

## Required Local Checks

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- direct decision-count proof for D-0634, D-0635, and D-0636
- exact allowed-path `scope-guard`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan
- classifier proof for the changed paths
- local goal-lint via synthetic PR event
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

## CI Expectations

Required PR checks must pass normally before merge. `public-safety` must remain
required and green before merge and after merge. Docs-only or full-suite
cost-control skips are acceptable only when reported by CI and public-safety
remains green.

## Successor Handoff

The restored NA-0327 lane should execute the exact implementation/design/blocker
successor selected by NA-0326 for qshield embedded relay/demo retry-cadence
normalization. It must preserve the measurement-versus-mitigation boundary,
keep qshield demo evidence distinct from qsl-server/qsl-attachments production
timing, and avoid unsupported prohibited timing-hidden, traffic-shape-hidden, metadata-free, anonymity, untraceable, production-readiness, public-internet-readiness, or external-review-complete claims.
