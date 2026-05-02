Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-02

# NA-0241 Demo Downgrade No-Mutation Hardening Testplan

Goals: G1, G3, G4, G5

## Objective

Prove `NA-0241` strengthens executable downgrade/transcript mismatch reject coverage, at least one no-state-mutation-on-reject path, and qshield demo negative acceptance without changing protocol wire semantics or service/runtime boundaries.

## Protected Invariant

- No silent downgrade or fallback.
- Suite-2 transcript and capability commitments fail closed.
- Rejected downgrade/transcript/capability input does not mutate state where mutable state exists.
- Demo negative cases are exercised only on current truthful demo surfaces.
- Demo remains non-production and must not imply production readiness.
- `public-safety` remains required and green.
- qsl-server remains transport-only.
- qsl-attachments remains opaque ciphertext-only.
- qsc-desktop and website remain untouched.

## Scope Guard

Allowed changed paths:

- `tools/refimpl/quantumshield_refimpl/src/**` only if directly required by bounded downgrade/no-mutation invariant enforcement
- `tools/refimpl/quantumshield_refimpl/tests/**`
- `inputs/suite2/vectors/**`
- `apps/qshield-cli/src/**`
- `scripts/ci/demo_cli_smoke.sh`
- `scripts/ci/metadata_conformance_smoke.sh` only if strictly required
- `docs/governance/evidence/NA-0241_demo_downgrade_no_mutation_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0241_demo_downgrade_no_mutation_hardening_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` only if consistent with evidence pattern

Forbidden path proof must confirm no `.github/**`, `scripts/ci/public_safety_gate.py`, `Cargo.toml`, `Cargo.lock`, `qsc/**`, `qsl/**`, `qsl-client/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, branch-protection settings, public-safety/check configuration, or unrelated runtime/protocol/crypto/demo/service paths changed.

`NEXT_ACTIONS.md` must remain unchanged in the `NA-0241` implementation PR; `NA-0241` remains READY pending later closeout.

## Downgrade/Transcript Mismatch Reject Proof

Required proof:

- `S2-TRANSCRIPT-REJECT-PQ-BIND-MISMATCH-NA0241` is present in `inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json`.
- The Suite-2 transcript vector runner rejects the case with `REJECT_S2_AD_MISMATCH`.
- Existing downgrade vectors remain green.

## No-State-Mutation-On-Reject Proof

Required proof:

- `capability_commitment_flags_mismatch_rejects_without_mutation` passes.
- The test snapshots receive state before rejection.
- Repeated unsupported capability-flag rejects return the same reason and preserve byte-identical state snapshots.
- Stateless transcript/PQ-binding mismatch vector behavior is documented as stateless and not overclaimed as durable state proof.

## Demo Negative Acceptance Proof

Required proof:

- `scripts/ci/demo_cli_smoke.sh` still proves positive init/register/establish/send/recv.
- The same smoke rejects invalid relay ID registration.
- The same smoke rejects a replayed establish record.
- No KT-negative scenario is claimed unless the demo surface carries enough KT evidence.

## Unwrap/Expect Audit Or Cleanup Proof

Required proof:

- `rg -n "unwrap\\(|expect\\(|panic!" apps/qshield-cli/src` reports only test-only relay mutex poison assertions after cleanup.
- User-facing relay request handlers avoid guarded unwraps and continue returning deterministic JSON errors.

## Relay-Auth Transient/Flaky Observation

If Phase 2 public-safety recovery was used, record:

- Original failed path: `qsl/qsl-client/qsc/tests/relay_auth_header.rs`.
- Original failed test: `relay_auth_with_token_send_receive_ok_and_no_secret_leak`.
- macOS full-suite rerun result.
- `public-safety` rerun result.
- No patch was made to the relay-auth test path.

## Local Validation Commands

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `cargo fmt --check`
- `cargo audit --deny warnings`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --locked --test na_0241_demo_downgrade_no_mutation -- --test-threads=1`
- `cargo build -p refimpl_actor --locked`
- `cargo build -p qshield-cli --locked`
- `python3 scripts/ci/validate_suite2_vectors.py`
- `scripts/ci/run_suite2_transcript_vectors.py --actor "${CARGO_TARGET_DIR:-target}/debug/refimpl_actor" --actor-name suite2-transcript-na0241 --out artifacts/suite2/transcript_vector_report_na0241.json`
- `scripts/ci/run_suite2_transcript_vectors.py --actor tools/actors/interop_actor_py/interop_actor.py --actor-name suite2-py-transcript-na0241 --out artifacts/suite2/transcript_vector_report_py_na0241.json`
- `scripts/ci/run_suite2_downgrade_vectors.py --actor "${CARGO_TARGET_DIR:-target}/debug/refimpl_actor" --actor-name suite2-downgrade-na0241 --out artifacts/suite2/downgrade_vector_report_na0241.json`
- `scripts/ci/demo_cli_smoke.sh`
- deterministic queue parser
- deterministic decision parser
- repo-local goal-lint via synthetic PR event
- markdown inventory commands from `AGENTS.md`
- manual markdown link-integrity runbook from `AGENTS.md`
- leak-safe added-line scan

## Required CI Context Expectations

The implementation PR must satisfy the protected context set:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

`CodeQL` may be accepted as neutral only if GitHub branch protection accepts it. `public-safety` must be success. No branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge is allowed.

## References

- `DECISIONS.md` (D-0447)
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0241_demo_downgrade_no_mutation_audit.md`
- `inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0241_demo_downgrade_no_mutation.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `scripts/ci/demo_cli_smoke.sh`
