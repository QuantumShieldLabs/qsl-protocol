Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0300 Core Replay Reject No-Mutation Testplan

## Objective

Validate that NA-0300 adds executable Suite-2 replay/reject/no-mutation proof
without changing protocol, wire, crypto, dependency, service, desktop, website,
or public-safety behavior.

## Protected Invariants

- Exactly one queue item remains READY during the implementation PR: NA-0300.
- No protocol or wire semantic change.
- No crypto state-machine, key schedule, or handshake change.
- No dependency change.
- No qsl-server, qsl-attachments, qsc-desktop, website, README, START_HERE,
  workflow, branch-protection, or public-safety configuration change.
- Rejected adversarial input does not mutate accepted state in the tested
  surface.
- Replay and duplicate adversarial input are deterministic in the tested
  surface.
- Unsupported or downgrade-like input fails closed in the tested surface.
- Panic/backtrace on adversarial input fails the harness.
- Plaintext/sentinel leakage in reject output fails the harness.
- Known readiness gaps remain visible.

## Allowed Scope

- `tools/refimpl/quantumshield_refimpl/tests/na_0300_core_replay_reject_no_mutation.rs`
- `docs/governance/evidence/NA-0300_core_replay_reject_no_mutation_harness.md`
- `tests/NA-0300_core_replay_reject_no_mutation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `scripts/**`
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- protocol/crypto implementation source
- qsl-server implementation paths
- qsl-attachments implementation paths
- qsc-desktop
- website or external website paths
- branch-protection or public-safety configuration

## Replay / Duplicate Requirements

The harness must replay a previously accepted Suite-2 wire message and prove:

- deterministic reject;
- stable reject code;
- no accepted-state mutation across repeated attempts.

Expected marker:

- `NA0300_REPLAY_REJECT_OK`

## Malformed / Invalid Requirements

The harness must feed malformed or invalid input and prove:

- deterministic reject;
- stable reject code;
- no accepted-state mutation;
- no sentinel echo in reject output.

Expected marker:

- `NA0300_MALFORMED_REJECT_OK`

## Unsupported / Downgrade Requirements

The harness must feed unsupported or downgrade-like input and prove:

- fail-closed rejection;
- stable reject code;
- no accepted-state mutation;
- no sentinel echo in reject output.

## No-Mutation Requirements

The harness must compare a stable accepted-state snapshot before and after each
rejected adversarial attempt. Rejected input must leave the snapshot unchanged
on both the first and repeated reject attempts.

Expected marker:

- `NA0300_NO_MUTATION_ON_REJECT_OK`

## No Panic / Leak Requirements

The harness must fail if adversarial input panics. The harness must fail if
reject error text includes the plaintext sentinel.

Expected markers:

- `NA0300_NO_PANIC_OK`
- `NA0300_NO_SECRET_LEAK_OK`
- `NA0300_CORE_REPLAY_REJECT_NO_MUTATION_OK`

## Required Local Checks

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo fmt --check
cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/metadata_conformance_smoke.sh
scripts/ci/metadata_phase2_identifier_padding_harness.sh
scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allow tools/refimpl/quantumshield_refimpl/tests/na_0300_core_replay_reject_no_mutation.rs --allow docs/governance/evidence/NA-0300_core_replay_reject_no_mutation_harness.md --allow tests/NA-0300_core_replay_reject_no_mutation_testplan.md --allow DECISIONS.md --allow TRACEABILITY.md --allow docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh tools/refimpl/quantumshield_refimpl/tests/na_0300_core_replay_reject_no_mutation.rs docs/governance/evidence/NA-0300_core_replay_reject_no_mutation_harness.md tests/NA-0300_core_replay_reject_no_mutation_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

## CI Expectations

The PR must merge only after required protected checks complete normally. The
public-safety check remains required. Because the change includes a Rust test
path, heavy suites may run and must not be bypassed.

## Successor Handoff

NA-0300 remains READY after the implementation PR merges. A separate closeout
directive may mark NA-0300 DONE and restore NA-0301 only after post-merge
public-safety is green.
