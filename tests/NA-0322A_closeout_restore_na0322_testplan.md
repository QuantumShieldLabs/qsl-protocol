Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0322A Closeout Restore NA-0322 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0322A after PR #903 inserted the formatter gate and PR #904
remediated exactly the single refimpl formatting blocker, then restore exactly
one READY successor:

`NA-0322 -- Metadata Runtime Timing and Traffic-Shape Measurement Harness`

This closeout must not implement NA-0322.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0322.
- NA-0322A is DONE.
- D-0623 exists once.
- D-0624 exists once.
- D-0625 exists once.
- D-0626 and D-0627 are absent.
- PR #903 and PR #904 evidence remains explicit.
- PR #904 formatting scope remains exactly
  `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`.
- Preserved D132 NA-0322 local work remains available in the bundle and stash.
- No runtime, protocol, wire, crypto, auth, state-machine, qshield, qsc/qsp,
  qsl-server, or qsl-attachments behavior is changed.
- No dependency, Cargo, workflow, branch-protection, public-safety, website,
  README, START_HERE, or broad formatting change is introduced.
- Measurement remains distinct from mitigation.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior.
- No anonymity, metadata-free, untraceable, timing-hidden, traffic-shape-hidden,
  production-readiness, public-internet-readiness, or external-review-complete
  claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0322A_closeout_restore_na0322_testplan.md`

## Forbidden Scope

- `tools/refimpl/**` formatting or implementation changes in this closeout
  lane.
- qshield runtime implementation.
- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- Runtime timing, jitter, batching, cover traffic, send scheduling, receive
  scheduling, transport padding, or service deployment behavior.
- `Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, `formal/**`,
  `inputs/**`, `apps/**`, `qsl/**`, `qsp/**`, `qsc/**`, `qsc-desktop/**`,
  `docs/public/**`, `README.md`, `START_HERE.md`, website or external website
  paths, branch-protection configuration, and public-safety configuration.

## Closeout Requirements

1. Record PR #903 merge evidence for the NA-0322A insertion.
2. Record PR #904 merge evidence for the exact single-file formatting
   remediation.
3. Record post-merge main public-safety success for `82061a911bd6`.
4. Mark NA-0322A DONE.
5. Add D-0625 for NA-0322A closeout and NA-0322 restoration.
6. Restore NA-0322 as the sole READY item with the exact selected title:
   `Metadata Runtime Timing and Traffic-Shape Measurement Harness`.
7. State that preserved D132 NA-0322 local work may resume after closeout.
8. State that NA-0322 is not implemented by closeout.
9. Preserve qshield embedded relay/demo and qsl-server/qsl-attachments
   production boundaries.
10. Preserve claim boundaries for timing metadata, traffic shape, anonymity,
    metadata-free behavior, untraceability, production readiness, public
    internet readiness, and external review completion.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- direct decision-count proof for D-0623, D-0624, D-0625, D-0626, and D-0627
- `git diff --check origin/main...HEAD`
- exact allowed-path `scope-guard`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan
- `cargo fmt --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- local goal-lint via synthetic PR event

## CI Expectations

Required checks must pass before merge. `public-safety` must remain required and
green before merge and after merge.

## Successor Handoff

The restored NA-0322 lane should resume the preserved D132 measurement harness
work, remap implementation decision evidence to D-0626, and keep NA-0322
closeout evidence reserved for D-0627. The measurement harness must remain
bounded to qshield embedded relay/demo evidence and must not claim mitigation,
hidden timing metadata, production timing proof, anonymity, metadata-free
behavior, or untraceability.
