Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0322A Insert Refimpl Formatting Remediation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the governance-only insertion of NA-0322A as a prerequisite formatter
remediation lane before NA-0322 resumes.

The insertion must not format or otherwise change the refimpl test file. It
only records that clean workspace formatting validation exposed one
pre-existing rustfmt drift outside NA-0322 scope.

## Protected Invariants

- Exactly one READY item exists after insertion: NA-0322A.
- NA-0322 is not READY until NA-0322A closeout restores it.
- D-0621 exists once.
- D-0622 exists once.
- D-0623 exists once.
- D-0624 is absent.
- The formatter blocker remains visible and is not bypassed.
- No runtime, protocol, wire, crypto, auth, state-machine, qshield, qsc/qsp,
  qsl-server, or qsl-attachments behavior is changed.
- No dependency, Cargo, workflow, branch-protection, public-safety, website,
  README, START_HERE, or broad formatting change is introduced.
- No anonymity, metadata-free, untraceable, timing-hidden, traffic-shape-hidden,
  production-readiness, public-internet-readiness, or external-review-complete
  claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0322A_insert_refimpl_formatting_remediation_testplan.md`

## Forbidden Scope

- `tools/refimpl/**` formatting or implementation changes in this insertion
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

## Required Proof

Record:

- clean `origin/main` workspace `cargo fmt --check` failure limited to
  `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`;
- `cargo fmt --package qshield-cli -- --check` success;
- exact queue transition from READY NA-0322 to READY NA-0322A with NA-0322
  blocked;
- D-0623 insertion and D-0624 absence;
- unchanged dependency/public-safety posture;
- scope guard showing only allowed insertion files changed.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- direct decision-count proof for D-0621, D-0622, D-0623, and D-0624
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
green before merge and after merge.

## Successor Handoff

After this insertion merges and post-merge public-safety is green, the next lane
must format exactly:

`tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`

The remediation lane must not broaden to unrelated rustfmt cleanup or runtime
changes.
