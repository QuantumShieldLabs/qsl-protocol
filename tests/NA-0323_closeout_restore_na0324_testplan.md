Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0323 Closeout Restore NA-0324 Testplan

## Objective

Close NA-0323 after PR #908 merged the metadata-runtime timing and
traffic-shape instrumentation/mitigation design plan, then restore exactly one
READY successor:

`NA-0324 -- Metadata Runtime Timing and Traffic-Shape Instrumentation Harness`

This closeout must not implement NA-0324.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0324.
- NA-0323 is DONE.
- D-0628 exists once.
- D-0629 exists once.
- D-0630 is absent.
- Measurement remains distinct from mitigation.
- The NA-0323 design plan is not presented as runtime implementation.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- Timing metadata and traffic shape are not claimed hidden.
- qsl-server and qsl-attachments production timing remain unproven and
  future-gated.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0323_closeout_restore_na0324_testplan.md`

## Forbidden Scope

- NA-0324 implementation.
- Runtime instrumentation implementation.
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

## Closeout Requirements

1. Record PR #908 head and merge evidence for NA-0323.
2. Record post-merge main public-safety success for PR #908.
3. Mark NA-0323 DONE.
4. Add D-0629 for NA-0323 closeout and NA-0324 restoration.
5. Add NA-0324 as the sole READY item with the exact selected title:
   `Metadata Runtime Timing and Traffic-Shape Instrumentation Harness`.
6. State that NA-0324 is not implemented by closeout.
7. Preserve measurement versus mitigation boundaries.
8. Preserve qshield embedded relay/demo and qsl-server/qsl-attachments
   production boundaries.
9. Preserve claim boundaries for timing metadata, traffic shape, anonymity,
   metadata-free behavior, untraceability, production readiness, public
   internet readiness, and external review completion.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- direct decision-count proof for D-0628, D-0629, and D-0630
- `git diff --check origin/main...HEAD`
- exact allowed-path `scope-guard`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan
- classifier proof for the changed paths
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- local goal-lint via synthetic PR event

## CI Expectations

Required checks must pass before merge. `public-safety` must remain required and
green before merge and after merge. Docs-only/full-suite cost-control skips are
acceptable only when reported as skipped by CI and public-safety remains green.

## Successor Handoff

The restored NA-0324 lane should build bounded qshield embedded relay/demo
instrumentation evidence for metadata-runtime timing and traffic shape. It must
keep instrumentation as measurement evidence, not mitigation, unless a later
directive separately authorizes mitigation implementation. qsl-server and
qsl-attachments production timing remain future-gated unless exact future scope
authorizes cross-repo work.
