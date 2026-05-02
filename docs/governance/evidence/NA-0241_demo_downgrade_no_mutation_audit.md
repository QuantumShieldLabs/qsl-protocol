Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-02

# NA-0241 Demo Downgrade No-Mutation Audit

Goals: G1, G3, G4, G5

## Objective

Record the executable downgrade/transcript reject, no-state-mutation, demo negative acceptance, and guarded unwrap/expect coverage added for `NA-0241`.

## Main Public-Safety Recovery

Latest `main` initially had `public-safety` red on short SHA `277c777285ca`. The underlying failed check was `macos-qsc-full-serial`, with the prior relay-auth test failure in `qsl/qsl-client/qsc/tests/relay_auth_header.rs`. The failure path was outside `NA-0241` scope and was not patched.

Bounded recovery used one job-specific macOS full-suite rerun and one `public-safety` rerun:

- `macos-qsc-full-serial` attempt 2 succeeded: <https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/25211287278/job/74016000076>
- `public-safety` attempt 2 succeeded: <https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/25211287276/job/74019668165>

This is recorded as a transient/flaky macOS full-suite observation, not as an in-scope code defect.

## Findings

- Existing Suite-2 transcript vector coverage already covered AD-header and flags mismatch rejects.
- Existing downgrade vector coverage already covered silent fallback, suite mismatch, AD mismatch, and peer-unsupported rejects.
- Existing demo smoke covered only a positive register/establish/send/recv flow.
- User-facing relay request handlers contained several guarded `unwrap()` calls after explicit validation. The cleanup was bounded to replacing those unwraps with `let else` / tuple destructuring while preserving response behavior.
- Remaining `unwrap()` / `panic!` hits under `apps/qshield-cli/src/**` are test-only relay mutex poison assertions.

## Executable Coverage Added

- `inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json`
  - Adds `S2-TRANSCRIPT-REJECT-PQ-BIND-MISMATCH-NA0241`.
  - The vector changes the PQ binding input while leaving the supplied transcript AD at the canonical no-prefix value, requiring deterministic `REJECT_S2_AD_MISMATCH`.
- `tools/refimpl/quantumshield_refimpl/tests/na_0241_demo_downgrade_no_mutation.rs`
  - Adds `capability_commitment_flags_mismatch_rejects_without_mutation`.
  - The test invokes the non-boundary receive path with unsupported capability flags, requires deterministic `REJECT_S2_LOCAL_UNSUPPORTED`, and compares pre/post receive-state snapshots.
- `scripts/ci/demo_cli_smoke.sh`
  - Adds invalid relay ID registration rejection.
  - Adds replayed establish-record rejection with HTTP `409` and `establish replay` reason.
  - Resolves the debug binary path through `${CARGO_TARGET_DIR:-$ROOT_DIR/target}` so qbuild and CI target layouts both run the same smoke.
- `apps/qshield-cli/src/commands/relay.rs`
  - Removes guarded user-facing relay handler unwraps without changing relay response semantics.

## No-State-Mutation Proof

The new refimpl test snapshots `Suite2RecvState` before the unsupported capability-flag reject and compares it against both repeated reject outputs. The reject returns before header/body AEAD processing or receive-state advancement, so no durable or in-memory state is mutated on the covered reject path.

The transcript/PQ-binding mismatch vector itself is stateless: the actor rejects before any session state exists. That is documented here rather than overclaimed as durable no-mutation proof.

## Demo Negative Cases

`demo_cli_smoke.sh` now proves two negative demo acceptance cases:

- Invalid relay ID format rejects deterministically during `qshield register`.
- Replayed establish record rejects deterministically through the demo relay replay cache.

Both cases use the existing local demo relay surface. No KT-negative claim is made because the current qshield demo smoke surface does not carry enough KT evidence to exercise that honestly.

## Commands

Targeted commands executed during implementation:

- `cargo fmt`
- `cargo test -p quantumshield_refimpl --locked --test na_0241_demo_downgrade_no_mutation -- --test-threads=1`
- `cargo build -p refimpl_actor --locked`
- `cargo build -p qshield-cli --locked`
- `scripts/ci/run_suite2_transcript_vectors.py --actor "${CARGO_TARGET_DIR:-target}/debug/refimpl_actor" --actor-name suite2-transcript-na0241 --out artifacts/suite2/transcript_vector_report_na0241.json`
- `scripts/ci/run_suite2_transcript_vectors.py --actor tools/actors/interop_actor_py/interop_actor.py --actor-name suite2-py-transcript-na0241 --out artifacts/suite2/transcript_vector_report_py_na0241.json`
- `scripts/ci/demo_cli_smoke.sh`

Baseline main-health commands executed before implementation:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`

Expected final validation:

- `git diff --check`
- `cargo fmt --check`
- `cargo audit --deny warnings`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --locked --test na_0241_demo_downgrade_no_mutation -- --test-threads=1`
- `scripts/ci/run_suite2_transcript_vectors.py --actor "${CARGO_TARGET_DIR:-target}/debug/refimpl_actor" --actor-name suite2-transcript-na0241 --out artifacts/suite2/transcript_vector_report_na0241.json`
- `scripts/ci/run_suite2_transcript_vectors.py --actor tools/actors/interop_actor_py/interop_actor.py --actor-name suite2-py-transcript-na0241 --out artifacts/suite2/transcript_vector_report_py_na0241.json`
- `scripts/ci/demo_cli_smoke.sh`

## Gaps And Recommendations

- Add a later stateful receive-wire test that tampers the envelope suite fields and proves parse reject behavior against a durable actor/session snapshot.
- Add a later demo KT-negative smoke only after the demo surface truthfully carries KT evidence.
- Keep qsl-server, qsl-attachments, qsc-desktop, website, Cargo metadata, and public-safety configuration out of this lane.
