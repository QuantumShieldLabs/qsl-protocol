Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0324 Closeout Restore NA-0325 Testplan

## Objective

Close NA-0324 after PR #910 merged the bounded qshield embedded relay/demo
timing and traffic-shape instrumentation harness, then restore exactly one
READY successor:

`NA-0325 -- Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix`

This closeout must not implement NA-0325.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0325.
- NA-0324 is DONE.
- D-0630 exists once.
- D-0631 exists once.
- D-0632 is absent.
- Instrumentation remains measurement evidence, not mitigation.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production timing.
- qsl-server and qsl-attachments production timing remain unproven and
  future-gated.
- Timing metadata and traffic shape are not claimed hidden.
- No metadata-free, anonymity, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0324_closeout_restore_na0325_testplan.md`

## Forbidden Scope

- NA-0325 implementation.
- Runtime timing mitigation.
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

## PR #910 Proof Requirements

1. PR #910 is merged.
2. PR #910 title is `NA-0324: add timing traffic instrumentation harness`.
3. PR #910 merge commit is `f518844b61b87438fb84719d6c77c77e5a5d493c`.
4. PR #910 head commit is recorded.
5. PR #910 evidence remains bounded to instrumentation harness proof.

## Public-Safety Proof Requirements

1. `public-safety` completed success for
   `f518844b61b87438fb84719d6c77c77e5a5d493c`.
2. qsc Linux full suite completed success for the same merge commit.
3. macOS qsc full serial completed success for the same merge commit.
4. Public-safety remains a required branch-protection check.
5. No closeout merge may proceed if required CI is failed, cancelled, timed out,
   action-required, missing, or ambiguous.

## Queue/Decision Proof Requirements

1. Start state has READY_COUNT 1 and READY NA-0324.
2. End state has READY_COUNT 1 and READY NA-0325.
3. NA-0324 is DONE.
4. D-0630 exists once.
5. D-0631 exists once.
6. D-0632 is absent.
7. Duplicate decision count is zero.

## Claim-Boundary Requirements

- Instrumentation is measurement evidence only.
- The closeout does not claim timing metadata is hidden.
- The closeout does not claim traffic shape is hidden.
- The closeout does not claim metadata-free, anonymity, untraceable,
  production-readiness, public-internet-readiness, or external-review-complete
  status.
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
- direct decision-count proof for D-0630, D-0631, and D-0632
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

The restored NA-0325 lane should produce a metadata-runtime timing and
traffic-shape mitigation option matrix grounded in NA-0324 instrumentation
evidence. It must keep measurement distinct from mitigation and must not
implement runtime timing/jitter/batching/cover-traffic mitigation unless a later
directive separately authorizes implementation.
