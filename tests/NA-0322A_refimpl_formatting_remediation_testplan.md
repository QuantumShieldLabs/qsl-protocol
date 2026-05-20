Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0322A Refimpl Formatting Remediation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0322A formats exactly the single refimpl vector-oracle test
file that blocked NA-0322 workspace formatting validation.

## Protected Invariants

- Only `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
  may be formatted.
- No other refimpl file may change.
- No qshield, qsc/qsp, qsl-server, qsl-attachments, protocol-core, crypto,
  state-machine, key-schedule, Cargo, dependency, workflow, public-safety,
  website, README, or START_HERE change is introduced.
- Workspace `cargo fmt --check` passes after remediation.
- The NA-0310 oracle test remains green.
- NA-0322 remains blocked until closeout restores it.
- No runtime timing mitigation, jitter, batching, cover traffic, send
  scheduling, receive scheduling, transport padding, or service timing behavior
  is implemented.
- No anonymity, metadata-free, untraceable, timing-hidden, traffic-shape-hidden,
  production-readiness, public-internet-readiness, or external-review-complete
  claim is introduced.

## Allowed Scope

- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `docs/governance/evidence/NA-0322A_refimpl_formatting_remediation.md`
- `tests/NA-0322A_refimpl_formatting_remediation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Any other `tools/refimpl/**` file.
- qshield runtime source or tests.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- qsl-server or qsl-attachments implementation.
- `Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, `formal/**`,
  `inputs/**`, `apps/**`, `qsl/**`, `qsp/**`, `qsc/**`, `qsc-desktop/**`,
  `docs/public/**`, `README.md`, `START_HERE.md`, website or external website
  paths, branch-protection configuration, and public-safety configuration.

## Required Local Checks

- `git diff --name-only`
- exact allowed-path `scope-guard`
- `cargo fmt --check`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1`
- full refimpl tests if feasible
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- direct decision-count proof for D-0623, D-0624, and D-0625
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

After this PR merges and post-merge public-safety is green, the next closeout
lane should mark NA-0322A DONE and restore NA-0322 as the sole READY item. The
closeout must not implement NA-0322.
