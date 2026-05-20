Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0322 Closeout Restore NA-0323 Testplan

## Objective

Close NA-0322 after PR #906 merged the bounded qshield embedded relay/demo
timing and traffic-shape measurement harness, then restore exactly one READY
successor:

`NA-0323 -- Metadata Runtime Timing and Traffic-Shape Instrumentation / Mitigation Design Plan`

This closeout must not implement NA-0323.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0323.
- NA-0322 is DONE.
- D-0626 exists once.
- D-0627 exists once.
- D-0628 is absent.
- Measurement remains distinct from mitigation.
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
- `tests/NA-0322_closeout_restore_na0323_testplan.md`

## Forbidden Scope

- qshield runtime implementation.
- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- runtime timing, jitter, batching, cover traffic, send scheduling, receive
  scheduling, transport padding, instrumentation implementation, mitigation
  implementation, or service deployment behavior.
- `Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, `formal/**`,
  `inputs/**`, `tools/**`, `apps/**`, `qsl/**`, `qsp/**`, `qsc/**`,
  `qsc-desktop/**`, `docs/public/**`, `README.md`, `START_HERE.md`, website or
  external website paths, branch-protection configuration, and public-safety
  configuration.

## Closeout Requirements

1. Record PR #906 head and merge evidence for NA-0322.
2. Record post-merge main public-safety success for PR #906.
3. Mark NA-0322 DONE.
4. Add D-0627 for NA-0322 closeout and NA-0323 restoration.
5. Add NA-0323 as the sole READY item with the exact selected title:
   `Metadata Runtime Timing and Traffic-Shape Instrumentation / Mitigation Design Plan`.
6. State that NA-0323 is not implemented by closeout.
7. Preserve qshield embedded relay/demo and qsl-server/qsl-attachments
   production boundaries.
8. Preserve claim boundaries for timing metadata, traffic shape, measurement
   versus mitigation, anonymity, metadata-free behavior, untraceability,
   production readiness, public internet readiness, and external review
   completion.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- direct decision-count proof for D-0626, D-0627, and D-0628
- `git diff --check origin/main...HEAD`
- exact allowed-path `scope-guard`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- local goal-lint via synthetic PR event

## CI Expectations

Required checks must pass before merge. `public-safety` must remain required and
green before merge and after merge. Docs-only/full-suite cost-control skips are
acceptable only when reported as skipped by CI and public-safety remains green.

## Successor Handoff

The restored NA-0323 lane should design instrumentation and mitigation options
from NA-0321 threat-model evidence and NA-0322 measurement evidence without
claiming runtime mitigation has already been implemented. Service timing for
qsl-server and qsl-attachments remains future-gated unless exact future scope
authorizes cross-repo work.
